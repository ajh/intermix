use vterm_sys::{Size};
use ego_tree;
use std::collections::BTreeMap;
use std::iter::Iterator;

use super::*;

/// The root of a grid based box layout
#[derive(Debug, Clone)]
pub struct Layout {
    pub size: Size,
    tree: ego_tree::Tree<Wrap>,
}

impl Layout {
    pub fn new(size: Size) -> Layout {
        let root = WrapBuilder::row().name("root".to_string()).build();

        Layout {
            size: size,
            tree: ego_tree::Tree::new(root),
        }
    }

    pub fn tree(&self) -> &ego_tree::Tree<Wrap> {
        &self.tree
    }

    // I'd like to explore Returning a new type around Tree which calls flush_changes when dropped.
    pub fn tree_mut(&mut self) -> &mut ego_tree::Tree<Wrap> {
        &mut self.tree
    }

    /// recalculate to account for changes
    pub fn flush_changes(&mut self) {
        self.update_root_wrap();
        let root_id = self.tree.root().id();
        self.compute_layout(root_id);
        self.compute_width(root_id);
        self.compute_x_position(root_id);
        self.compute_height(root_id);
        self.compute_y_position(root_id);
    }

    /// Update the root separaetly from the others because it makes the recursive code simpiler,
    /// and it may have new margin etc settings that effect its values.
    fn update_root_wrap(&mut self) {
        let mut root_ref = self.tree.root_mut();
        let mut root_wrap = root_ref.value();

        let grid_width = root_wrap.grid_width();
        root_wrap.set_computed_grid_width(grid_width);
        root_wrap.set_is_new_line(false);
        root_wrap.set_outside_width(Some(self.size.width));
        root_wrap.set_outside_x(Some(0));
        root_wrap.set_outside_height(Some(self.size.height));
        root_wrap.set_outside_y(Some(0));
    }

    /// Assigns:
    ///
    /// * computed_grid_width
    /// * is_new_line
    ///
    fn compute_layout(&mut self, parent_id: ego_tree::NodeId<Wrap>) {
        let parent_grid_width = self.tree.get(parent_id).value().computed_grid_width().unwrap();

        let mut columns_in_line = 0;

        let child_ids: Vec<ego_tree::NodeId<Wrap>> = self.tree
                                                         .get(parent_id)
                                                         .children()
                                                         .map(|c| c.id())
                                                         .collect();
        for child_id in child_ids {
            {
                let mut child_node = self.tree.get_mut(child_id);
                let mut child_wrap = child_node.value();

                let mut grid_width = child_wrap.grid_width().unwrap_or(GRID_COLUMNS_COUNT);
                if grid_width > parent_grid_width {
                    grid_width = parent_grid_width
                };
                child_wrap.set_computed_grid_width(Some(grid_width));

                columns_in_line += grid_width;
                if columns_in_line > parent_grid_width {
                    columns_in_line = grid_width;
                    child_wrap.set_is_new_line(true);
                }
            }

            self.compute_layout(child_id);
        }
    }

    /// Here's the algo:
    ///
    /// 1. iterate through children by lines.
    ///
    /// 2. For each lines, figure out:
    /// * how many cols are missing due to rounding errors
    /// * which nodes are the most effected
    ///
    /// 3. Assign widths to child nodes, adding back the missing columns to the most effect nodes.
    ///
    /// Assigns:
    ///
    /// * set_outside_width
    ///
    fn compute_width(&mut self, parent_id: ego_tree::NodeId<Wrap>) {
        let lines = self.tree.get(parent_id).lines();
        let parent_width = self.tree.get(parent_id).value().computed_width().unwrap();

        for mut line in lines {
            let mut line_width = 0;
            let mut line_grid_columns_count = 0;

            // calculate provisional widths
            for child_id in line.iter() {
                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_wrap = child_ref.value();
                let percent = child_wrap.grid_width().unwrap() as f32 / GRID_COLUMNS_COUNT as f32;
                let width = (parent_width as f32 * percent).floor() as usize;

                child_wrap.set_outside_width(Some(width));

                line_width += width;
                line_grid_columns_count += child_wrap.grid_width().unwrap();
            }

            // figure how many columns are unused due to rounding errors
            let mut unused_cols = {
                let percent = line_grid_columns_count as f32 / GRID_COLUMNS_COUNT as f32;

                let mut expected_width = (parent_width as f32 * percent).round() as usize;
                if expected_width > parent_width {
                    expected_width = parent_width
                }

                if expected_width > line_width {
                    expected_width - line_width
                } else {
                    0
                }
            };

            // add them back in fairly
            line.sort_by(|a, b| {
                let a_ref = self.tree.get(*a);
                let a_wrap = a_ref.value();
                let b_ref = self.tree.get(*b);
                let b_wrap = b_ref.value();
                a_wrap.outside_width().unwrap().cmp(&b_wrap.outside_width().unwrap())
            });

            for child_id in line.iter() {
                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_wrap = child_ref.value();

                if unused_cols > 0 {
                    unused_cols -= 1;
                } else {
                    break;
                }

                let val = child_wrap.outside_width().unwrap() + 1;
                child_wrap.set_outside_width(Some(val));
            }

            // recurse
            for child_id in line {
                self.compute_width(child_id);
            }
        }
    }

