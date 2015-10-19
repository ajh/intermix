use std::io::{Write, Stdout};
use std::io;

use super::*;

/// a collection of cells and other metadata about the screen
#[derive(Debug)]
pub struct Screen {
    pub rows_count: usize,
    pub cols_count: usize,
    pub cells: Vec<Vec<Cell>>
}

impl Screen {
    pub fn new(rows_count: usize, cols_count: usize) -> Screen {
        Screen {
            rows_count: rows_count,
            cols_count: cols_count,

            // index rows first then cols, so like
            // cells[row_num][cols_num]
            cells: vec!(vec!(Default::default(); cols_count); rows_count)
        }
    }

    pub fn debug_draw(&self) {
        // go to top left
        io::stdout().write(b"\x1b[0;0H");

        for row in &(self.cells) {
            for cell in row {
                io::stdout().write(cell.ch.to_string().as_bytes());
            }
        }
    }
}

// TODO: test this
impl PartialEq for Screen {
    fn eq(&self, other: &Self) -> bool {
        let is_same_size = self.rows_count == other.rows_count
            && self.cols_count == other.cols_count;

        if !is_same_size {
            return false;
        }

        for (row_a, row_b) in self.cells.iter().zip(other.cells.iter()) {
            for (cell_a, cell_b) in row_a.iter().zip(row_b.iter()) {
                // TODO: check other stuff in cell besides ch
                if cell_a.ch != cell_b.ch {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_equal_when_screens_are_the_same() {
        let mut a = Screen::new(2, 2);
        let mut b = Screen::new(2, 2);
        assert_eq!(a, b);
    }

    #[test]
    #[should_panic]
    fn it_is_not_equal_when_screens_have_different_rows_count() {
        let mut a = Screen::new(1, 2);
        let mut b = Screen::new(2, 2);
        assert_eq!(a, b);
    }

    #[test]
    #[should_panic]
    fn it_is_not_equal_when_screens_have_different_colss_count() {
        let mut a = Screen::new(2, 1);
        let mut b = Screen::new(2, 2);
        assert_eq!(a, b);
    }

    #[test]
    #[should_panic]
    fn it_is_not_equal_when_screens_have_different_ch_in_cells() {
        let mut a = Screen::new(1, 1);
        a.cells[0][0].ch = 'a' as char;
        let mut b = Screen::new(1, 1);
        b.cells[0][0].ch = 'b' as char;
        assert_eq!(a, b);
    }
}
