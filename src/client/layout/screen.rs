use vterm_sys;
use ego_tree;

pub type Size = vterm_sys::ScreenSize;
pub type Pos = vterm_sys::Pos;

pub const GRID_COLUMNS_COUNT: i16 = 12;

/// Represents the Screen for an entire screen.
#[derive(Debug)]
pub struct Screen {
    pub size: Size,
    tree: ego_tree::Tree<Box>,
}

impl Screen {
    pub fn new(size: Size) -> Screen {
        // Maybe a Builder to clean this up? Or new takes an Option struct with defaults?
        let mut root = Box::new();
        root.set_computed_grid_width(GRID_COLUMNS_COUNT);
        root.set_computed_height(size.rows as i16);
        root.set_computed_width(size.cols as i16);
        root.set_computed_x(0);
        root.set_computed_y(0);
        root.set_grid_width(GRID_COLUMNS_COUNT);
        root.set_height(size.rows as i16);
        root.set_is_new_line(false);
        root.set_name("root".to_string());
        root.set_width(size.cols as i16);

        Screen {
            size: size,
            tree: ego_tree::Tree::new(root),
        }
    }

    pub fn tree(&self) -> &ego_tree::Tree<Box> {
        &self.tree
    }

    pub fn tree_mut(&mut self) -> &mut ego_tree::Tree<Box> {
        &mut self.tree
    }

    /// recalculate layout to account for changes to the screen
    pub fn flush_changes(&mut self) {
        let root_id = self.tree.root().id();
        self.compute_layout(root_id);
        self.compute_width(root_id);
        self.compute_x_position(root_id);
        self.compute_height(root_id);
        self.compute_y_position(root_id);
    }

