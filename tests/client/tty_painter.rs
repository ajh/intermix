// TODO:
//
// * move these to `tests` directory, but first need to split out lib and bin
// * Move helper stuff to a helper file
// * more tests
use std::io::prelude::*;
use std::io;
use std::sync::{Arc, Mutex};
use libintermix::client::tty_painter::*;
use vterm_sys::*;
use super::support::capture_io::*;

struct CellsIterator<'a> {
    pos: Pos,
    vterm: &'a VTerm,
    size: ScreenSize,
}

impl<'a> CellsIterator<'a> {
    pub fn new(vterm: &'a VTerm) -> CellsIterator<'a> {
        CellsIterator {
            pos: Default::default(),
            vterm: vterm,
            size: vterm.get_size(),
        }
    }

    fn advance(&mut self) {
        if ((self.pos.col + 1) as u16) < self.size.cols {
            self.pos.col += 1;
        } else {
            self.pos.col = 0;
            self.pos.row += 1;
        }
    }
}

impl<'a> Iterator for CellsIterator<'a> {
    type Item = ScreenCell;

    fn next(&mut self) -> Option<ScreenCell> {
        let cell: Option<ScreenCell>;

        if (self.pos.col as u16) < self.size.cols && (self.pos.row as u16) < self.size.rows {
            cell = Some(self.vterm.screen.get_cell(&self.pos));
            self.advance();
        } else {
            cell = None;
        }

        cell
    }
}

struct ScreenCellBuilder {
    size: ScreenSize,
    chars: Vec<Vec<char>>,
}

impl ScreenCellBuilder {
    fn new(size: ScreenSize) -> ScreenCellBuilder {
        ScreenCellBuilder {
            size: size,
            chars: vec!(vec!()),
        }
    }

    fn chars(&mut self, chars: Vec<Vec<char>>) -> &mut ScreenCellBuilder {
        if chars.len() != self.size.rows as usize {
            panic!("wrong number of rows. Expected {} got {}", self.size.rows, chars.len());
        }
        //TODO: check cols too
        self.chars = chars.clone();

        self
    }

    fn finalize(&self) -> Vec<ScreenCell> {
        let mut cells: Vec<ScreenCell> = vec![];
        for row in 0..self.size.rows {
            for col in 0..self.size.cols {
                let mut cell = ScreenCell { pos: Pos { row: row as i16, col: col as i16 }, .. Default::default() };

                let is_char_defined = self.chars.len() > row as usize && self.chars[row as usize].len() > col as usize;
                let ch = if is_char_defined { self.chars[row as usize][col as usize] } else { ' ' };
                cell.chars.push(ch);

                cells.push(cell);
            }
        }

        cells
    }
}

fn drawn_cells(bytes: &Vec<u8>, size: ScreenSize) -> Vec<ScreenCell> {
    let mut vterm = VTerm::new(size);
    vterm.state.set_default_colors(Color { red: 230, green: 230, blue: 230 },
                                   Color { red: 5, green: 5, blue: 5 });
    vterm.state.reset(true);
    vterm.write(bytes);

    let iterator = CellsIterator::new(&vterm);
    iterator.collect()
}

#[test]
fn it_correctly_draws_empty_screen() {
    let (mut io, bytes) = CaptureIO::new();
    let mut painter = TtyPainter::new(io);

    let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 2, cols: 2 }).finalize();

    // paint them into libvterm
    painter.draw_cells(&cells, &Pos { row: 0, col: 0 });

    assert_eq!(cells, drawn_cells(&bytes.lock().unwrap(), ScreenSize { cols: 2, rows: 2}));
}

#[test]
fn it_correctly_draws_position_of_chars() {
    let (mut io, bytes) = CaptureIO::new();
    let mut painter = TtyPainter::new(io);

    let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 3, cols: 3 })
        .chars(vec![vec!['y', ' ', ' '],
                    vec![' ', 'o', ' '],
                    vec![' ', ' ', '!']])
        .finalize();

    painter.draw_cells(&cells, &Pos { row: 0, col: 0 });

    assert_eq!(cells, drawn_cells(&bytes.lock().unwrap(), ScreenSize { cols: 3, rows: 3}));
}

#[test]
fn it_draws_consecutive_chars() {
    let (mut io, bytes) = CaptureIO::new();
    let mut painter = TtyPainter::new(io);

    let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 3, cols: 3 })
        .chars(vec![vec!['y', 'o', '!'],
                    vec![' ', ' ', ' '],
                    vec![' ', ' ', ' ']])
        .finalize();

    painter.draw_cells(&cells, &Pos { row: 0, col: 0 });

    assert_eq!(cells, drawn_cells(&bytes.lock().unwrap(), ScreenSize { cols: 3, rows: 3}));
}

#[test]
fn it_draws_chars_with_gaps() {
    let (mut io, bytes) = CaptureIO::new();
    let mut painter = TtyPainter::new(io);

    let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 3, cols: 3 })
        .chars(vec![vec!['a', ' ', 'b'],
                    vec!['c', ' ', 'd'],
                    vec!['e', ' ', 'f']])
        .finalize();

    painter.draw_cells(&cells, &Pos { row: 0, col: 0 });

    assert_eq!(cells, drawn_cells(&bytes.lock().unwrap(), ScreenSize { cols: 3, rows: 3}));
}

#[test]
fn it_draws_vertical_chars() {
    let (mut io, bytes) = CaptureIO::new();
    let mut painter = TtyPainter::new(io);

    let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 3, cols: 3 })
        .chars(vec![vec![' ', 'y', ' '],
                    vec![' ', 'o', ' '],
                    vec![' ', '!', ' ']])
        .finalize();

    painter.draw_cells(&cells, &Pos { row: 0, col: 0 });

    assert_eq!(cells, drawn_cells(&bytes.lock().unwrap(), ScreenSize { cols: 3, rows: 3}));
}

#[test]
fn it_clears_chars() {
    let (mut io, bytes) = CaptureIO::new();
    let mut painter = TtyPainter::new(io);

    let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 2, cols: 2 })
        .chars(vec![vec!['a', 'b'],
                    vec!['c', 'd']])
        .finalize();
    painter.draw_cells(&cells, &Pos { row: 0, col: 0 });

    let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 2, cols: 2 })
        .chars(vec![vec!['h', ' '],
                    vec![' ', 'i']])
        .finalize();
    painter.draw_cells(&cells, &Pos { row: 0, col: 0 });

    assert_eq!(cells, drawn_cells(&bytes.lock().unwrap(), ScreenSize { cols: 2, rows: 2}));
}
