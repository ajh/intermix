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

#[derive(Debug, Clone)]
pub struct Node {
    grid_width: GridWidth,
    children: Option<Vec<Node>>,
    size: Size,
    pos: Pos,
    widget: Option<Widget>,
}

impl Node {
    pub fn container(widget: Widget) -> Node {
        Node {
            grid_width: GridWidth::Max,
            children: None,
            size: Default::default(),
            pos: Default::default(),
            widget: Some(widget),
        }
    }

    pub fn row(children: Vec<Node>) -> Node {
        Node {
            grid_width: GridWidth::Max,
            children: Some(children),
            size: Default::default(),
            pos: Default::default(),
            widget: None,
        }
    }

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

    pub fn set_screen_width(&mut self, screen_width: u16) {
        match self.grid_width {
            GridWidth::Max => self.size.cols = screen_width,
            GridWidth::Cols(c) => {
                let percent = c as f32 / GRID_COLUMNS_COUNT as f32;
                self.size.cols = (screen_width as f32 * percent).floor() as u16;
            }
        }

        if let Some(children) = self.children.as_mut() {
            for child in children.iter_mut() {
                child.set_screen_width(screen_width);
            }
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
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }

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

#[derive(Debug, Clone)]
pub struct Screen {
    size: Size,
    root: Option<Node>,
}

impl Screen {
    pub fn new(size: Size, root: Option<Node>) -> Screen {
        Screen {
            size: size,
            root: root,
        }
    }

    pub fn display(&mut self) -> String {
        // rows then cols
        let mut scene: Vec<Vec<char>> = vec![vec![' '; self.size.cols as usize]; self.size.rows as usize];

        if let Some(root) = self.root.as_mut() {
            root.set_screen_width(self.size.cols);
            root.calc_height();
            root.set_pos(Pos { row: 0, col: 0 });

            for widget in root.widgets() {
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
