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

        self.height = self.children.iter()
            .fold(999, |min, c| if c.height < min { c.height } else { min } );
    }

    pub fn set_width(&mut self, width: u16) {
        self.width = width;

        for child in &mut self.children {
            let grid_width = child.grid_width; // borrowck workaround
            println!("{} {} {}", self.width, grid_width, GRID_COLUMNS_COUNT);
            let w = (self.width as f32 * (grid_width as f32 / GRID_COLUMNS_COUNT as f32)).floor();
            println!("{}", w);
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
        for w in &mut self.widgets {
            w.width = self.width;
        }
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

    //pub fn widget_iter<'a>(&'a self) -> Iter<'a, Widget> {
        //self.widgets.iter()
    //}
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
    pub fn new(fill: char, height: u16) -> Widget {
        Widget {
            fill: fill,
            height: height,
            width: 0,
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
        let mut drawing: Vec<Vec<char>> = vec![vec![' '; self.width as usize]; self.height as usize];

        // Kwality code
        for row in &self.rows {
            for col in &row.children {
                for w in &col.widgets {
                    for y in (w.y..w.y+w.height) {
                        for x in (w.x..w.x+w.width) {
                            println!("{:?} y={} x={}", w, y, x);
                            drawing[y as usize][x as usize] = w.fill;
                        }
                    }
                }
            }
        }

        println!("{:?}", drawing);

        drawing.iter()
            .map(|row| row.iter().cloned().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
