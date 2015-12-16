mod node;
mod widget;
mod widgets;

use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;
pub use self::node::*;
pub use self::widget::*;
pub use self::widgets::*;

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
        self.root.calc_width(width, &self.size);
        self.root.calc_col_position(0, &self.size);
        self.root.calc_height(&self.size);
        self.root.set_row_pos(0, &self.size);
    }

    pub fn display(&mut self) -> String {
        self.calculate_layout();

        // scene is 2d vec organized rows then cols
        let mut scene: Vec<Vec<char>> = vec![vec![' '; self.size.cols as usize]; self.size.rows as usize];

        // draw widgets into scene
        for widget in self.root.widgets() {
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

        // draw border
        let width = scene.first().unwrap().len();
        for line in scene.iter_mut() {
            line.insert(0, '|');
            line.push('|');
        }

        let mut top_bottom = vec!['-'; width];
        top_bottom.insert(0, '.');
        top_bottom.push('.');

        scene.insert(0, top_bottom.clone());
        scene.push(top_bottom);

        // convert 2d vec into a newline separated string
        scene.iter()
            .map(|row| row.iter().cloned().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
