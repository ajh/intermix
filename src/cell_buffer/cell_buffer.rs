use vterm_sys::{Size, Pos};
use super::cell::Cell;
use std::ops::{Index, IndexMut};

pub struct CellBuffer {
    size: Size,
    cells: Vec<Cell>,
}

impl CellBuffer {
    pub fn new(size: Size) -> CellBuffer {
        CellBuffer {
            size: size,
            cells: vec![Cell::new(); size.width * size.height],
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
}

impl Index<Pos> for CellBuffer {
    type Output = Cell;

    fn index<'a>(&'a self, pos: Pos) -> &'a Cell {
        &self.cells[(pos.y * self.size.width) + pos.x]
    }
}

impl IndexMut<Pos> for CellBuffer {
    fn index_mut<'a>(&'a mut self, pos: Pos) -> &'a mut Cell {
        self.cells.index_mut((pos.y * self.size.width) + pos.x)
    }
}
