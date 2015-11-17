extern crate libvterm_sys;
extern crate term;

use libvterm_sys::*;
use std::io::prelude::*;
use ::pane::Pane;

#[derive(Debug, Default)]
pub struct Pen {
    attrs: ScreenCellAttr,
    fg: Color,
    bg: Color,
    pos: Pos,
    is_visible: bool,
}

#[derive(Debug, Default)]
pub struct TtyPainter {
    // the physical state of the tty that is being painted
    pen: Pen,
}

impl TtyPainter {
    /// Sync physical terminal state with pen state
    ///
    /// TODO: DRY with draw_cell
    pub fn reset<F: Write>(&mut self, io: &mut F) {
        let mut sgrs: Vec<isize> = vec![];

        if self.pen.attrs.bold {
            sgrs.push(1);
        } else  {
            sgrs.push(22);
        }

        if self.pen.attrs.underline != 0 {
            sgrs.push(4);
        } else {
            sgrs.push(24);
        }

        if self.pen.attrs.italic {
            sgrs.push(3);
        } else {
            sgrs.push(23);
        }

        if self.pen.attrs.blink {
            sgrs.push(5);
        } else {
            sgrs.push(25);
        }

        if self.pen.attrs.reverse {
            sgrs.push(7);
        } else {
            sgrs.push(27);
        }

        if self.pen.attrs.strike {
            sgrs.push(9);
        } else {
            sgrs.push(29);
        }

        if self.pen.attrs.font != 0 {
            sgrs.push(10 + self.pen.attrs.font as isize);
        } else {
            sgrs.push(10);
        }

        if sgrs.len() != 0 {
            let mut sgr = "\x1b[".to_string();
            for (i, val) in sgrs.iter().enumerate() {
                let bare_val = val & !(1 << 31);
                if i == 0 {
                    sgr.push_str(&format!("{}", bare_val));
                } else if val & (1 << 31) != 0 {
                    sgr.push_str(&format!(":{}", bare_val));
                } else {
                    sgr.push_str(&format!(";{}", bare_val));
                }
            }
            sgr.push_str("m");
            io.write_all(sgr.as_bytes()).unwrap();
        }

        io.flush().unwrap();
    }

    /// TODO: make this take &self not &mut self because changing the pen is just an implementation
    /// detail. Use Cell or whatever for interior mutability.
    pub fn draw_cells<F: Write>(&mut self, cells: &Vec<ScreenCell>, io: &mut F, offset: &Pos) {
        // make cursor invisible
        let ti = term::terminfo::TermInfo::from_env().unwrap();
        let cmd = ti.strings.get("civis").unwrap();
        let s = term::terminfo::parm::expand(&cmd,
                                             &[],
                                             &mut term::terminfo::parm::Variables::new())
                    .unwrap();
        io.write_all(&s).unwrap();
        self.pen.is_visible = false;

        for cell in cells {
            self.draw_cell(cell, io, offset)
        }

        io.flush().unwrap();
    }