    /// Assigns:
    ///
    /// * set_outside_x
    ///
    fn compute_x_position(&mut self, parent_id: ego_tree::NodeId<Wrap>) {
        let lines = self.tree.get(parent_id).lines();
        let parent_width = self.tree.get(parent_id).value().computed_width().unwrap();
        let parent_x = self.tree.get(parent_id).value().computed_x().unwrap();
        let parent_align = self.tree.get(parent_id).value().align();

        for line in lines {
            let line_width = line.iter()
                                 .map(|id| self.tree.get(*id).value())
                                 .map(|b| b.outside_width().unwrap())
                                 .fold(0, ::std::ops::Add::add);

            let unused_cols = if parent_width > line_width {
                parent_width - line_width
            } else {
                0
            };

            let offset = match parent_align {
                Align::Left => 0,
                Align::Center => (unused_cols as f32 / 2.0).round() as usize,
                Align::Right => unused_cols,
            };

            let mut x = parent_x + offset;

            for id in line {
                {
                    let mut child_ref = self.tree.get_mut(id);
                    let mut child_wrap = child_ref.value();
                    child_wrap.set_outside_x(Some(x));
                    x += child_wrap.outside_width().unwrap();
                }

                self.compute_x_position(id);
            }
        }
    }

    /// Here's the algo:
    ///
    /// 1. iterate through children by lines.
    ///
    /// 2. For each lines, figure out the provisional height of each line based on highest child in the line
    ///
    /// 3. for lines without any children with heights, they evenly divide the remaining parents
    ///    height for their provisional heights. This may be zero!
    ///
    /// 3. if all the lines don't fit the parents height, proportionally remove height on all until
    ///    they do.
    ///
    /// 3. assign heights as the min of line height or desired height.
    ///
    fn compute_height(&mut self, parent_id: ego_tree::NodeId<Wrap>) {
        let lines = self.tree.get(parent_id).lines();
        if lines.len() == 0 {
            return;
        }

        let parent_height = self.tree.get(parent_id).value().computed_height().unwrap();

        // calculate provisional line heights
        #[derive(Debug)]
        struct HeightInfo {
            height: usize,
            is_defined: bool,
        };
        let mut line_heights = BTreeMap::new();

        for (i, line) in lines.iter().enumerate() {
            let height_maybe = line.iter().map(|child_id| {
                self.tree.get(*child_id).value().height()
            }).max_by_key(|height_maybe| {
                match *height_maybe {
                    Some(h) => h,
                    None => 0,
                }
            }).unwrap();

            let info = match height_maybe {
                Some(h) => HeightInfo { height: h, is_defined: true },
                None => HeightInfo { height: 0, is_defined: false },
            };

            line_heights.insert(i, info);
        }

        let sum_of_line_heights = line_heights.values().fold(0, |sum, info| sum + info.height);
        if sum_of_line_heights > parent_height {
            // haircut
            let excess = sum_of_line_heights - parent_height;
            let lines_count = line_heights.iter().filter(|&(_,i)| i.is_defined).count();

            if lines_count > 0 {
                for (_,i) in line_heights.iter_mut().filter(|&(_, ref i)| i.is_defined) {
                    i.height -= excess / lines_count; // integer division
                }

                // add in remainders
                for (_,i) in line_heights.iter_mut().filter(|&(_, ref i)| i.is_defined).take(excess % lines_count) {
                    i.height -= 1;
                }
            }
        }
        else if sum_of_line_heights < parent_height {
            // add height evenly to lines that don't have defined height
            let unused = parent_height - sum_of_line_heights;
            let lines_count = line_heights.iter().filter(|&(_,i)| !i.is_defined).count();

            if lines_count > 0 {
                for (_,i) in line_heights.iter_mut().filter(|&(_, ref i)| !i.is_defined) {
                    i.height += unused / lines_count; // integer division
                }

                // add in remainders
                for (_,i) in line_heights.iter_mut().filter(|&(_, ref i)| !i.is_defined).take(unused % lines_count) {
                    i.height += 1;
                }
            }
        }

        // assign computed height and recurse
        for (line_index, info) in line_heights.iter() {
            for child_id in lines.get(*line_index).unwrap().iter() {
                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_wrap = child_ref.value();

                let computed_height = match child_wrap.height() {
                    Some(h) => *[h, info.height].iter().min().unwrap(),
                    None => info.height,
                };
                child_wrap.set_outside_height(Some(computed_height));
            }
        }

        // recurse
        for line in lines {
            for child_id in line {
                self.compute_height(child_id);
            }
        }
    }

    /// Assigns:
    ///
    /// * set_outside_y
    ///
    fn compute_y_position(&mut self, parent_id: ego_tree::NodeId<Wrap>) {
        let lines = self.tree.get(parent_id).lines();
        let parent_height = self.tree.get(parent_id).value().computed_height().unwrap();
        let parent_y = self.tree.get(parent_id).value().computed_y().unwrap();
        let parent_vertical_align = self.tree.get(parent_id).value().vertical_align();

        let lines_height = lines.iter()
                                .map(|line| {
                                    line.iter()
                                        .map(|id| self.tree.get(*id).value())
                                        .map(|n| n.outside_height().unwrap())
                                        .max()
                                        .unwrap()
                                })
                                .fold(0, ::std::ops::Add::add);
        let unused_rows = if parent_height > lines_height {
            parent_height - lines_height
        } else {
            0
        };

        let offset = match parent_vertical_align {
            VerticalAlign::Top => 0,
            VerticalAlign::Middle => (unused_rows as f32 / 2.0).round() as usize,
            VerticalAlign::Bottom => unused_rows,
        };

        let mut y = parent_y + offset;

        for line in lines.iter() {
            for child_id in line.iter() {
                {
                    let mut child_ref = self.tree.get_mut(*child_id);
                    let mut child_wrap = child_ref.value();
                    child_wrap.set_outside_y(Some(y));
                }

                self.compute_y_position(*child_id);
            }

            y += line.iter()
                     .map(|id| self.tree.get(*id).value())
                     .map(|n| n.outside_height().unwrap())
                     .max()
                     .unwrap();
        }
    }
}
