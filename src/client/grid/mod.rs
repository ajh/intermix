use std::slice::Iter;
use vterm_sys;

pub type Size = vterm_sys::ScreenSize;
pub type Pos = vterm_sys::Pos;

const GRID_COLUMNS_COUNT: u16 = 12;

#[derive(Debug, Clone)]
enum GridWidth {
    /// rows
    Max,
    /// cols
    Cols (u16),
}

/// A Node is a rectangle that gets aligned into a layout with other nodes.
///
/// It starts its life only knowing its grid width in a 12 column grid system. The size and
/// position values get calculated at run time based on its position in the layout.
///
/// There are three constructors for nodes:
///
/// * leaf - creates a node holding a widget. Cannot contain other nodes.
/// * row - creates a 12 grid width node.
/// * col - creates a node with the given grid width.
///
/// Nodes will try to position themselves to the right of a prior sibling node, but will wrap
/// without enough room.
#[derive(Debug, Clone)]
pub struct Node {
    grid_width: GridWidth,
    pos: Pos,
    size: Size,
    widget: Option<Widget>,
    children: Option<Vec<Node>>,
}

impl Node {
    /// Create a leaf node that holds a widget
    pub fn leaf(widget: Widget) -> Node {
        Node {
            grid_width: GridWidth::Max,
            children: None,
            size: Default::default(),
            pos: Default::default(),
            widget: Some(widget),
        }
    }

    /// Create a row node that is full width. It will always wrap below a prior sibling node if one
    /// exists.
    pub fn row(children: Vec<Node>) -> Node {
        Node {
            grid_width: GridWidth::Max,
            children: Some(children),
            size: Default::default(),
            pos: Default::default(),
            widget: None,
        }
    }

    /// Create a column node with the given grid width.
    pub fn col(grid_width: u16, children: Vec<Node>) -> Node {
        // TODO: validate grid_width value
        Node {
            grid_width: GridWidth::Cols (grid_width),
            children: Some(children),
            size: Default::default(),
            pos: Default::default(),
            widget: None,
        }
    }

    pub fn calc_height(&mut self) {
        if let Some(widget) = self.widget.as_ref() {
            self.size.rows = widget.size.rows as u16;
        }

        if let Some(children) = self.children.as_mut() {
            for child in children.iter_mut() {
                child.calc_height();
            }

            let max_height = children.iter()
                .map(|c| c.get_size().rows)
                .max();

            self.size.rows = match max_height {
                Some(h) => h,
                None => 0,
            };
        }
    }

    pub fn set_screen_width(&mut self, screen_width: u16, parent_width: u16) {
        match self.grid_width {
            GridWidth::Max => self.size.cols = parent_width,
            GridWidth::Cols(c) => {
                let percent = c as f32 / GRID_COLUMNS_COUNT as f32;
                self.size.cols = (screen_width as f32 * percent).round() as u16;
            }
        }

        if let Some(children) = self.children.as_mut() {
            for child in children.iter_mut() {
                child.set_screen_width(screen_width, self.size.cols);
            }
        }

        if let Some(widget) = self.widget.as_mut() {
            let mut s = widget.get_size().clone();
            s.cols = self.size.cols;
            widget.set_size(s);
        }
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;

        if let Some(children) = self.children.as_mut() {
            let mut last_col = self.pos.col;

            // TODO: this should flow things to a new line when they don't fit
            for child in children.iter_mut() {
                child.set_pos(Pos { row: self.pos.row, col: last_col});
                last_col += child.get_size().cols as i16;
            }
        }

        if let Some(widget) = self.widget.as_mut() {
            widget.set_pos(self.pos.clone());
        }
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }

    /// Return an iterator over all the widgets within this node or its descendants
    pub fn widgets(&self) -> Widgets {
        let mut widgets = vec![];

        if let Some(widget) = self.widget.as_ref() {
            widgets.push(widget);
        }

        if let Some(children) = self.children.as_ref() {
            for child in children.iter() {
                let mut more_widgets = child.widgets().collect::<Vec<&Widget>>();
                widgets.append(&mut more_widgets);
            }
        }

        Widgets {
            widgets: widgets,
            index: 0,
        }
    }
}

/// A widget is something that can be drawn to the screen.
///
/// Its size and position is calculated at run time.
#[derive(Debug, Clone)]
pub struct Widget {
    fill: char,
    size: Size,
    pos: Pos,
}

impl Widget {
    pub fn new(fill: char, size: Size) -> Widget {
        Widget {
            fill: fill,
            size: size,
            pos: Pos { row: 0, col: 0 },
        }
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}

/// Represents the layout for an entire screen. Contains one node which is the root.
#[derive(Debug, Clone)]
pub struct Screen {
    size: Size,
    root: Option<Node>,
}

impl Screen {
    pub fn new(size: Size, root: Node) -> Screen {
        Screen {
            size: size,
            root: Some(root),
        }
    }

    pub fn empty(size: Size) -> Screen {
        Screen {
            size: size,
            root: None,
        }
    }

    fn calculate_layout(&mut self) {
        if self.root.is_none() { return }
        let root = self.root.as_mut().unwrap();
        root.set_screen_width(self.size.cols, self.size.cols);
        root.calc_height();
        root.set_pos(Pos { row: 0, col: 0 });
    }

    pub fn display(&mut self) -> String {
        self.calculate_layout();
        println!("{:?}", self);

        // rows then cols
        let mut scene: Vec<Vec<char>> = vec![vec![' '; self.size.cols as usize]; self.size.rows as usize];

        if let Some(root) = self.root.as_mut() {
            for widget in root.widgets() {
                println!("{:?}", widget);

                if widget.get_pos().row as u16 >= self.size.rows { continue }
                if widget.get_pos().col as u16 >= self.size.cols { continue }

                let row_end = *[(widget.get_pos().row as u16) + widget.get_size().rows, self.size.rows]
                    .iter()
                    .min()
                    .unwrap();
                let col_end = *[(widget.get_pos().col as u16) + widget.get_size().cols, self.size.cols]
                    .iter()
                    .min()
                    .unwrap();

                for y in ((widget.get_pos().row as u16)..row_end) {
                    for x in ((widget.get_pos().col as u16)..col_end) {
                        scene[y as usize][x as usize] = widget.fill;
                    }
                }
            }
        }

        scene.iter()
            .map(|row| row.iter().cloned().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

/// An iterator for widgets within a Node.
#[derive(Debug)]
pub struct Widgets<'a> {
    widgets: Vec<&'a Widget>,
    index: usize,
}

impl<'a> Iterator for Widgets<'a> {
    type Item = &'a Widget;

    fn next(&mut self) -> Option<&'a Widget> {
        if self.index < self.widgets.len() {
            let w = Some(self.widgets[self.index]);
            self.index += 1;
            w
        } else {
            None
        }
    }
}
