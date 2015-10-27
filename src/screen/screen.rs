use std::io::{self, Write, Stdout};
use std::fmt;

use super::*;

/// a collection of cells and other metadata about the screen
pub struct Screen {
    pub rows_count: usize,
    pub cols_count: usize,
    pub cells: Vec<Vec<Cell>>
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // cells_str will look like this:
        //
        //   01234567890123
        //  0      eiei
        //  1    h
        //  2 d
        //  3    hiekje
        //  4
        //  5 hello world
        //
        let mut cells_str = " ".to_string();

        for i in 0..self.rows_count {
            if i > 10 {
                cells_str.push_str("...");
                break;
            }

            cells_str.push_str(&(i % 10).to_string());
        }


        for (i, row) in self.cells.iter().enumerate() {
            if i > 10 {
                cells_str.push_str("...");
                continue
            }

            cells_str.push_str(&format!("\n{} ", i.to_string()));

            for (j, cell) in row.iter().enumerate() {
                if j > 10 {
                    cells_str.push_str("...");
                    continue
                }

                let ch = if cell.ch == '\x00' as char {
                    ' ' // TODO: There are other unprintables that should get the same treatment
                }
                else {
                    cell.ch
                };

                cells_str.push(ch); // Could add other attrs here too
            }
        }

        write!(f, "Screen ( rows_count: {}, cols_count: {},\n{}\n)", self.rows_count, self.cols_count, cells_str)
    }
}

impl Screen {
    pub fn new(rows_count: usize, cols_count: usize) -> Screen {
        // index rows first then cols, so like
        // cells[row_num][cols_num]
        let mut cells = vec!();
        for x in 0..rows_count {
            cells.push(vec!());
            for y in 0..cols_count {
                cells[x].push(Cell { x: x, y: y, ..Default::default() });
            }
        }

        Screen {
            rows_count: rows_count,
            cols_count: cols_count,
            cells:      cells,
        }
    }

    pub fn update_cell(&mut self, x: usize, y: usize, ch: char) {
        if self.cells[y as usize][x as usize].ch != ch {
          self.cells[y as usize][x as usize].ch = ch;
          self.cells[y as usize][x as usize].dirty = true;
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
    fn it_is_not_equal_when_screens_have_different_cols_count() {
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