    /// Assigns computed_grid_width and is_new_line values
    fn compute_layout(&mut self, parent_id: ego_tree::NodeId<Box>) {
        let parent_grid_width = self.tree.get(parent_id).value().computed_grid_width().unwrap();

        let mut columns_in_line = 0;

        let child_ids: Vec<ego_tree::NodeId<Box>> = self.tree.get(parent_id).children().map(|c| c.id()).collect();
        for child_id in child_ids {
            {
                let mut child_node = self.tree.get_mut(child_id);
                let mut child_box = child_node.value();

                let mut grid_width = child_box.grid_width().unwrap_or(GRID_COLUMNS_COUNT);
                if grid_width > parent_grid_width {
                    grid_width = parent_grid_width
                };
                child_box.set_computed_grid_width(grid_width);

                columns_in_line += grid_width;
                if columns_in_line > parent_grid_width {
                    columns_in_line = 0;
                    child_box.set_is_new_line(true);
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
    fn compute_width(&mut self, parent_id: ego_tree::NodeId<Box>) {
        let mut lines = self.tree.get(parent_id).lines();
        let parent_grid_width = self.tree.get(parent_id).value().computed_grid_width().unwrap();
        let parent_width = self.tree.get(parent_id).value().computed_width().unwrap();

        for mut line in lines {
            println!("{:?}", line);
            let mut line_width = 0;
            let mut line_grid_columns_count = 0;

            // calculate provisionary widths
            for child_id in line.iter() {
                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_box = child_ref.value();
                let percent = child_box.grid_width().unwrap() as f32 / parent_grid_width as f32;
                let width = (parent_width as f32 * percent).floor() as i16;

                child_box.set_computed_width(width);

                line_width += width;
                line_grid_columns_count += child_box.grid_width().unwrap();
            }

            // figure how many columns are unused due to rounding errors
            let mut unused_cols = {
                let percent = line_grid_columns_count as f32 / parent_grid_width as f32;

                let mut expected_width = (parent_width as f32 * percent).round() as i16;
                if expected_width > parent_width { expected_width = parent_width }

                expected_width - line_width
            };

            // add them back in fairly
            line.sort_by(|a,b| {
                let a_ref = self.tree.get(*a);
                let a_box = a_ref.value();
                let b_ref = self.tree.get(*b);
                let b_box = b_ref.value();
                a_box.computed_width().unwrap().cmp(&b_box.computed_width().unwrap())
            });

            for child_id in line.iter() {
                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_box = child_ref.value();

                if unused_cols > 0 {
                    unused_cols -= 1;
                }
                else {
                    break
                }

                let val = child_box.computed_width().unwrap() + 1;
                child_box.set_computed_width(val);
            }

            // recurse
            for child_id in line {
                self.compute_width(child_id);
            }
        }
    }

    fn compute_x_position(&mut self, parent_id: ego_tree::NodeId<Box>) {
        let mut lines = self.tree.get(parent_id).lines();
        let parent_width = self.tree.get(parent_id).value().computed_width().unwrap();
        let parent_x = self.tree.get(parent_id).value().computed_x().unwrap();

        for line in lines {
                //let a_ref = self.tree.get(*a);
                //let a_box = a_ref.value();
            let line_width = line.iter()
                .map(|id| self.tree.get(*id).value())
                .map(|b| b.outside_width().unwrap())
                .fold(0, ::std::ops::Add::add);
            let unused_cols = parent_width - line_width;
            let offset = 0; // todo alignment

            let mut x = parent_x + offset;

            for id in line {
                {
                    let mut child_ref = self.tree.get_mut(id);
                    let mut child_box = child_ref.value();
                    child_box.set_computed_x(x);
                }

                self.compute_x_position(id);
            }
        }
    }

    /// This one is botton up unlike the others which is top down. It returns the height of its
    /// children.
    fn compute_height(&mut self, parent_id: ego_tree::NodeId<Box>) -> i16 {
        let mut lines = self.tree.get(parent_id).lines();

        for line in lines.iter() {
            for child_id in line.iter() {
                let children_height = self.compute_height(*child_id);

                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_box = child_ref.value();
                let h = if let Some(i) = child_box.height() { i } else { children_height };
                child_box.set_computed_height(h);
            }
        }

        lines.iter()
            .map(|line| line.iter().map(|id| self.tree.get(*id).value()).map(|b| b.outside_height().unwrap()).max().unwrap())
            .fold(0, ::std::ops::Add::add)
    }

    fn compute_y_position(&mut self, parent_id: ego_tree::NodeId<Box>) {
        let mut lines = self.tree.get(parent_id).lines();
        let parent_height = self.tree.get(parent_id).value().computed_height().unwrap();
        let parent_y = self.tree.get(parent_id).value().computed_y().unwrap();

        let lines_height = lines.iter()
            .map(|line| line.iter().map(|id| self.tree.get(*id).value()).map(|n| n.outside_height().unwrap()).max().unwrap())
            .fold(0, ::std::ops::Add::add);
        let unused_rows = parent_height - lines_height;
        let offset = 0; // todo alignment

        let mut y = parent_y + offset;

        for line in lines.iter() {
            for child_id in line.iter() {
                {
                    let mut child_ref = self.tree.get_mut(*child_id);
                    let mut child_box = child_ref.value();
                    child_box.set_computed_y(y);
                }

                self.compute_y_position(*child_id);
            }

            y += line.iter().map(|id| self.tree.get(*id).value()).map(|n| n.outside_height().unwrap()).max().unwrap();
        }
    }
}

pub trait BoxContainer {
    // return vec of child elements organized into horizontal lines
    fn lines(&self) -> Vec<Vec<ego_tree::NodeId<Box>>>;
}

impl<'a> BoxContainer for ego_tree::NodeRef<'a, Box> {
    fn lines(&self) -> Vec<Vec<ego_tree::NodeId<Box>>> {
        let mut output = vec![];
        let mut line = vec![];

        for child in self.children() {
            if child.value().is_new_line() && line.len() > 0 {
                output.push(line);
                line = vec![];
            }

            line.push(child.id());
        }

        if line.len() > 0 { output.push(line) }

        output
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Box {
    computed_grid_width:  Option<i16>,
    computed_height:      Option<i16>,
    computed_width:       Option<i16>,
    computed_x:           Option<i16>,
    computed_y:           Option<i16>,
    grid_width:           Option<i16>,
    height:               Option<i16>,
    is_new_line:          bool,
    name:                 String,
    width:                Option<i16>,
}

macro_rules! accessor_optional_i16 {
    // Can this be less redundent?
    ($field_name:ident, $setter_name:ident) => {
        pub fn $field_name(&self) -> Option<i16> {
            self.$field_name
        }

        pub fn $setter_name(&mut self, val: i16) {
            self.$field_name = Some(val)
        }
    }
}

impl Box {
    pub fn new() -> Box {
        Default::default()
    }

    accessor_optional_i16!(computed_grid_width,  set_computed_grid_width);
    accessor_optional_i16!(computed_height,      set_computed_height);
    accessor_optional_i16!(computed_width,       set_computed_width);
    accessor_optional_i16!(computed_x,           set_computed_x);
    accessor_optional_i16!(computed_y,           set_computed_y);
    accessor_optional_i16!(grid_width,           set_grid_width);
    accessor_optional_i16!(height,               set_height);
    accessor_optional_i16!(width,                set_width);

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, val: String) {
        self.name = val
    }

    pub fn is_new_line(&self) -> bool {
      self.is_new_line
    }

    pub fn outside_height(&self) -> Option<i16> {
        // add box model stuff later
        self.computed_height()
    }

    pub fn outside_width(&self) -> Option<i16> {
        // add box model stuff later
        self.computed_width()
    }

    pub fn set_is_new_line(&mut self, val: bool) {
        self.is_new_line = val
    }
}

impl Default for Box {
    fn default() -> Box {
        Box {
            name:                 String::new(), // maybe a uuid?
            computed_grid_width:  None,
            computed_height:      None,
            computed_width:       None,
            computed_x:           None,
            computed_y:           None,
            grid_width:           None,
            height:               None,
            is_new_line:          false,
            width:                None,
        }
    }
}

mod tests {
    use super::*;
    use ego_tree;

    fn assert_scene_eq(actual: &str, expected: &str) {
        let actual = actual.trim();
        let expected = expected.trim();

        if actual != expected {
            panic!("scenes not equal.\nactual:\n{}\nexpected:\n{}", actual, expected);
        }
    }

    #[test]
    fn it_draws_an_empty_document() {
        let mut screen = Screen::new(Size { rows: 2, cols: 2});
        screen.flush_changes();
        assert_scene_eq(&draw_screen(&screen), "
····
·  ·
·  ·
····");
    }

    #[test]
    fn it_draws_a_single_row() {
        let mut row = Box::new();
        row.set_name("a".to_string());
        row.set_grid_width(12);
        row.set_height(2);

        let mut screen = Screen::new(Size { rows: 2, cols: 2});
        screen.tree_mut().root_mut().append(row);
        screen.flush_changes();

        println!("{:#?}", screen.tree());

        assert_scene_eq(&draw_screen(&screen), "
····
·aa·
·aa·
····");
    }

    fn draw_screen(screen: &Screen) -> String {
        // scene is 2d vec organized rows then cols
        let mut scene: Vec<Vec<char>> = vec![vec![' '; screen.size.cols as usize]; screen.size.rows as usize];

        let leafs: Vec<&Box> = screen.tree().nodes().filter(|n| n.parent().is_some() && !n.has_children()).map(|n| n.value()).collect();
        for leaf in leafs {
            if leaf.computed_x().unwrap() >= screen.size.cols as i16 { continue }
            if leaf.computed_y().unwrap() >= screen.size.rows as i16 { continue }

            let col_end = *[leaf.computed_x().unwrap() + leaf.computed_width().unwrap(), screen.size.cols as i16]
                .iter()
                .min()
                .unwrap();
            let row_end = *[leaf.computed_y().unwrap() + leaf.computed_height().unwrap(), screen.size.rows as i16]
                .iter()
                .min()
                .unwrap();

            for y in (leaf.computed_y().unwrap()..row_end) {
                for x in (leaf.computed_x().unwrap()..col_end) {
                    scene[y as usize][x as usize] = leaf.name().chars().next().unwrap();
                }
            }
        }

        // draw scene border
        {
            let width = scene.first().unwrap().len();
            for line in scene.iter_mut() {
                line.insert(0, '·');
                line.push('·');
            }

            let mut top_bottom = vec!['·'; width];
            top_bottom.insert(0, '·');
            top_bottom.push('·');

            scene.insert(0, top_bottom.clone());
            scene.push(top_bottom);
        }

        // convert 2d vec into a newline separated string
        scene.iter()
            .map(|row| row.iter().cloned().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
