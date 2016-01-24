use vterm_sys;
use sxd_document::{dom, Package};

pub type Size = vterm_sys::ScreenSize;
pub type Pos = vterm_sys::Pos;

pub const GRID_COLUMNS_COUNT: i16 = 12;

/// Represents the Screen for an entire screen.
#[derive(Debug)]
pub struct Screen {
    pub size: Size,
    package: Package,
}

impl Screen {
    pub fn new(size: Size) -> Screen {
        Screen {
            size: size,
            package: Package::new(),
        }
    }

    pub fn document(&self) -> dom::Document {
        self.package.as_document()
    }

    pub fn root(&self) -> dom::Root {
        self.document().root()
    }

    /// recalculate layout to account for changes to the screen
    pub fn flush_changes(&self) {
        self.layout_children(self.root().elements(), GRID_COLUMNS_COUNT);
        self.compute_children_widths(self.root().lines(), GRID_COLUMNS_COUNT, self.size.cols as i16);
        self.compute_x_position(self.root().lines(), 0, self.size.cols as i16);
        // calc col position
        // calc height
        // set row pos
    }

    fn layout_children(&self, children: Vec<dom::Element>, parent_grid_width: i16) {
        let mut columns_in_line = 0;

        for child in children {
            let mut grid_width = child.grid_width().unwrap_or(12);
            if grid_width > parent_grid_width { grid_width = parent_grid_width };
            child.set_computed_grid_width(grid_width);

            columns_in_line += grid_width;
            if columns_in_line > parent_grid_width {
                columns_in_line = 0;
                child.set_is_new_line(true);
            }

            self.layout_children(child.elements(), grid_width);
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
    fn compute_children_widths(&self, lines: Vec<Vec<dom::Element>>, parent_grid_width: i16, parent_width: i16) {
        for mut line in lines {
            let mut line_width = 0;
            let mut line_grid_columns_count = 0;

            // calculate provisionary widths
            for child in line.iter() {
                let mut width = 2;
                let percent = child.grid_width().unwrap() as f32 / parent_grid_width as f32;
                let width = (parent_width as f32 * percent).floor() as i16;

                child.set_computed_width(width);

                line_width += width;
                line_grid_columns_count += child.grid_width().unwrap();
            }

            // figure how many columns are unused due to rounding errors
            let mut unused_cols = {
                let percent = line_grid_columns_count as f32 / parent_grid_width as f32;

                let mut expected_width = (parent_width as f32 * percent).round() as i16;
                if expected_width > parent_width { expected_width = parent_width }

                expected_width - line_width
            };

            // add them back in fairly
            line.sort_by(|a,b| a.computed_width().unwrap().cmp(&b.computed_width().unwrap()));

            for child in line.iter() {
                if unused_cols > 0 {
                    unused_cols -= 1;
                }
                else {
                    break
                }

                child.set_computed_width(child.computed_width().unwrap() + 1);
            }

            // recurse
            for child in line {
                self.compute_children_widths(
                    child.lines(),
                    child.computed_grid_width().unwrap(),
                    child.computed_width().unwrap());
            }
        }
    }

    fn compute_x_position(&self, lines: Vec<Vec<dom::Element>>, parent_x: i16, parent_width: i16) {
        for line in lines {
            let line_width = line.iter()
                .map(|e| e.outside_width().unwrap() as i16)
                .fold(0, ::std::ops::Add::add);
            let unused_cols = parent_width - line_width;
            let offset = 0; // todo alignment

            let mut x = parent_x + offset;

            for element in line {
                element.set_computed_x(x);
                self.compute_x_position(
                    element.lines(),
                    element.computed_x().unwrap(),
                    element.computed_width().unwrap());
            }
        }
    }
}

pub trait ElementContainer {
    fn elements(&self) -> Vec<dom::Element>;
    fn lines(&self) -> Vec<Vec<dom::Element>>;
}

impl<'d> ElementContainer for dom::Element<'d> {
    fn elements(&self) -> Vec<dom::Element> {
        self.children()
            .into_iter()
            .filter_map(|c| c.element()).collect()
    }

    fn lines(&self) -> Vec<Vec<dom::Element>> {
        let mut output = vec![];
        let mut line = vec![];

        for child in self.elements().into_iter() {
            if child.is_new_line() && line.len() > 0 {
                output.push(line);
                line = vec![];
            }

            line.push(child);
        }

        if line.len() > 0 { output.push(line) }

        output
    }
}

impl<'d> ElementContainer for dom::Root<'d> {
    fn elements(&self) -> Vec<dom::Element> {
        self.children()
            .into_iter()
            .filter_map(|c| c.element()).collect()
    }

    fn lines(&self) -> Vec<Vec<dom::Element>> {
        let mut output = vec![];
        let mut line = vec![];

        for child in self.elements().into_iter() {
            if child.is_new_line() && line.len() > 0 {
                output.push(line);
                line = vec![];
            }

            line.push(child);
        }

        if line.len() > 0 { output.push(line) }

        output
    }
}

pub trait ScreenElement {
    fn computed_grid_width(&self) -> Option<i16>;
    fn computed_width(&self) -> Option<i16>;
    fn computed_x(&self) -> Option<i16>;
    fn computed_y(&self) -> Option<i16>;
    fn grid_width(&self) -> Option<i16>;
    fn is_new_line(&self) -> bool;
    fn outside_width(&self) -> Option<i16>;
    fn set_computed_grid_width(&self, i16);
    fn set_computed_width(&self, i16);
    fn set_computed_x(&self, i16);
    fn set_computed_y(&self, i16);
    fn set_grid_width(&self, i16);
    fn set_is_new_line(&self, bool);
    fn set_width(&self, i16);
    fn width(&self) -> Option<i16>;
}

macro_rules! accessor_optional_i16 {
    // Can this be less redundent?
    ($getter_name:ident, $setter_name:ident, $xml_name:expr) => {
        fn $getter_name(&self) -> Option<i16> {
            if let Some(val) = self.attribute_value($xml_name) {
                val.parse::<i16>().ok()
            }
            else {
                None
            }
        }

        fn $setter_name(&self, val: i16) {
            self.set_attribute_value($xml_name, &val.to_string());
        }
    }
}

impl<'d> ScreenElement for dom::Element<'d> {
    accessor_optional_i16!(computed_grid_width,  set_computed_grid_width,  "computed_grid_width");
    accessor_optional_i16!(computed_width,       set_computed_width,       "computed_width");
    accessor_optional_i16!(computed_x,           set_computed_x,           "computed_x");
    accessor_optional_i16!(computed_y,           set_computed_y,           "computed_y");
    accessor_optional_i16!(grid_width,           set_grid_width,           "grid_width");
    accessor_optional_i16!(width,                set_width,                "width");

    fn is_new_line(&self) -> bool {
      self.attribute_value("new_line").unwrap_or("false") == "true"
    }

    fn outside_width(&self) -> Option<i16> {
        // add box model stuff later
        self.computed_width()
    }

    fn set_is_new_line(&self, val: bool) {
        self.set_attribute_value("new_line", if val { "true" } else { "" });
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

    fn debug_document(document: & ::sxd_document::dom::Document) {
        ::sxd_document::writer::format_document(document, &mut ::std::io::stdout()).unwrap();
    }

    #[test]
    fn it_draws_an_empty_document() {
        let screen = Screen::new(Size { rows: 2, cols: 2});
        screen.flush_changes();
        assert_scene_eq(&draw_screen(&screen), "
~~~~
~  ~
~  ~
~~~~");
    }

    #[test]
    fn it_draws_a_single_row() {
        let screen = Screen::new(Size { rows: 2, cols: 2});
        let row = screen.document().create_element("box");
        row.set_attribute_value("name", "a");
        row.set_grid_width(12);
        screen.root().append_child(row);
        screen.flush_changes();

        debug_document(&screen.document());

        assert_scene_eq(&draw_screen(&screen), "
~~~~
~aa~
~aa~
~~~~");
    }

    fn draw_screen(screen: &Screen) -> String {
        // scene is 2d vec organized rows then cols
        let mut scene: Vec<Vec<char>> = vec![vec![' '; screen.size.cols as usize]; screen.size.rows as usize];

        // draw scene border
        let width = scene.first().unwrap().len();
        for line in scene.iter_mut() {
            line.insert(0, '~');
            line.push('~');
        }

        let mut top_bottom = vec!['~'; width];
        top_bottom.insert(0, '~');
        top_bottom.push('~');

        scene.insert(0, top_bottom.clone());
        scene.push(top_bottom);

        // convert 2d vec into a newline separated string
        scene.iter()
            .map(|row| row.iter().cloned().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
