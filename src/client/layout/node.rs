use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;
use super::{Size, Pos, Widget, Widgets};

pub const GRID_COLUMNS_COUNT: u16 = 12;

#[derive(Debug, Clone)]
pub enum GridWidth {
    /// rows
    Max,
    /// cols
    Cols (u16),
}

#[derive(Debug, Clone)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct NodeOptions {
    pub align: Align,
    pub vertical_align: VerticalAlign,
}

impl Default for NodeOptions {
    fn default() -> NodeOptions {
        NodeOptions {
            align: Align::Left,
            vertical_align: VerticalAlign::Top,
        }
    }
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
    actual_grid_width: u16,
    pub grid_width: GridWidth,
    pos: Pos,
    size: Size,
    /// whether this node wrapped below its earlier siblings
    is_below: bool,
    widget: Option<Widget>,
    pub options: NodeOptions,
    pub children: Option<Vec<Node>>,
}

impl Node {
    /// Create a leaf node that holds a widget
    pub fn leaf(widget: Widget) -> Node {
        Node {
            actual_grid_width: 0,
            children: None,
            grid_width: GridWidth::Max,
            is_below: false,
            options: Default::default(),
            pos: Default::default(),
            size: Default::default(),
            widget: Some(widget),
        }
    }

    /// Create a row node that is full width. It will always wrap below a prior sibling node if one
    /// exists.
    pub fn row(options: NodeOptions, children: Vec<Node>) -> Node {
        Node {
            actual_grid_width: 0,
            children: Some(children),
            grid_width: GridWidth::Max,
            is_below: false,
            options: options,
            pos: Default::default(),
            size: Default::default(),
            widget: None,
        }
    }

    /// Create a column node with the given grid width.
    pub fn col(grid_width: u16, options: NodeOptions, children: Vec<Node>) -> Node {
        // TODO: validate grid_width value
        Node {
            actual_grid_width: 0,
            children: Some(children),
            grid_width: GridWidth::Cols (grid_width),
            is_below: false,
            options: options,
            pos: Default::default(),
            size: Default::default(),
            widget: None,
        }
    }

    /// Calculate actual grid columns and where nodes must wrap under other nodes
    ///
    /// The actual grid column may be smaller than the desired one, when for example a col 9 is
    /// inside a col 6.
    pub fn calc_layout(&mut self, assigned_grid_width: u16) {
        self.actual_grid_width = assigned_grid_width;

        if let Some(children) = self.children.as_mut() {
            let mut columns_in_row = 0;

            for child in children.iter_mut() {
                match child.grid_width {
                    GridWidth::Max => {
                        columns_in_row = 0;
                        child.is_below = true;
                        child.calc_layout(self.actual_grid_width);
                    }
                    GridWidth::Cols(c) => {
                        columns_in_row += c;
                        if columns_in_row > GRID_COLUMNS_COUNT {
                            columns_in_row = 0;
                            child.is_below = true;
                        }

                        child.calc_layout(if c <= self.actual_grid_width { c } else { self.actual_grid_width });
                    }
                }
            }
        }
    }

    /// Here's the algo:
    ///
    /// 1. iterate through children by rows.
    ///
    /// 2. For each row, figure out a couple things:
    /// * how many cols are missing due to rounding errors
    /// * which nodes are the most effected
    ///
    /// 3. Assign widths to child nodes, adding back the missing columns to the most effect nodes.
    pub fn calc_width(&mut self, assigned_width: u16, screen_size: &Size) {
        self.size.cols = assigned_width;

        if let Some(widget) = self.widget.as_mut() {
            let mut s = widget.get_size().clone();
            s.cols = self.size.cols;
            widget.set_size(s);
        }

        if self.children.is_some() {
            // copy these to work around borrowck issues
            let self_size_cols = self.size.cols;
            let self_actual_grid_width = self.actual_grid_width;

            for row in &mut self.children_wrapped_mut() {
                let mut widths: Vec<WidthInfo> = row.iter()
                    .enumerate()
                    .map(|(i, child)| match child.grid_width {
                        GridWidth::Cols(c) => WidthInfo::new_from_col(c, screen_size.cols, i),
                        GridWidth::Max => WidthInfo::new_from_row(self_size_cols, self_actual_grid_width, i)})
                    .sort_by(|a,b| b.delta.partial_cmp(&a.delta).unwrap());

                let mut unused_cols = {
                    let grid_columns = widths.iter()
                        .map(|t| t.grid_columns)
                        .fold(0, ::std::ops::Add::add);

                    let percent = grid_columns as f32 / GRID_COLUMNS_COUNT as f32;
                    let mut expected_width = (screen_size.cols as f32 * percent).round() as u16;
                    if expected_width > self_size_cols { expected_width = self_size_cols }

                    let actual_width = widths.iter()
                        .map(|t| t.cols)
                        .fold(0, ::std::ops::Add::add);

                    expected_width - actual_width
                };

                for info in widths {
                    let mut cols = info.cols;
                    if unused_cols > 0 {
                        unused_cols -= 1;
                        cols += 1;
                    }

                    row[info.child_index].calc_width(cols, screen_size);
                }
            }
        }
    }

    // This depends on widths already being calculated
    pub fn calc_col_position(&mut self, assigned_col: i16, screen_size: &Size) {
        self.pos.col = assigned_col;

        if self.children.is_some() {
            // copy to work around borrowck
            let self_pos_col = self.pos.col;
            let self_size_cols = self.size.cols;
            let align = self.options.align.clone();

            for row in &mut self.children_wrapped_mut() {
                let row_width = row.iter()
                    .map(|c| c.get_size().cols as i16)
                    .fold(0, ::std::ops::Add::add);
                let unused_cols = self_size_cols as i16 - row_width;
                let pos_offet = match align {
                    Align::Left => 0,
                    Align::Center => (unused_cols as f32 / 2.0).floor() as i16,
                    Align::Right => unused_cols,
                };

                let mut col = self_pos_col + pos_offet;

                for child in row {
                    child.calc_col_position(col, screen_size);
                    col += child.get_size().cols as i16;
                }
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
                if child.is_below && row.len() > 0 {
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
                if child.is_below && row.len() > 0 {
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

/// This is an internal data structure used when calculating width
#[derive(Debug)]
struct WidthInfo {
    cols: u16,
    delta: f32,
    grid_columns: u16,
    child_index: usize,
}

impl WidthInfo {
    pub fn new_from_col(grid_columns: u16, screen_width: u16, child_index: usize) -> WidthInfo {
        let percent = grid_columns as f32 / GRID_COLUMNS_COUNT as f32;
        let width_f32 = screen_width as f32 * percent;
        let floor = width_f32.floor();
        WidthInfo {
            cols: floor as u16,
            delta: width_f32 - floor,
            grid_columns: grid_columns,
            child_index: child_index,
        }
    }

    pub fn new_from_row(parent_width: u16, parent_grid_columns: u16, child_index: usize) -> WidthInfo {
        WidthInfo {
            cols: parent_width,
            delta: 0.0,
            grid_columns: parent_grid_columns,
            child_index: child_index,
        }
    }
}
