use std::slice::Iter;

const GRID_COLUMNS_COUNT: u16 = 12;

#[derive(Debug, Clone)]
pub struct Row {
    children: Vec<Column>,

    width: u16,
    height: u16,
    x: u16,
    y: u16,
}

impl Row {
    pub fn new(children: Vec<Column>) -> Row {
        Row {
            children: children,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn calc_height(&mut self) {
        for child in &mut self.children {
            child.calc_height();
        }

        self.height = match self.children.iter().map(|c| c.height).min() {
            Some(h) => h,
            None => 0,
        };
    }

    pub fn set_width(&mut self, width: u16) {
        self.width = width;

        for child in &mut self.children {
            let grid_width = child.grid_width; // borrowck workaround
            let w = (self.width as f32 * (grid_width as f32 / GRID_COLUMNS_COUNT as f32)).floor();
            child.set_width(w as u16);
        }
    }

    pub fn set_y(&mut self, y: u16) {
        self.y = y;

        for child in &mut self.children {
            child.set_y(y);
        }
    }

    pub fn set_x(&mut self, y: u16) {
        self.y = y;

        let mut last_x = self.x;
        for child in &mut self.children {
            child.set_x(last_x);
            last_x += child.width;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    /// how many grid columns wide
    pub grid_width: u16,
    width: u16,
    height: u16,
    x: u16,
    y: u16,

    widgets: Vec<Widget>
}

impl Column {
    pub fn new(width: u16, widgets: Vec<Widget>) -> Column {
        Column {
            grid_width: width,
            widgets: widgets,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn calc_height(&mut self) {
        self.height = self.widgets
            .iter()
            .fold(0, |sum, w| sum + w.height);
    }

    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    pub fn set_y(&mut self, y: u16) {
        self.y = y;

        let mut last_y = self.y;
        for w in &mut self.widgets {
            w.y = last_y;
            last_y += w.height;
        }
    }

    pub fn set_x(&mut self, x: u16) {
        self.x = x;

        for w in &mut self.widgets {
            w.x = self.x
        }
    }
}

#[derive(Debug, Clone)]
pub struct Widget {
    fill: char,
    height: u16,
    width: u16,
    x: u16,
    y: u16,
}

impl Widget {
    pub fn new(fill: char, height: u16, width: u16) -> Widget {
        Widget {
            fill: fill,
            height: height,
            width: width,
            x: 0,
            y: 0,
        }
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

        let mut last_y = 0;
        for row in &mut self.rows {
            row.set_width(self.width);
            row.calc_height();
            row.set_y(last_y);
            row.set_x(0);
            last_y += row.height;
        }

        // rows then cols
        let mut scene: Vec<Vec<char>> = vec![vec![' '; self.width as usize]; self.height as usize];

        for widget in WidgetIter::new(&self) {
            if widget.y >= self.height { continue }
            if widget.x >= self.width { continue }

            let y_end = *[widget.y+widget.height, self.height]
                .iter()
                .min()
                .unwrap();
            let x_end = *[widget.x+widget.width, self.width]
                .iter()
                .min()
                .unwrap();

            for y in (widget.y..y_end) {
                for x in (widget.x..x_end) {
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
