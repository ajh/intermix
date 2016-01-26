use vterm_sys;
use ego_tree;

pub type Size = vterm_sys::ScreenSize;
pub type Pos = vterm_sys::Pos;

pub const GRID_COLUMNS_COUNT: i16 = 12;

/// Represents the Screen for an entire screen.
#[derive(Debug)]
pub struct Screen {
    pub size: Size,
    tree: ego_tree::Tree<Wrap>,
}

impl Screen {
    pub fn new(size: Size) -> Screen {
        // Maybe a Builder to clean this up? Or new takes an Option struct with defaults?
        let mut root = Wrap::new();
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

    pub fn tree(&self) -> &ego_tree::Tree<Wrap> {
        &self.tree
    }

    pub fn tree_mut(&mut self) -> &mut ego_tree::Tree<Wrap> {
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
    fn compute_layout(&mut self, parent_id: ego_tree::NodeId<Wrap>) {
        let parent_grid_width = self.tree.get(parent_id).value().computed_grid_width().unwrap();

        let mut columns_in_line = 0;

        let child_ids: Vec<ego_tree::NodeId<Wrap>> = self.tree.get(parent_id).children().map(|c| c.id()).collect();
        for child_id in child_ids {
            {
                let mut child_node = self.tree.get_mut(child_id);
                let mut child_wrap = child_node.value();

                let mut grid_width = child_wrap.grid_width().unwrap_or(GRID_COLUMNS_COUNT);
                if grid_width > parent_grid_width {
                    grid_width = parent_grid_width
                };
                child_wrap.set_computed_grid_width(grid_width);

                columns_in_line += grid_width;
                if columns_in_line > parent_grid_width {
                    columns_in_line = 0;
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
    fn compute_width(&mut self, parent_id: ego_tree::NodeId<Wrap>) {
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
                let mut child_wrap = child_ref.value();
                let percent = child_wrap.grid_width().unwrap() as f32 / parent_grid_width as f32;
                let width = (parent_width as f32 * percent).floor() as i16;

                child_wrap.set_computed_width(width);

                line_width += width;
                line_grid_columns_count += child_wrap.grid_width().unwrap();
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
                let a_wrap = a_ref.value();
                let b_ref = self.tree.get(*b);
                let b_wrap = b_ref.value();
                a_wrap.computed_width().unwrap().cmp(&b_wrap.computed_width().unwrap())
            });

            for child_id in line.iter() {
                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_wrap = child_ref.value();

                if unused_cols > 0 {
                    unused_cols -= 1;
                }
                else {
                    break
                }

                let val = child_wrap.computed_width().unwrap() + 1;
                child_wrap.set_computed_width(val);
            }

            // recurse
            for child_id in line {
                self.compute_width(child_id);
            }
        }
    }

    fn compute_x_position(&mut self, parent_id: ego_tree::NodeId<Wrap>) {
        let mut lines = self.tree.get(parent_id).lines();
        let parent_width = self.tree.get(parent_id).value().computed_width().unwrap();
        let parent_x = self.tree.get(parent_id).value().computed_x().unwrap();

        for line in lines {
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
                    let mut child_wrap = child_ref.value();
                    child_wrap.set_computed_x(x);
                }

                self.compute_x_position(id);
            }
        }
    }

    /// This one is botton up unlike the others which is top down. It returns the height of its
    /// children.
    fn compute_height(&mut self, parent_id: ego_tree::NodeId<Wrap>) -> i16 {
        let mut lines = self.tree.get(parent_id).lines();

        for line in lines.iter() {
            for child_id in line.iter() {
                let children_height = self.compute_height(*child_id);

                let mut child_ref = self.tree.get_mut(*child_id);
                let mut child_wrap = child_ref.value();
                let h = if let Some(i) = child_wrap.height() { i } else { children_height };
                child_wrap.set_computed_height(h);
            }
        }

        lines.iter()
            .map(|line| line.iter().map(|id| self.tree.get(*id).value()).map(|b| b.outside_height().unwrap()).max().unwrap())
            .fold(0, ::std::ops::Add::add)
    }

    fn compute_y_position(&mut self, parent_id: ego_tree::NodeId<Wrap>) {
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
                    let mut child_wrap = child_ref.value();
                    child_wrap.set_computed_y(y);
                }

                self.compute_y_position(*child_id);
            }

            y += line.iter().map(|id| self.tree.get(*id).value()).map(|n| n.outside_height().unwrap()).max().unwrap();
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

        if line.len() > 0 { output.push(line) }

        output
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wrap {
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

impl Wrap {
    pub fn new() -> Wrap {
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

impl Default for Wrap {
    fn default() -> Wrap {
        Wrap {
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

pub struct WrapBuilder {
    name: Option<String>,
    grid_width: Option<i16>,
    height: Option<i16>,
    width: Option<i16>,
}

impl WrapBuilder {
    /// call this to create a column
    pub fn col(val: i16) -> WrapBuilder {
        WrapBuilder {
            name: None,
            grid_width: Some(val),
            height: None,
            width: None,
        }
    }

    /// call this to create a row
    pub fn row() -> WrapBuilder {
        WrapBuilder {
            name: None,
            grid_width: Some(GRID_COLUMNS_COUNT),
            height: None,
            width: None,
        }
    }

    /// call this if you don't want to specifiy a grid width
    pub fn new() -> WrapBuilder {
        WrapBuilder {
            name: None,
            grid_width: None,
            height: None,
            width: None,
        }
    }

    pub fn name(mut self, name: String) -> WrapBuilder {
        self.name = Some(name);
        self
    }
    pub fn height(mut self, val: i16) -> WrapBuilder {
        self.height = Some(val);
        self
    }
    pub fn width(mut self, val: i16) -> WrapBuilder {
        self.height = Some(val);
        self
    }
    pub fn build(self) -> Wrap {
        let mut wrap = Wrap::new();

        if self.name.is_some()       { wrap.set_name(self.name.unwrap()); }
        if self.grid_width.is_some() { wrap.set_grid_width(self.grid_width.unwrap()); }
        if self.height.is_some()     { wrap.set_height(self.height.unwrap()); }
        if self.width.is_some()      { wrap.set_width(self.width.unwrap()); }

        wrap
    }
}
