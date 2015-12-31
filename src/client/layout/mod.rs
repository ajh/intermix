mod node;
mod node_iters;

use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;
pub use self::node::*;
pub use self::node_iters::*;

pub type Size = vterm_sys::ScreenSize;
pub type Pos = vterm_sys::Pos;

/// Represents the layout for an entire screen. Contains one node which is the root.
#[derive(Debug, Clone)]
pub struct Layout {
    pub size: Size,
    pub root: Node,
}

impl Layout {
    pub fn new(size: Size, root: Node) -> Layout {
        Layout {
            size: size,
            root: root,
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
    pub fn calculate_layout(&mut self) {
        let grid_width = match self.root.grid_width {
            GridWidth::Max => GRID_COLUMNS_COUNT,
            GridWidth::Cols(c) => c,
        };
        self.root.calc_layout(grid_width);

        let width = match self.root.grid_width {
            GridWidth::Max => self.size.cols,
            GridWidth::Cols(c) => {
                let percent = c as f32 / GRID_COLUMNS_COUNT as f32;
                (self.size.cols as f32 * percent).round() as u16
            }
        };
        self.root.calc_width(width);
        self.root.calc_col_position(0);
        self.root.calc_height();
        self.root.set_row_pos(0);
    }
}
