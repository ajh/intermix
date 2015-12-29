use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;
use super::{Size, Pos, LeafIter};

pub const GRID_COLUMNS_COUNT: u16 = 12;

#[derive(Debug, Clone)]
pub enum GridWidth {
    /// Fill the width of the parent container
    Max,
    /// Be at most this many grid columns wide.
    Cols (u16),
}

#[derive(Debug, Clone)]
pub enum Align {
    Left,
    Center,
    Right,
}
impl Default for Align {
    fn default() -> Align { Align::Left }
}

#[derive(Debug, Clone)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}
impl Default for VerticalAlign {
    fn default() -> VerticalAlign { VerticalAlign::Top }
}

#[derive(Debug, Clone, Default)]
pub struct NodeOptions {
    pub align: Align,
    pub vertical_align: VerticalAlign,
    pub height: Option<u16>,
    pub width: Option<u16>,
}

/// A Node is a rectangle that gets aligned into a layout with other nodes.
///
/// It starts its life only knowing its grid width in a 12 column grid system. The size and
/// position values get calculated at run time based on its position in the layout.
///
/// There are three constructors for nodes:
///
/// * leaf - creates a node holding a value. Cannot contain other nodes.
/// * row - creates a 12 grid width node.
/// * col - creates a node with the given grid width.
///
/// Nodes will try to position themselves to the right of a prior sibling node, but will wrap
/// without enough room.
#[derive(Debug, Clone)]
pub struct Node {
    pub align               : Align,
    pub computed_grid_width : u16,
    pub computed_pos        : Pos,
    pub computed_size       : Size,
    pub grid_width          : GridWidth,
    pub height              : Option<u16>,
    pub new_line            : bool,
    pub vertical_align      : VerticalAlign,
    pub width               : Option<u16>,
    pub value               : String,

    pub children: Vec<Node>,
}

impl Node {
    /// Create a leaf node
    pub fn leaf_v2(value: String, options: NodeOptions) -> Node {
        Node {
            align               :  options.align,
            children            :  vec![],
            computed_grid_width :  0,
            computed_pos        :  Default::default(),
            computed_size       :  Default::default(),
            grid_width          :  GridWidth::Max,
            height              :  options.height,
            new_line            :  false,
            value               :  value,
            vertical_align      :  options.vertical_align,
            width               :  options.width,
        }
    }

    /// Create a row node that is full width. It will always wrap below a prior sibling node if one
    /// exists.
    pub fn row(options: NodeOptions, children: Vec<Node>) -> Node {
        Node {
            align               :  options.align,
            children            :  children,
            computed_grid_width :  0,
            computed_pos        :  Default::default(),
            computed_size       :  Default::default(),
            grid_width          :  GridWidth::Max,
            height              :  options.height,
            new_line            :  false,
            value               :  String::new(),
            vertical_align      :  options.vertical_align,
            width               :  options.width,
        }
    }

    /// Create a column node with the given grid width.
    pub fn col(grid_width: u16, options: NodeOptions, children: Vec<Node>) -> Node {
        // TODO: validate grid_width value
        Node {
            align               :  options.align,
            children            :  children,
            computed_grid_width :  0,
            computed_pos        :  Default::default(),
            computed_size       :  Default::default(),
            grid_width          :  GridWidth::Cols (grid_width),
            height              :  options.height,
            new_line            :  false,
            value               :  String::new(),
            vertical_align      :  options.vertical_align,
            width               :  options.width,
        }
    }

