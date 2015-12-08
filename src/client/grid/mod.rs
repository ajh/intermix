use std::slice::Iter;
use vterm_sys;

pub type Size = vterm_sys::ScreenSize;
pub type Pos = vterm_sys::Pos;

const GRID_COLUMNS_COUNT: u16 = 12;

pub trait Alignable {
    fn calc_height(&mut self);
    fn get_pos(&self) -> &Pos;
    fn get_size(&self) -> &Size;
    fn set_pos(&mut self, pos: Pos);
    fn set_width(&mut self, width: u16);
}

#[derive(Debug, Clone)]
pub struct Row {
    children: Vec<Column>,
    size: Size,
    pos: Pos,
}

impl Row {
    pub fn new(children: Vec<Column>) -> Row {
        Row {
            children: children,
            size: Default::default(),
            pos: Default::default(),
        }
    }

    pub fn calc_height(&mut self) {
        for child in &mut self.children {
            child.calc_height();
        }

        self.size.rows = match self.children.iter().map(|c| c.get_size().rows).min() {
            Some(h) => h,
            None => 0,
        };
    }

    pub fn set_width(&mut self, width: u16) {
        self.size.cols = width;

        for child in &mut self.children {
            let grid_width = child.grid_width; // borrowck workaround
            let w = (self.size.cols as f32 * (grid_width as f32 / GRID_COLUMNS_COUNT as f32)).floor();
            child.set_width(w as u16);
        }
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;

        let mut last_col = self.pos.col;
        for child in &mut self.children {
            child.set_pos(Pos { row: self.pos.row, col: last_col});
            last_col += child.get_size().cols as i16;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    /// how many grid columns wide
    pub grid_width: u16,
    size: Size,
    pos: Pos,

    widgets: Vec<Widget>
}

impl Column {
    pub fn new(grid_width: u16, widgets: Vec<Widget>) -> Column {
        Column {
            grid_width: grid_width,
            widgets: widgets,
            size: Default::default(),
            pos: Default::default(),
        }
    }

    pub fn calc_height(&mut self) {
        self.size.rows = self.widgets
            .iter()
            .fold(0, |sum, w| sum + w.size.rows as u16);
    }

    pub fn set_width(&mut self, width: u16) {
        self.size.cols = width;
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;

        let mut last_row = self.pos.row;
        for widget in &mut self.widgets {
            widget.set_pos(Pos { row: last_row, col: self.pos.col });
            last_row += widget.get_size().rows as i16;
        }
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
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
    width: u16,
    height: u16,
    rows: Vec<Row>,
}

impl Screen {
    pub fn new(width: u16, height: u16, rows: Vec<Row>) -> Screen {
        Screen {
            height: height,
            rows: rows,
            width: width,
        }
    }

    pub fn display(&mut self) -> String {
        let mut output: Vec<String> = vec![format!(""); self.height as usize];

        let mut last_row = 0;
        for row in &mut self.rows {
            row.set_width(self.width);
            row.calc_height();
            row.set_pos(Pos { row: last_row, col: 0 });
            last_row += row.size.rows as i16;
        }

        // rows then cols
        let mut scene: Vec<Vec<char>> = vec![vec![' '; self.width as usize]; self.height as usize];

        for widget in WidgetIter::new(&self) {
            if widget.get_pos().row as u16 >= self.height { continue }
            if widget.get_pos().col as u16 >= self.width { continue }

            let row_end = *[(widget.get_pos().row as u16) + widget.get_size().rows, self.height]
                .iter()
                .min()
                .unwrap();
            let col_end = *[(widget.get_pos().col as u16) + widget.get_size().cols, self.width]
                .iter()
                .min()
                .unwrap();

            for y in ((widget.get_pos().row as u16)..row_end) {
                for x in ((widget.get_pos().col as u16)..col_end) {
                    scene[y as usize][x as usize] = widget.fill;
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
pub struct WidgetIter<'a> {
    row_index: usize,
    col_index: usize,
    widget_index: usize,
    screen: &'a Screen,
}

impl<'a> WidgetIter<'a> {
    pub fn new(screen: &'a Screen) -> WidgetIter<'a> {
        WidgetIter {
            row_index: 0,
            col_index: 0,
            widget_index: 0,
            screen: screen,
        }
    }
}

impl<'a> Iterator for WidgetIter<'a> {
    type Item = &'a Widget;

    fn next(&mut self) -> Option<&'a Widget> {
        println!("{:?}", self);
        if self.row_index >= self.screen.rows.len() {
            println!("0");
            return None;
        }
        if self.col_index >= self.screen.rows[self.row_index].children.len() {
            println!("1");
            self.row_index += 1;
            self.col_index = 0;
            self.widget_index = 0;
            return self.next();
        }
        if self.widget_index >= self.screen.rows[self.row_index].children[self.col_index].widgets.len() {
            println!("2");
            self.col_index += 1;
            self.widget_index = 0;
            return self.next();
        }

        println!("3");
        let output = &self.screen.rows[self.row_index].children[self.col_index].widgets[self.widget_index];
        self.widget_index += 1;

        Some(output)
    }
}
