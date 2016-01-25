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
        Screen {
            size: size,
            tree: ego_tree::Tree::new(Default::default()),
        }
    }

    /// recalculate layout to account for changes to the screen
    pub fn flush_changes(&self) {
        //self.layout_children(self.root().elements(), GRID_COLUMNS_COUNT);
        //self.compute_children_widths(self.root().lines(), GRID_COLUMNS_COUNT, self.size.cols as i16);
        //self.compute_x_position(self.root().lines(), 0, self.size.cols as i16);
        //self.compute_children_heights(self.root().lines());
        //self.compute_y_position(self.root().lines(), 0, self.size.rows as i16);
    }

    //fn layout_children(&self, children: Vec<dom::Element>, parent_grid_width: i16) {
        //let mut columns_in_line = 0;

        //for child in children {
            //let mut grid_width = child.grid_width().unwrap_or(12);
            //if grid_width > parent_grid_width { grid_width = parent_grid_width };
            //child.set_computed_grid_width(grid_width);

            //columns_in_line += grid_width;
            //if columns_in_line > parent_grid_width {
                //columns_in_line = 0;
                //child.set_is_new_line(true);
            //}

            //self.layout_children(child.elements(), grid_width);
        //}
    //}

    ///// Here's the algo:
    /////
    ///// 1. iterate through children by lines.
    /////
    ///// 2. For each lines, figure out:
    ///// * how many cols are missing due to rounding errors
    ///// * which nodes are the most effected
    /////
    ///// 3. Assign widths to child nodes, adding back the missing columns to the most effect nodes.
    //fn compute_children_widths(&self, lines: Vec<Vec<dom::Element>>, parent_grid_width: i16, parent_width: i16) {
        //for mut line in lines {
            //let mut line_width = 0;
            //let mut line_grid_columns_count = 0;

            //// calculate provisionary widths
            //for child in line.iter() {
                //let mut width = 2;
                //let percent = child.grid_width().unwrap() as f32 / parent_grid_width as f32;
                //let width = (parent_width as f32 * percent).floor() as i16;

                //child.set_computed_width(width);

                //line_width += width;
                //line_grid_columns_count += child.grid_width().unwrap();
            //}

            //// figure how many columns are unused due to rounding errors
            //let mut unused_cols = {
                //let percent = line_grid_columns_count as f32 / parent_grid_width as f32;

                //let mut expected_width = (parent_width as f32 * percent).round() as i16;
                //if expected_width > parent_width { expected_width = parent_width }

                //expected_width - line_width
            //};

            //// add them back in fairly
            //line.sort_by(|a,b| a.computed_width().unwrap().cmp(&b.computed_width().unwrap()));

            //for child in line.iter() {
                //if unused_cols > 0 {
                    //unused_cols -= 1;
                //}
                //else {
                    //break
                //}

                //child.set_computed_width(child.computed_width().unwrap() + 1);
            //}

            //// recurse
            //for child in line {
                //self.compute_children_widths(
                    //child.lines(),
                    //child.computed_grid_width().unwrap(),
                    //child.computed_width().unwrap());
            //}
        //}
    //}

    //fn compute_x_position(&self, lines: Vec<Vec<dom::Element>>, parent_x: i16, parent_width: i16) {
        //for line in lines {
            //let line_width = line.iter()
                //.map(|e| e.outside_width().unwrap())
                //.fold(0, ::std::ops::Add::add);
            //let unused_cols = parent_width - line_width;
            //let offset = 0; // todo alignment

            //let mut x = parent_x + offset;

            //for element in line {
                //element.set_computed_x(x);
                //self.compute_x_position(
                    //element.lines(),
                    //element.computed_x().unwrap(),
                    //element.computed_width().unwrap());
            //}
        //}
    //}

    ///// This one is botton up unlike the others which is top down. It returns the height of its
    ///// children.
    //fn compute_children_heights(&self, lines: Vec<Vec<dom::Element>>) -> i16 {
        //for line in lines.iter() {
            //for child in line.iter() {
                //let h = self.compute_children_heights(child.lines());
                //child.set_computed_height(if let Some(i) = child.height() { i } else { h });
            //}
        //}

        //lines.iter()
            //.map(|line| line.iter().map(|c| c.outside_height().unwrap()).max().unwrap())
            //.fold(0, ::std::ops::Add::add)
    //}

    //fn compute_y_position(&self, lines: Vec<Vec<dom::Element>>, parent_y: i16, parent_height: i16) {
        //let lines_height = lines.iter()
            //.map(|line| line.iter().map(|n| n.outside_height().unwrap()).max().unwrap())
            //.fold(0, ::std::ops::Add::add);
        //let unused_rows = parent_height - lines_height;
        //let offset = 0; // todo alignment

        //let mut y = parent_y + offset;

        //for line in lines.iter() {
            //for element in line.iter() {
                //element.set_computed_y(y);
                //self.compute_y_position(
                    //element.lines(),
                    //element.computed_y().unwrap(),
                    //element.computed_height().unwrap());
            //}

            //y += line.iter().map(|n| n.outside_height().unwrap()).max().unwrap();
        //}
    //}
}