    fn draw_cell<F: Write>(&mut self, cell: &ScreenCell, io: &mut F, offset: &Pos) {
        let mut sgrs: Vec<isize> = vec![];

        if self.pen.attrs.bold != cell.attrs.bold {
            if cell.attrs.bold {
                sgrs.push(1);
            } else  {
                sgrs.push(22);
            }
            self.pen.attrs.bold = cell.attrs.bold;
        }

        if self.pen.attrs.underline != cell.attrs.underline {
            if cell.attrs.underline != 0 {
                sgrs.push(4);
            } else {
                sgrs.push(24);
            }
            self.pen.attrs.underline = cell.attrs.underline;
        }

        if self.pen.attrs.italic != cell.attrs.italic {
            if cell.attrs.italic {
                sgrs.push(3);
            } else {
                sgrs.push(23);
            }
            self.pen.attrs.italic = cell.attrs.italic;
        }

        if self.pen.attrs.blink != cell.attrs.blink {
            if cell.attrs.blink {
                sgrs.push(5);
            } else {
                sgrs.push(25);
            }
            self.pen.attrs.blink = cell.attrs.blink;
        }

        if self.pen.attrs.reverse != cell.attrs.reverse {
            if cell.attrs.reverse {
                sgrs.push(7);
            } else {
                sgrs.push(27);
            }
            self.pen.attrs.reverse = cell.attrs.reverse;
        }

        if self.pen.attrs.strike != cell.attrs.strike {
            if cell.attrs.strike {
                sgrs.push(9);
            } else {
                sgrs.push(29);
            }
            self.pen.attrs.strike = cell.attrs.strike;
        }

        if self.pen.attrs.font != cell.attrs.font {
            if cell.attrs.font != 0 {
                sgrs.push(10 + cell.attrs.font as isize);
            } else {
                sgrs.push(10);
            }
            self.pen.attrs.font = cell.attrs.font;
        }

        // if self.pen.fg.red   != cell.fg.red   ||
        // self.pen.fg.green != cell.fg.green ||
        // self.pen.fg.blue  != cell.fg.blue {
        // /trace!("changing fg color: prev {} {} {} cell {} {} {}",
        // /self.pen.fg.red,
        // /self.pen.fg.green,
        // /self.pen.fg.blue,
        // /self.pen.bg.red,
        // /self.pen.bg.green,
        // /self.pen.bg.blue);
        // let index = color_to_index(state, &cell.fg);
        // if index == -1 { sgrs.push(39); }
        // else if index < 8 { sgrs.push(30 + index); }
        // else if index < 16 { sgrs.push(90 + (index - 8)); }
        // else {
        // sgrs.push(38);
        // sgrs.push(5 | (1<<31));
        // sgrs.push(index | (1<<31));
        // }
        // }

        // if self.pen.bg.red   != cell.bg.red   ||
        // self.pen.bg.green != cell.bg.green ||
        // self.pen.bg.blue  != cell.bg.blue {
        // let index = color_to_index(state, &cell.bg);
        // if index == -1 { sgrs.push(49); }
        // else if index < 8 { sgrs.push(40 + index); }
        // else if index < 16 { sgrs.push(100 + (index - 8)); }
        // else {
        // sgrs.push(48);
        // sgrs.push(5 | (1<<31));
        // sgrs.push(index | (1<<31));
        // }
        // }

        if sgrs.len() != 0 {
            let mut sgr = "\x1b[".to_string();
            for (i, val) in sgrs.iter().enumerate() {
                let bare_val = val & !(1 << 31);
                if i == 0 {
                    sgr.push_str(&format!("{}", bare_val));
                } else if val & (1 << 31) != 0 {
                    sgr.push_str(&format!(":{}", bare_val));
                } else {
                    sgr.push_str(&format!(";{}", bare_val));
                }
            }
            sgr.push_str("m");
            io.write_all(sgr.as_bytes()).unwrap();
        }

        // apply offset
        let pos = Pos {
            row: cell.pos.row + offset.row,
            col: cell.pos.col + offset.col,
        };

         if pos.row != self.pen.pos.row || pos.col != self.pen.pos.col {
            // trace!("moving cursor to row {:?} col {:?}", cell.pos.row, cell.pos.col);
            let ti = term::terminfo::TermInfo::from_env().unwrap();
            let cmd = ti.strings.get("cup").unwrap();
            let params = [term::terminfo::parm::Param::Number(pos.row as i16),
                          term::terminfo::parm::Param::Number(pos.col as i16)];
            let s = term::terminfo::parm::expand(&cmd,
                                                 &params,
                                                 &mut term::terminfo::parm::Variables::new())
                        .unwrap();
            io.write_all(&s).unwrap();
        }

        let bytes = cell.chars_as_utf8_bytes();

        // See tmux's tty.c:1155 function `tty_cell`
        if bytes.len() > 0 {
            io.write_all(&bytes).ok().expect("failed to write");
        } else {
            // like tmux's tty_repeat_space
            io.write_all(&[b'\x20']).ok().expect("failed to write"); // space
        }

        // This is wrong. Really I need to know the user's screen size to know when wrap.
        self.pen.pos.col += 1;

        if cell.width > 1 {
            trace!("cell has width > 1 {:?}", cell)
        }
    }

    /// TODO: take a offset from the pane
    pub fn move_cursor<F: Write>(&mut self, pos: Pos, is_visible: bool, io: &mut F) {
        let ti = term::terminfo::TermInfo::from_env().unwrap();

        if pos != self.pen.pos {
            //trace!("move_cursor to {:?}", pos);
            self.pen.pos = pos;

            let cmd = ti.strings.get("cup").unwrap();
            let params = [term::terminfo::parm::Param::Number(self.pen.pos.row as i16),
            term::terminfo::parm::Param::Number(self.pen.pos.col as i16)];
            let s = term::terminfo::parm::expand(&cmd,
                                                 &params,
                                                 &mut term::terminfo::parm::Variables::new())
                .unwrap();
            io.write_all(&s).unwrap();
        }

        if is_visible != self.pen.is_visible {
            //trace!("move_cursor visible? {:?}", is_visible);
            self.pen.is_visible = is_visible;

            let cap = if self.pen.is_visible { "cnorm" } else { "civis" };
            let cmd = ti.strings.get(cap).unwrap();
            let s = term::terminfo::parm::expand(&cmd,
                                                 &[],
                                                 &mut term::terminfo::parm::Variables::new())
                        .unwrap();
            io.write_all(&s).unwrap();
        }

        io.flush().unwrap();
    }

    /// Implemented like tmux's tty_redraw_region
    ///
    /// If the pane is the full width of the physical terminal this can be optimized by using
    /// scroll regions, but that isn't implemented.
    ///
    /// Tmux also has an optimization where it'll no-op this if the effected region is >= 50% of
    /// the pane, but will instead schedule a "pane redraw". That is also not implemented.
    pub fn insert_line<F: Write>(&mut self, scroll_region_size: &ScreenSize, scroll_region_pos: &Pos, io: &mut F) {
        // I'd like to iterate through all the cells in the pane. Can I get access to this?
    }

