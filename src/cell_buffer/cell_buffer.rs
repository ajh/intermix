use vterm_sys::{Size, Pos, Rect, RectAssist, RectPositions};
use super::cell::Cell;
use std::ops::{Index, IndexMut};

pub type CellAndPosIter<'a> = ::std::iter::Zip<::std::slice::Iter<'a, Cell>, RectPositions<'a>>;
pub type MutCellAndPosIter<'a> = ::std::iter::Zip<::std::slice::IterMut<'a, Cell>, RectPositions<'a>>;

pub struct CellBuffer {
    rect: Rect,
    cells: Vec<Cell>,
}

impl CellBuffer {
    pub fn new(size: Size) -> CellBuffer {
        CellBuffer {
            cells: vec![Cell::new(); size.width * size.height],
            rect: Rect::new(Pos::new(0,0), size),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> MutCellAndPosIter {
        self.cells.iter_mut().zip(self.rect.positions())
    }

    pub fn iter<'a>(&'a self) -> CellAndPosIter {
        self.cells.iter().zip(self.rect.positions())
    }

    pub fn resize(&mut self) {
        panic!("not implemented")
    }

    // don't need this yet
    //fn index_to_pos(index: usize, width: usize) -> Pos {
        //Pos::new(index % width, index / width)
    //}

    fn pos_to_index(pos: &Pos, width: usize) -> usize {
        (pos.y * width) + pos.x
    }
}

impl Index<Pos> for CellBuffer {
    type Output = Cell;

    fn index<'a>(&'a self, pos: Pos) -> &'a Cell {
        &self.cells[CellBuffer::pos_to_index(&pos, self.rect.size.width)]
    }
}

impl IndexMut<Pos> for CellBuffer {
    fn index_mut<'a>(&'a mut self, pos: Pos) -> &'a mut Cell {
        self.cells.index_mut(CellBuffer::pos_to_index(&pos, self.rect.size.width))
    }
}
