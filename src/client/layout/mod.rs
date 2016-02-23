use vterm_sys;
use ego_tree;

pub type Size = vterm_sys::ScreenSize;
pub type Pos = vterm_sys::Pos;

pub const GRID_COLUMNS_COUNT: i16 = 12;

/// Represents the Screen for an entire screen.
#[derive(Debug, Clone)]
pub struct Screen {
    pub size: Size,
    tree: ego_tree::Tree<Wrap>,
}

impl Screen {
    pub fn new(size: Size) -> Screen {
        let root = WrapBuilder::row().name("root".to_string()).build();

        Screen {
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

    /// recalculate layout to account for changes to the screen
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
        root_wrap.set_outside_width(Some(self.size.cols as i16));
        root_wrap.set_outside_x(Some(0));
        root_wrap.set_outside_height(Some(self.size.rows as i16));
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
        let parent_grid_width = self.tree.get(parent_id).value().computed_grid_width().unwrap();
        let parent_width = self.tree.get(parent_id).value().computed_width().unwrap();

        for mut line in lines {
            let mut line_width = 0;
            let mut line_grid_columns_count = 0;

            // calculate provisionary widths
            for child_id in line.iter() {
                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_wrap = child_ref.value();
                let percent = *[child_wrap.grid_width().unwrap(), parent_grid_width]
                                   .into_iter()
                                   .min()
                                   .unwrap() as f32 /
                              parent_grid_width as f32;
                let width = (parent_width as f32 * percent).floor() as i16;

                child_wrap.set_outside_width(Some(width));

                line_width += width;
                line_grid_columns_count += child_wrap.grid_width().unwrap();
            }

            // figure how many columns are unused due to rounding errors
            let mut unused_cols = {
                let percent = line_grid_columns_count as f32 / parent_grid_width as f32;

                let mut expected_width = (parent_width as f32 * percent).round() as i16;
                if expected_width > parent_width {
                    expected_width = parent_width
                }

                expected_width - line_width
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
            let unused_cols = parent_width - line_width;
            let offset = match parent_align {
                Align::Left => 0,
                Align::Center => (unused_cols as f32 / 2.0).round() as i16,
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

    /// This one is botton up unlike the others which is top down. It returns the height of its
    /// children.
    ///
    /// Assigns:
    ///
    /// * set_outside_height
    ///
    fn compute_height(&mut self, parent_id: ego_tree::NodeId<Wrap>) -> i16 {
        let lines = self.tree.get(parent_id).lines();

        for line in lines.iter() {
            for child_id in line.iter() {
                let children_height = self.compute_height(*child_id);

                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_wrap = child_ref.value();
                let h = if let Some(i) = child_wrap.height() {
                    i
                } else {
                    children_height
                };
                child_wrap.set_computed_height(Some(h));
            }
        }

        lines.iter()
             .map(|line| {
                 line.iter()
                     .map(|id| self.tree.get(*id).value())
                     .map(|b| b.outside_height().unwrap())
                     .max()
                     .unwrap()
             })
             .fold(0, ::std::ops::Add::add)
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
        let unused_rows = parent_height - lines_height;
        let offset = match parent_vertical_align {
            VerticalAlign::Top => 0,
            VerticalAlign::Middle => (unused_rows as f32 / 2.0).round() as i16,
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

pub trait LineContainer {
    // return vec of child elements organized into horizontal lines
    fn lines(&self) -> Vec<Vec<ego_tree::NodeId<Wrap>>>;
}

impl<'a> LineContainer for ego_tree::NodeRef<'a, Wrap> {
    fn lines(&self) -> Vec<Vec<ego_tree::NodeId<Wrap>>> {
        let mut output = vec![];
        let mut line = vec![];

        for child in self.children() {
            if child.value().is_new_line() && line.len() > 0 {
                output.push(line);
                line = vec![];
            }

            line.push(child.id());
        }

        if line.len() > 0 {
            output.push(line)
        }

        output
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}
impl Default for Align {
    fn default() -> Align {
        Align::Left
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}
impl Default for VerticalAlign {
    fn default() -> VerticalAlign {
        VerticalAlign::Top
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wrap {
    align: Align,
    computed_grid_width: Option<i16>,
    computed_height: Option<i16>,
    computed_width: Option<i16>,
    computed_x: Option<i16>,
    computed_y: Option<i16>,
    grid_width: Option<i16>,
    has_border: bool,
    height: Option<i16>,
    is_new_line: bool,
    margin: i16,
    name: String,
    padding: i16,
    vertical_align: VerticalAlign,
    width: Option<i16>,
}

macro_rules! fn_option_accessor {
    // Can this be less redundent?
    ($field_name:ident, $setter_name:ident, $type_name:ident) => {
        pub fn $field_name(&self) -> Option<$type_name> {
            self.$field_name
        }

        pub fn $setter_name(&mut self, val: Option<$type_name>) {
            self.$field_name = val
        }
    }
}

macro_rules! fn_accessor {
    // Can this be less redundent?
    ($field_name:ident, $writer_name:ident, $type_name:ident) => {
        pub fn $field_name(&self) -> $type_name {
            self.$field_name
        }

        pub fn $writer_name(&mut self, val: $type_name) {
            self.$field_name = val
        }
    }
}

impl Wrap {
    pub fn new() -> Wrap {
        Default::default()
    }

    fn_option_accessor!(computed_grid_width, set_computed_grid_width, i16);
    fn_option_accessor!(computed_height, set_computed_height, i16);
    fn_option_accessor!(computed_width, set_computed_width, i16);
    fn_option_accessor!(computed_x, set_computed_x, i16);
    fn_option_accessor!(computed_y, set_computed_y, i16);
    fn_option_accessor!(grid_width, set_grid_width, i16);
    fn_option_accessor!(height, set_height, i16);
    fn_option_accessor!(width, set_width, i16);

    fn_accessor!(align, set_align, Align);
    fn_accessor!(has_border, set_has_border, bool);
    fn_accessor!(is_new_line, set_is_new_line, bool);
    fn_accessor!(margin, set_margin, i16);
    fn_accessor!(padding, set_padding, i16);
    fn_accessor!(vertical_align, set_vertical_align, VerticalAlign);

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, val: String) {
        self.name = val
    }

    pub fn outside_height(&self) -> Option<i16> {
        if let Some(mut h) = self.computed_height() {
            h += 2 *
                 (self.margin + self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            Some(h)
        } else {
            None
        }
    }

    pub fn set_outside_height(&mut self, val: Option<i16>) {
        if let Some(mut v) = val {
            v -= 2 *
                 (self.margin + self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            self.set_computed_height(Some(v));
        } else {
            self.set_computed_height(None);
        }
    }

    pub fn outside_width(&self) -> Option<i16> {
        if let Some(mut w) = self.computed_width() {
            w += 2 *
                 (self.margin + self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            Some(w)
        } else {
            None
        }
    }

    pub fn set_outside_width(&mut self, val: Option<i16>) {
        if let Some(mut v) = val {
            v -= 2 *
                 (self.margin + self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            self.set_computed_width(Some(v));
        } else {
            self.set_computed_width(None);
        }
    }

    pub fn outside_x(&self) -> Option<i16> {
        if let Some(mut x) = self.computed_x() {
            x -= self.margin + self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            Some(x)
        } else {
            None
        }
    }

    pub fn set_outside_x(&mut self, val: Option<i16>) {
        if let Some(mut v) = val {
            v += self.margin + self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            self.set_computed_x(Some(v));
        } else {
            self.set_computed_x(None);
        }
    }

    pub fn outside_y(&self) -> Option<i16> {
        if let Some(mut y) = self.computed_y() {
            y -= self.margin + self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            Some(y)
        } else {
            None
        }
    }

    pub fn set_outside_y(&mut self, val: Option<i16>) {
        if let Some(mut v) = val {
            v += self.margin + self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            self.set_computed_y(Some(v));
        } else {
            self.set_computed_y(None);
        }
    }

    pub fn border_height(&self) -> Option<i16> {
        if let Some(mut h) = self.computed_height() {
            h += 2 *
                 (self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            Some(h)
        } else {
            None
        }
    }

    pub fn border_width(&self) -> Option<i16> {
        if let Some(mut w) = self.computed_width() {
            w += 2 *
                 (self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            Some(w)
        } else {
            None
        }
    }

    pub fn border_x(&self) -> Option<i16> {
        if let Some(mut x) = self.computed_x() {
            x -= self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            Some(x)
        } else {
            None
        }
    }

    pub fn border_y(&self) -> Option<i16> {
        if let Some(mut y) = self.computed_y() {
            y -= self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            Some(y)
        } else {
            None
        }
    }
}

impl Default for Wrap {
    fn default() -> Wrap {
        Wrap {
            align: Default::default(),
            computed_grid_width: None,
            computed_height: None,
            computed_width: None,
            computed_x: None,
            computed_y: None,
            grid_width: None,
            has_border: false,
            height: None,
            is_new_line: false,
            margin: 0,
            name: String::new(), // maybe a uuid?
            padding: 0,
            vertical_align: Default::default(),
            width: None,
        }
    }
}

pub struct WrapBuilder {
    align: Option<Align>,
    grid_width: Option<i16>,
    has_border: Option<bool>,
    height: Option<i16>,
    margin: Option<i16>,
    name: Option<String>,
    padding: Option<i16>,
    vertical_align: Option<VerticalAlign>,
    width: Option<i16>,
}

macro_rules! fn_writer {
    ($field_name:ident, $type_name:ident) => {
        pub fn $field_name(mut self, val: $type_name) -> WrapBuilder {
            self.$field_name = Some(val);
            self
        }
    }
}

impl WrapBuilder {
    /// call this to create a column
    pub fn col(val: i16) -> WrapBuilder {
        WrapBuilder {
            align: None,
            grid_width: Some(val),
            has_border: None,
            height: None,
            margin: None,
            name: None,
            padding: None,
            vertical_align: None,
            width: None,
        }
    }

    /// call this to create a row
    pub fn row() -> WrapBuilder {
        WrapBuilder {
            align: None,
            grid_width: Some(GRID_COLUMNS_COUNT),
            has_border: None,
            height: None,
            margin: None,
            name: None,
            padding: None,
            vertical_align: None,
            width: None,
        }
    }

    fn_writer!(align, Align);
    fn_writer!(grid_width, i16);
    fn_writer!(has_border, bool);
    fn_writer!(height, i16);
    fn_writer!(margin, i16);
    fn_writer!(name, String);
    fn_writer!(padding, i16);
    fn_writer!(vertical_align, VerticalAlign);
    fn_writer!(width, i16);

    pub fn build(self) -> Wrap {
        let mut wrap = Wrap::new();

        if self.align.is_some() {
            wrap.set_align(self.align.unwrap())
        }
        if self.grid_width.is_some() {
            wrap.set_grid_width(self.grid_width)
        }
        if self.has_border.is_some() {
            wrap.set_has_border(self.has_border.unwrap())
        }
        if self.height.is_some() {
            wrap.set_height(self.height)
        }
        if self.margin.is_some() {
            wrap.set_margin(self.margin.unwrap())
        }
        if self.name.is_some() {
            wrap.set_name(self.name.unwrap())
        }
        if self.padding.is_some() {
            wrap.set_padding(self.padding.unwrap())
        }
        if self.vertical_align.is_some() {
            wrap.set_vertical_align(self.vertical_align.unwrap())
        }
        if self.width.is_some() {
            wrap.set_width(self.width)
        }

        wrap
    }
}