//pub trait ElementContainer {
    //// return vec of child elements
    //fn elements(&self) -> Vec<dom::Element>;

    //// return vec of child elements organized into horizontal lines
    //fn lines(&self) -> Vec<Vec<dom::Element>> {
        //let mut output = vec![];
        //let mut line = vec![];

        //for child in self.elements().into_iter() {
            //if child.is_new_line() && line.len() > 0 {
                //output.push(line);
                //line = vec![];
            //}

            //line.push(child);
        //}

        //if line.len() > 0 { output.push(line) }

        //output
    //}
//}

//impl<'d> ElementContainer for dom::Element<'d> {
    //fn elements(&self) -> Vec<dom::Element> {
        //self.children()
            //.into_iter()
            //.filter_map(|c| c.element()).collect()
    //}
//}

//impl<'d> ElementContainer for dom::Root<'d> {
    //fn elements(&self) -> Vec<dom::Element> {
        //self.children()
            //.into_iter()
            //.filter_map(|c| c.element()).collect()
    //}
//}

#[derive(Debug, Clone, PartialEq)]
pub enum GridWidth {
    /// Fill the width of the parent container
    Max,
    /// Be at most this many grid columns wide.
    Cols (i16),
}

#[derive(Debug, Clone, PartialEq)]
struct Box {
    computed_grid_width:  Option<i16>,
    computed_height:      Option<i16>,
    computed_width:       Option<i16>,
    computed_x:           Option<i16>,
    computed_y:           Option<i16>,
    grid_width:           GridWidth,
    height:               Option<i16>,
    is_new_line:          bool,
    width:                Option<i16>,
}

macro_rules! accessor_optional_i16 {
    // Can this be less redundent?
    ($field_name:ident, $setter_name:ident) => {
        fn $field_name(&self) -> Option<i16> {
            self.$field_name
        }

        fn $setter_name(&mut self, val: i16) {
            self.$field_name = Some(val)
        }
    }
}

impl Box {
    accessor_optional_i16!(computed_grid_width,  set_computed_grid_width);
    accessor_optional_i16!(computed_height,      set_computed_height);
    accessor_optional_i16!(computed_width,       set_computed_width);
    accessor_optional_i16!(computed_x,           set_computed_x);
    accessor_optional_i16!(computed_y,           set_computed_y);
    accessor_optional_i16!(height,               set_height);
    accessor_optional_i16!(width,                set_width);

    fn grid_width(&self) -> GridWidth {
        self.grid_width
    }

    fn set_grid_width(&mut self, val: GridWidth) {
        self.grid_width = val
    }

    fn is_new_line(&self) -> bool {
      self.is_new_line
    }

    fn outside_height(&self) -> Option<i16> {
        // add box model stuff later
        self.computed_height()
    }

    fn outside_width(&self) -> Option<i16> {
        // add box model stuff later
        self.computed_width()
    }

    fn set_is_new_line(&mut self, val: bool) {
        self.is_new_line = val
    }
}

impl Default for Box {
    fn default() -> Box {
        Box {
            computed_grid_width:  None,
            computed_height:      None,
            computed_width:       None,
            computed_x:           None,
            computed_y:           None,
            grid_width:           GridWidth::Max,
            height:               None,
            is_new_line:          false,
            width:                None,
        }
    }
}

mod tests {
    use super::*;

    fn assert_scene_eq(actual: &str, expected: &str) {
        let actual = actual.trim();
        let expected = expected.trim();

        if actual != expected {
            panic!("scenes not equal.\nactual:\n{}\nexpected:\n{}", actual, expected);
        }
    }

    #[test]
    fn it_draws_an_empty_document() {
        let screen = Screen::new(Size { rows: 2, cols: 2});
        screen.flush_changes();
        assert_scene_eq(&draw_screen(&screen), "
····
·  ·
·  ·
····");
    }

    #[test]
    fn it_draws_a_single_row() {
        let screen = Screen::new(Size { rows: 2, cols: 2});
        let row = screen.document().create_element("box");
        row.set_attribute_value("name", "a");
        row.set_grid_width(12);
        row.set_height(2);
        screen.root().append_child(row);
        screen.flush_changes();

        assert_scene_eq(&draw_screen(&screen), "
····
·aa·
·aa·
····");
    }

    fn draw_screen(screen: &Screen) -> String {
        // scene is 2d vec organized rows then cols
        let mut scene: Vec<Vec<char>> = vec![vec![' '; screen.size.cols as usize]; screen.size.rows as usize];

        //for node in xpath.find("box") {
            //let element = node.element().unwrap();
            //println!("{:?}", element);
            //if element.computed_x().unwrap() >= screen.size.cols as i16 { continue }
            //if element.computed_y().unwrap() >= screen.size.rows as i16 { continue }

            //let col_end = *[element.computed_x().unwrap() + element.computed_width().unwrap(), screen.size.cols as i16]
                //.iter()
                //.min()
                //.unwrap();
            //let row_end = *[element.computed_y().unwrap() + element.computed_height().unwrap(), screen.size.rows as i16]
                //.iter()
                //.min()
                //.unwrap();

            //for y in (element.computed_y().unwrap()..row_end) {
                //for x in (element.computed_x().unwrap()..col_end) {
                    //let name = element.attribute_value("name").unwrap();
                    //scene[y as usize][x as usize] = name.chars().next().unwrap();
                //}
            //}
        //}

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
