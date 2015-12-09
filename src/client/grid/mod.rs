use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;

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

    pub fn calc_width(&mut self, assigned_width: u16, screen_size: &Size) {
        self.size.cols = assigned_width;

        if let Some(widget) = self.widget.as_mut() {
            let mut s = widget.get_size().clone();
            s.cols = self.size.cols;
            widget.set_size(s);
        }

        //if self.children.is_some() {
            //let self_size_cols = self.size.cols;

            //for row in &mut self.children_wrapped_mut() {
                //let row_has_max = row.iter()
                    //.any(|c| match c.grid_width { GridWidth::Max => true, _ => false } );

                //if row_has_max {
                    //for child in row.iter_mut() {
                        //let width = match child.grid_width {
                            //GridWidth::Max => self_size_cols,
                            //GridWidth::Cols(c) => {
                                //warn!("Somehow a col node is sharing a line with a max node!");
                                //let percent = c as f32 / GRID_COLUMNS_COUNT as f32;
                                //(screen_size.cols as f32 * percent).floor() as u16
                            //}
                        //};

                        //child.calc_width(width, screen_size);
                    //}

                    //continue;
                //}

                //// calculate widths naively
                //// if there is a MAX in the row
                ////   assign widths
                //// else
                ////   calc total expected width in cols based on number of grid columns
                ////   apply difference starting at beginning

                //// tuple of (&mut Node, cols, delta, grid_cols)
                //let mut widths = vec![];
                //for child in row {
                    //let info = match child.grid_width {
                        //GridWidth::Cols(c) => {
                            //let percent = c as f32 / GRID_COLUMNS_COUNT as f32;
                            //let width_f32 = screen_size.cols as f32 * percent;
                            //let floor = width_f32.floor();
                            //(child, floor as u16, width_f32 - floor, c)
                        //},
                        //_ => (child,0,0.0,0),
                    //};

                    //println!("self_size_cols {} info {:?}", self_size_cols, info);
                    //widths.push(info);
                //}

                //let grid_columns = widths.iter()
                    //.map(|t| t.3)
                    //.fold(0, ::std::ops::Add::add);

                //let percent = grid_columns as f32 / GRID_COLUMNS_COUNT as f32;
                //let mut expected_width = (screen_size.cols as f32 * percent).round() as u16;
                //if expected_width > self_size_cols { expected_width = self_size_cols }
                //println!("expected_width {}", expected_width);

                //let actual_width = widths.iter()
                    //.map(|t| t.1)
                    //.fold(0, ::std::ops::Add::add);

                //let mut unused_cols = expected_width - actual_width;

                //widths.sort_by(|a,b| b.2.partial_cmp(&a.2).unwrap());
                //for (node, cols, _, _) in widths {
                    //let cols = if unused_cols > 0 {
                        //unused_cols -= 1;
                        //cols + 1
                    //}
                    //else {
                        //cols
                    //};

                    //node.calc_width(cols, screen_size);
                //}

                ////let row_has_max = row.iter()
                    ////.any(|c| match c.grid_width { GridWidth::Max => true, _ => false } );
                ////if row_has_max {
                    ////for (i, child) in row.iter_mut().enumerate() {
                        ////child.calc_width(naive_widths[i], screen_size);
                    ////}
                ////}
                ////else {
                    ////let grid_columns = row.iter()
                        ////.map(|c| match c.grid_width { GridWidth::Cols(c) => c, _ => 0 })
                        ////.fold(0, ::std::ops::Add::add);

                    ////let percent = grid_columns as f32 / GRID_COLUMNS_COUNT as f32;
                    ////let mut expected_width = (screen_size.cols as f32 * percent).round() as u16;
                    ////if expected_width > self_size_cols { expected_width = self_size_cols }

                    ////let actual_width = naive_widths.iter().fold(0, ::std::ops::Add::add);
                    ////println!("expected {} actual {} self_size_cols {} row {:?}", expected_width, actual_width, self_size_cols, row);

                    //// the fair way to add the extras is to add them to the ones with the biggest
                    //// difference between the floor value and the float value.
                ////}



                ////let expected_width =
                ////if sum < self.size.cols {
                ////}
            //}
        //}

        if let Some(children) = self.children.as_mut() {
            for child in children.iter_mut() {
                match child.grid_width {
                    GridWidth::Max => child.calc_width(self.size.cols, screen_size),
                    GridWidth::Cols(c) => {
                        let percent = c as f32 / GRID_COLUMNS_COUNT as f32;
                        let w = (screen_size.cols as f32 * percent).round() as u16;
                        child.calc_width(w, screen_size);
                    }
                }
            }
        }
    }

    // This depends on widths already being calculated
    pub fn calc_col_position(&mut self, assigned_col: i16, screen_size: &Size) {
        self.pos.col = assigned_col;

        if let Some(children) = self.children.as_mut() {
            let mut last_col = self.pos.col;

            for child in children.iter_mut() {
                if last_col as u16 + child.get_size().cols > screen_size.cols {
                    last_col = 0; // wrap
                }

                child.calc_col_position(last_col, screen_size);

                last_col += child.get_size().cols as i16;
            }
        }

        if let Some(widget) = self.widget.as_mut() {
            let mut p = widget.get_pos().clone();
            p.col = self.pos.col;
            widget.set_pos(p);
        }
    }

    pub fn calc_height(&mut self, screen_size: &Size) {
        if let Some(widget) = self.widget.as_ref() {
            self.size.rows = widget.size.rows as u16;
        }

        if let Some(children) = self.children.as_mut() {
            for child in children.iter_mut() {
                child.calc_height(screen_size);
            }
        }

        if self.children.is_some() {
            self.size.rows = self.children_wrapped()
                .iter()
                .map(|row| row.iter().map(|c| c.get_size().rows).max().unwrap())
                .fold(0, ::std::ops::Add::add);
        }
    }

    pub fn set_row_pos(&mut self, assigned_row: i16, screen_size: &Size) {
        self.pos.row = assigned_row;

        if let Some(widget) = self.widget.as_mut() {
            let mut p = widget.get_pos().clone();
            p.row = self.pos.row;
            widget.set_pos(p);
        }

        if self.children.is_some() {
            let mut current_row = self.pos.row;

            for row in &mut self.children_wrapped_mut() {
                for child in row.iter_mut() {
                    child.set_row_pos(current_row, screen_size);
                }
                current_row += row.iter().map(|n| n.get_size().rows).max().unwrap() as i16;
            }
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

    /// Return lists of Nodes that are one the same row. When a Node wraps below others it is
    /// returned in a new vec.
    ///
    /// For example if children are:
    ///
    /// * a row
    /// * a col size 4
    /// * a col size 4
    /// * a col size 8
    /// * a row
    ///
    /// we'd get these vecs:
    ///
    /// [
    ///   [a row],
    ///   [col size 4, col size 4],
    ///   [col size 8],
    ///   [row],
    /// ]
    ///
    fn children_wrapped(&self) -> Vec<Vec<&Node>> {
        let mut output = vec![];
        let mut row = vec![];

        if let Some(children) = self.children.as_ref() {
            for child in children.iter() {
                if child.get_pos().col == self.pos.col && row.len() > 0 {
                    output.push(row);
                    row = vec![];
                }

                row.push(child);
            }
        }

        if row.len() > 0 { output.push(row) }

        output
    }

    /// Is there a way to DRY this with the non-mutabile version?
    fn children_wrapped_mut(&mut self) -> Vec<Vec<&mut Node>> {
        let mut output = vec![];
        let mut row = vec![];

        if let Some(children) = self.children.as_mut() {
            for child in children.iter_mut() {
                if child.get_pos().col == self.pos.col && row.len() > 0 {
                    output.push(row);
                    row = vec![];
                }

                row.push(child);
            }
        }

        if row.len() > 0 { output.push(row) }

        output
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

    /// Here's the algo:
    ///
    /// 1. set node widths, since this depends on no other info
    /// 2. set node col positions, taking into account wrapping
    /// 3. set node heights
    /// 4. set node row positions
    ///
    /// Although, now I'm thinking I should figure out the wrapping first based on simple grid
    /// column counting.
    ///
    /// I think that opens the door to calculating width, height, and positions all at one go?
    /// Maybe?
    ///
    fn calculate_layout(&mut self) {
        if self.root.is_none() { return }
        let root = self.root.as_mut().unwrap();
        let width = match root.grid_width {
            GridWidth::Max => self.size.cols,
            GridWidth::Cols(c) => {
                let percent = c as f32 / GRID_COLUMNS_COUNT as f32;
                (self.size.cols as f32 * percent).round() as u16
            }
        };
        root.calc_width(width, &self.size);
        root.calc_col_position(0, &self.size);
        root.calc_height(&self.size);
        root.set_row_pos(0, &self.size);
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