    /// Calculate computed grid columns and where nodes must wrap under other nodes
    ///
    /// The computed grid column may be smaller than the desired one, when for example a col 9 is
    /// inside a col 6.
    pub fn calc_layout(&mut self, assigned_grid_width: u16) {
        self.computed_grid_width = assigned_grid_width;

        let mut columns_in_row = 0;

        for child in self.children.iter_mut() {
            match child.grid_width {
                GridWidth::Max => {
                    columns_in_row = 0;
                    child.new_line = true;
                    child.calc_layout(self.computed_grid_width);
                }
                GridWidth::Cols(c) => {
                    columns_in_row += c;
                    if columns_in_row > GRID_COLUMNS_COUNT {
                        columns_in_row = 0;
                        child.new_line = true;
                    }

                    child.calc_layout(if c <= self.computed_grid_width { c } else { self.computed_grid_width });
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
        self.computed_size.cols = assigned_width;

        // copy these to work around borrowck issues
        let self_size_cols = self.computed_size.cols;
        let self_computed_grid_width = self.computed_grid_width;

        for line in &mut self.lines_mut() {
            let mut widths: Vec<WidthInfo> = line.iter()
                .enumerate()
                .map(|(i, child)| match child.grid_width {
                    GridWidth::Cols(c) => WidthInfo::new_from_col(c, screen_size.cols, i),
                    GridWidth::Max => WidthInfo::new_from_row(self_size_cols, self_computed_grid_width, i)})
                .sort_by(|a,b| b.delta.partial_cmp(&a.delta).unwrap());

            let mut unused_cols = {
                let grid_columns = widths.iter()
                    .map(|t| t.grid_columns)
                    .fold(0, ::std::ops::Add::add);

                let percent = grid_columns as f32 / GRID_COLUMNS_COUNT as f32;
                let mut expected_width = (screen_size.cols as f32 * percent).round() as u16;
                if expected_width > self_size_cols { expected_width = self_size_cols }

                let computed_width = widths.iter()
                    .map(|t| t.cols)
                    .fold(0, ::std::ops::Add::add);

                expected_width - computed_width
            };

            for info in widths {
                let mut cols = info.cols;
                if unused_cols > 0 {
                    unused_cols -= 1;
                    cols += 1;
                }

                line[info.child_index].calc_width(cols, screen_size);
            }
        }
    }

    // This depends on widths already being calculated
    pub fn calc_col_position(&mut self, assigned_col: i16, screen_size: &Size) {
        self.computed_pos.col = assigned_col;

        // copy to work around borrowck
        let self_pos_col = self.computed_pos.col;
        let self_size_cols = self.computed_size.cols;
        let align = self.align.clone();

        for line in &mut self.lines_mut() {
            let row_width = line.iter()
                .map(|c| c.get_computed_size().cols as i16)
                .fold(0, ::std::ops::Add::add);
            let unused_cols = self_size_cols as i16 - row_width;
            let pos_offet = match align {
                Align::Left => 0,
                Align::Center => (unused_cols as f32 / 2.0).floor() as i16,
                Align::Right => unused_cols,
            };

            let mut col = self_pos_col + pos_offet;

            for child in line {
                child.calc_col_position(col, screen_size);
                col += child.get_computed_size().cols as i16;
            }
        }
    }

    pub fn calc_height(&mut self, screen_size: &Size) {
        for child in self.children.iter_mut() {
            child.calc_height(screen_size);
        }

        if self.height.is_some() {
            self.computed_size.rows = self.height.clone().unwrap();
        }
        else if !self.children.is_empty() {
            self.computed_size.rows = self.lines()
                .iter()
                .map(|line| line.iter().map(|c| c.get_computed_size().rows).max().unwrap())
                .fold(0, ::std::ops::Add::add);
        }
    }

    pub fn set_row_pos(&mut self, assigned_row: i16, screen_size: &Size) {
        self.computed_pos.row = assigned_row;

        if !self.children.is_empty() {
            // copy to work around borrowck
            let self_size_rows = self.computed_size.rows;
            let v_align = self.vertical_align.clone();
            let self_computed_pos_row = self.computed_pos.row;

            let mut lines = self.lines_mut();
            let total_height = lines.iter()
                .map(|line| line.iter().map(|n| n.get_computed_size().rows).max().unwrap() as i16)
                .fold(0, ::std::ops::Add::add);
            let unused_rows = self_size_rows as i16 - total_height;
            let offset = match v_align {
                VerticalAlign::Top => 0,
                VerticalAlign::Middle => (unused_rows as f32 / 2.0).floor() as i16,
                VerticalAlign::Bottom => unused_rows,
            };

            let mut current_row = self_computed_pos_row + offset;

            for line in lines.iter_mut() {
                for child in line.iter_mut() {
                    child.set_row_pos(current_row, screen_size);
                }
                current_row += line.iter().map(|n| n.get_computed_size().rows).max().unwrap() as i16;
            }
        }
    }

    pub fn get_computed_size(&self) -> &Size {
        &self.computed_size
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    // Return an iterator over all leaf nodes which are descendants of this node
    pub fn leaf_iter(&self) -> LeafIter {
        let mut leafs = vec![];

        if self.is_leaf() {
            leafs.push(self);
        }

        for child in self.children.iter() {
            let mut more_leaves = child.leaf_iter().collect::<Vec<&Node>>();
            leafs.append(&mut more_leaves);
        }

        LeafIter {
            nodes: leafs,
            index: 0,
        }
    }

    /// Return lists of Nodes that are one the same line. When a Node wraps below others it is
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
    fn lines(&self) -> Vec<Vec<&Node>> {
        let mut output = vec![];
        let mut line = vec![];

        for child in self.children.iter() {
            if child.new_line && line.len() > 0 {
                output.push(line);
                line = vec![];
            }

            line.push(child);
        }

        if line.len() > 0 { output.push(line) }

        output
    }

    /// Is there a way to DRY this with the non-mutabile version?
    fn lines_mut(&mut self) -> Vec<Vec<&mut Node>> {
        let mut output = vec![];
        let mut line = vec![];

        for child in self.children.iter_mut() {
            if child.new_line && line.len() > 0 {
                output.push(line);
                line = vec![];
            }

            line.push(child);
        }

        if line.len() > 0 { output.push(line) }

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
