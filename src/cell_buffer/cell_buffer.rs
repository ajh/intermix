use vterm_sys::{Size, Pos, Rect, RectAssist};
use super::cell::Cell;
use std::ops::{Index, IndexMut};

pub struct CellBuffer {
    size: Size,
    cells: Vec<Cell>,
}

impl CellBuffer {
    pub fn new(size: Size) -> CellBuffer {
        let mut cells = Vec::with_capacity(size.width * size.height);
        for pos in Rect::new(Pos::new(0,0), size.clone()).positions() {
            cells.push(Cell::new(pos));
        }

        CellBuffer {
            size: size,
            cells: cells,
        }
    }

    pub fn iter(&self) -> ::std::slice::Iter<Cell> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> ::std::slice::IterMut<Cell> {
        self.cells.iter_mut()
    }

    pub fn resize(&mut self) {
        panic!("not implemented")
    }

    #[allow(dead_code)]
    fn index_to_pos(index: usize, width: usize) -> Pos {
        Pos::new(index % width, index / width)
    }

    fn pos_to_index(pos: &Pos, width: usize) -> usize {
        (pos.y * width) + pos.x
    }
}

impl Index<Pos> for CellBuffer {
    type Output = Cell;

    fn index<'a>(&'a self, pos: Pos) -> &'a Cell {
        &self.cells[CellBuffer::pos_to_index(&pos, self.size.width)]
    }
}

impl IndexMut<Pos> for CellBuffer {
    fn index_mut<'a>(&'a mut self, pos: Pos) -> &'a mut Cell {
        self.cells.index_mut(CellBuffer::pos_to_index(&pos, self.size.width))
    }
}