    //pub fn delete_line<F: Write>(&mut self, pane: &Pane, io: &mut F) {
        ////deleteLine: CSR(top, bottom) + CUP(y, 0) + DL(1) + CSR(0, height)
    //}
}

// TODO:
//
// * move these to `tests` directory, but first need to split out lib and bin
// * Move helper stuff to a helper file
// * more tests
mod tests {
    extern crate libvterm_sys;

    use libvterm_sys::*;
    use super::*;
    use std::io::prelude::*;
    use std::io;

    struct CaptureIO {
        pub cursor: usize,
        pub bytes: Vec<u8>,
    }

    impl CaptureIO {
        fn new() -> CaptureIO {
            CaptureIO {
                cursor: 0,
                bytes: vec![],
            }
        }

        fn slice(&self) -> &[u8] {
            &self.bytes
        }
    }

    impl Write for CaptureIO {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            for byte in buf {
                self.bytes.push(*byte);
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

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

    fn drawn_cells(io: &CaptureIO, size: ScreenSize) -> Vec<ScreenCell> {
        let mut vterm = VTerm::new(size);
        vterm.state.set_default_colors(Color { red: 230, green: 230, blue: 230 },
                                       Color { red: 5, green: 5, blue: 5 });
        vterm.state.reset(true);
        vterm.write(&io.bytes);

        let iterator = CellsIterator::new(&vterm);
        iterator.collect()
    }

    #[test]
    fn it_correctly_draws_empty_screen() {
        let mut painter: TtyPainter = Default::default();
        let mut io = CaptureIO::new();

        let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 2, cols: 2 }).finalize();

        // paint them into libvterm
        painter.draw_cells(&cells, &mut io, &Pos { row: 0, col: 0 });

        assert_eq!(cells, drawn_cells(&io, ScreenSize { cols: 2, rows: 2}));
    }

    #[test]
    fn it_correctly_draws_position_of_chars() {
        let mut painter: TtyPainter = Default::default();
        let mut io = CaptureIO::new();

        let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 3, cols: 3 })
            .chars(vec![vec!['y', ' ', ' '],
                        vec![' ', 'o', ' '],
                        vec![' ', ' ', '!']])
            .finalize();

        painter.draw_cells(&cells, &mut io, &Pos { row: 0, col: 0 });

        assert_eq!(cells, drawn_cells(&io, ScreenSize { cols: 3, rows: 3}));
    }

    #[test]
    fn it_draws_consecutive_chars() {
        let mut painter: TtyPainter = Default::default();
        let mut io = CaptureIO::new();

        let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 3, cols: 3 })
            .chars(vec![vec!['y', 'o', '!'],
                        vec![' ', ' ', ' '],
                        vec![' ', ' ', ' ']])
            .finalize();

        painter.draw_cells(&cells, &mut io, &Pos { row: 0, col: 0 });

        assert_eq!(cells, drawn_cells(&io, ScreenSize { cols: 3, rows: 3}));
    }

    #[test]
    fn it_draws_chars_with_gaps() {
        let mut painter: TtyPainter = Default::default();
        let mut io = CaptureIO::new();

        let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 3, cols: 3 })
            .chars(vec![vec!['a', ' ', 'b'],
                        vec!['c', ' ', 'd'],
                        vec!['e', ' ', 'f']])
            .finalize();

        painter.draw_cells(&cells, &mut io, &Pos { row: 0, col: 0 });

        assert_eq!(cells, drawn_cells(&io, ScreenSize { cols: 3, rows: 3}));
    }

    #[test]
    fn it_draws_vertical_chars() {
        let mut painter: TtyPainter = Default::default();
        let mut io = CaptureIO::new();

        let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 3, cols: 3 })
            .chars(vec![vec![' ', 'y', ' '],
                        vec![' ', 'o', ' '],
                        vec![' ', '!', ' ']])
            .finalize();

        painter.draw_cells(&cells, &mut io, &Pos { row: 0, col: 0 });

        assert_eq!(cells, drawn_cells(&io, ScreenSize { cols: 3, rows: 3}));
    }

    #[test]
    fn it_clears_chars() {
        let mut painter: TtyPainter = Default::default();
        let mut io = CaptureIO::new();

        let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 2, cols: 2 })
            .chars(vec![vec!['a', 'b'],
                        vec!['c', 'd']])
            .finalize();
        painter.draw_cells(&cells, &mut io, &Pos { row: 0, col: 0 });

        let cells: Vec<ScreenCell> = ScreenCellBuilder::new(ScreenSize { rows: 2, cols: 2 })
            .chars(vec![vec!['h', ' '],
                        vec![' ', 'i']])
            .finalize();
        painter.draw_cells(&cells, &mut io, &Pos { row: 0, col: 0 });

        assert_eq!(cells, drawn_cells(&io, ScreenSize { cols: 2, rows: 2}));
    }
}
