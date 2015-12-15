use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;
use super::{Size, Pos, Widgets};

/// A widget is something that can be drawn to the screen.
///
/// Its size and position is calculated at run time.
#[derive(Debug, Clone)]
pub struct Widget {
    pub id: String,
    pub program_id: String,
    pub fill: char,
    pub size: Size,
    actual_size: Size,
    pub pos: Pos,
}

impl Widget {
    pub fn new(fill: char, size: Size) -> Widget {
        Widget {
            fill: fill,
            size: size,
            actual_size: Default::default(),
            pos: Pos { row: 0, col: 0 },
            id: "".to_string(),
            program_id: "".to_string(),
        }
    }

    pub fn new_with_program_id(program_id: String, size: Size) -> Widget {
        Widget {
            fill: ' ',
            size: size,
            actual_size: Default::default(),
            pos: Pos { row: 0, col: 0 },
            id: "".to_string(),
            program_id: program_id,
        }
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }
}
