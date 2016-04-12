// Cell
// Cell::new()
// c.char
// c.bold
// c.etc_etc
// c.is_dirty
//
// CellBuf
// CellBuf::new(size)
// cell_buf.resize(size, options)
// cell_buf.cells() -> CellsIterator
// cell_buf.mut_cells() -> MutCellsIterator
// cell_buf.index(pos: &Position) -> &Cell
// cell_buf.index_mut(pos: &Position) -> &mut Cell
//
// # program damage
//
//     // get rect for program
//     for pos in rect {
//       Client.cell_buf[pos] = blah
//     }
//
//     self.painter.paint_cell_buf(&self.cell_buf)
//
// # layout damage
//
//     let rect = Rect::new(Pos::new(0,0), self.size.clone())
//
//     // draw borders and stuff
//     // draw programs from their own cell_buf
//     self.painter.paint_cell_buf(&self.cell_buf)
//
// # tty_painter
//
//     pub fn paint_cell_buf(cell_buf: &mut CellBuf) {
//       for cell in cell_buf.mut_cells().filter(|c| c.is_dirty) {
//         // draw whatever is in that cell
//         cell.is_dirty = false
//       }
//     }
//
// # Discussion
//
// This allows:
// * transformations
// * resizing the client terminal
// * refreshing the screen
// * refreshing the screen after changing the layout
// * changing the size of a program

mod cell;
mod cell_buffer;
