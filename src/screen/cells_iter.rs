use super::Cell;

pub struct CellsIter<'a> {
    index: usize,
    cells: &'a Vec<Vec<Cell>>,
}

impl<'a> CellsIter<'a> {
    pub fn new(cells: &'a Vec<Vec<Cell>>) -> CellsIter<'a> {
        CellsIter { index: 0, cells: cells }
    }
}

impl<'a> Iterator for CellsIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        let rows_count = self.cells.len();
        let cols_count = self.cells[0].len();

        if self.index >= (rows_count * cols_count) {
            return None;
        }

        let mut x = self.index;
        let mut y = 0;
        self.index += 1;

        while x >= cols_count {
            x -= cols_count;
            y += 1;
        }

        Some(self.cells[y].get(x).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn it_returns_none_with_no_cells() {
        let cells: Vec<Vec<Cell>> = vec!(vec!());
        let mut iter = CellsIter::new(&cells);
        assert!(iter.next().is_none());
    }

    #[test]
    fn it_works_with_one_row() {
        let mut screen = Screen::new(1, 3);
        screen.cells[0][0].ch = 'a';
        screen.cells[0][1].ch = 'b';
        screen.cells[0][2].ch = 'c';

        let mut iter = CellsIter::new(&screen.cells);
        assert_eq!(iter.next().unwrap().ch, 'a');
        assert_eq!(iter.next().unwrap().ch, 'b');
        assert_eq!(iter.next().unwrap().ch, 'c');
    }

    #[test]
    fn it_works_with_one_col() {
        let mut screen = Screen::new(3, 1);
        screen.cells[0][0].ch = 'a';
        screen.cells[1][0].ch = 'b';
        screen.cells[2][0].ch = 'c';

        let mut iter = CellsIter::new(&screen.cells);
        assert_eq!(iter.next().unwrap().ch, 'a');
        assert_eq!(iter.next().unwrap().ch, 'b');
        assert_eq!(iter.next().unwrap().ch, 'c');
    }

    #[test]
    fn it_works_with_rect() {
        let mut screen = Screen::new(2, 2);
        screen.cells[0][0].ch = 'a';
        screen.cells[0][1].ch = 'b';
        screen.cells[1][0].ch = 'c';
        screen.cells[1][1].ch = 'd';

        let mut iter = CellsIter::new(&screen.cells);
        assert_eq!(iter.next().unwrap().ch, 'a');
        assert_eq!(iter.next().unwrap().ch, 'b');
        assert_eq!(iter.next().unwrap().ch, 'c');
        assert_eq!(iter.next().unwrap().ch, 'd');
    }

    #[test]
    fn it_returns_none_after_end() {
        let mut screen = Screen::new(1, 1);
        screen.cells[0][0].ch = 'a';

        let mut iter = CellsIter::new(&screen.cells);
        assert_eq!(iter.next().unwrap().ch, 'a');
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }
}
