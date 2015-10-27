//use super::Cell;

//pub struct CellsIter<'a> {
    //index: usize,
    //cells: &'a Vec<Vec<Cell>>,
//}

//impl<'a> CellsIter<'a> {
    //fn new(cells: &'a Vec<Vec<Cell>>) -> CellsIter<'a> {
        //CellsIter { index: 0, cells: cells }
    //}
//}

//impl<'a> Iterator for CellsIter<'a> {
    //type Item = Cell;

    //fn next(&mut self) -> Option<Cell> {
        //let rows_count = self.cells.len();
        //let cols_count = self.cells[0].len();

        //if self.index >= (rows_count * cols_count) {
            //return None;
        //}

        //let mut i = self.index;
        //let mut y = 0;

        //while i > cols_count {
            //i -= cols_count;
            //y += 1;
        //}

        //self.index += 1;
        ////Some(self.cells[y][i])
        //None
    //}
//}

//#[cfg(test)]
//mod tests {
    //use super::*;
    //use super::super::*;

    //#[test]
    //fn it_returns_none_with_no_cells() {
        //let cells: Vec<Vec<Cell>> = vec!(vec!());
        //let mut iter = CellsIter::new(&cells);
        //assert!(iter.next().is_none());
    //}

    //#[test]
    //fn it_works_with_one_row() {
        //let mut screen = Screen::new(1, 3);
        //screen.cells[0][0].ch = 'a';
        //screen.cells[0][1].ch = 'b';
        //screen.cells[0][2].ch = 'c';

        //let cells: Vec<Vec<Cell>> = vec!(vec!());
        //let mut iter = CellsIter::new(&cells);
        //assert_eq!(iter.next().unwrap().ch, 'a');
        //assert_eq!(iter.next().unwrap().ch, 'b');
        //assert_eq!(iter.next().unwrap().ch, 'c');
    //}

    //#[test]
    //fn it_works_with_one_col() {
    //}

    //#[test]
    //fn it_works_with_rect() {
    //}

    //#[test]
    //fn it_returns_none_after_end() {
    //}
//}
