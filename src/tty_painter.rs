extern crate libvterm_sys;
extern crate term;

use libvterm_sys::*;
use std::io::prelude::*;

#[derive(Debug, Default)]
pub struct Pen {
    attrs: ScreenCellAttr,
    fg: Color,
    bg: Color,
    pos: Pos,
}

#[derive(Debug, Default)]
pub struct TtyPainter {
    // the physical state of the tty that is being painted
    pen: Pen,
}

impl TtyPainter {
    /// TODO: make this take &self not &mut self because changing the pen is just an implementation
    /// detail. Use Cell or whatever for interior mutability.
    pub fn draw_cells<F: Write>(&mut self, cells: &Vec<ScreenCell>, io: &mut F, offset: &Pos) {
        // turn off cursor
        let ti = term::terminfo::TermInfo::from_env().unwrap();
        let cmd = ti.strings.get("civis").unwrap();
        let s = term::terminfo::parm::expand(&cmd, &[], &mut term::terminfo::parm::Variables::new()).unwrap();
        io.write_all(&s).unwrap();

        for cell in cells { self.draw_cell(cell, io, offset) }

        let ti = term::terminfo::TermInfo::from_env().unwrap();
        let cmd = ti.strings.get("cvvis").unwrap();
        let s = term::terminfo::parm::expand(&cmd, &[], &mut term::terminfo::parm::Variables::new()).unwrap();
        io.write_all(&s).unwrap();

        io.flush().unwrap();
    }

    fn draw_cell<F: Write>(&mut self, cell: &ScreenCell, io: &mut F, offset: &Pos) {
        let mut sgrs: Vec<isize> = vec!();

        if !self.pen.attrs.bold && cell.attrs.bold                    { sgrs.push(1); }
        if self.pen.attrs.bold && !cell.attrs.bold                    { sgrs.push(22); }
        if self.pen.attrs.underline == 0 && cell.attrs.underline != 0 { sgrs.push(4); }
        if self.pen.attrs.underline != 0 && cell.attrs.underline == 0 { sgrs.push(24); }
        if !self.pen.attrs.italic && cell.attrs.italic                { sgrs.push(3); }
        if self.pen.attrs.italic && !cell.attrs.italic                { sgrs.push(23); }
        if !self.pen.attrs.blink && cell.attrs.blink                  { sgrs.push(5); }
        if self.pen.attrs.blink && !cell.attrs.blink                  { sgrs.push(25); }
        if !self.pen.attrs.reverse && cell.attrs.reverse              { sgrs.push(7); }
        if self.pen.attrs.reverse && !cell.attrs.reverse              { sgrs.push(27); }
        if !self.pen.attrs.strike && cell.attrs.strike                { sgrs.push(9); }
        if self.pen.attrs.strike && !cell.attrs.strike                { sgrs.push(29); }
        if self.pen.attrs.font == 0 && cell.attrs.font != 0           { sgrs.push(10 + cell.attrs.font as isize); }
        if self.pen.attrs.font != 0 && cell.attrs.font == 0           { sgrs.push(10); }

        //if self.pen.fg.red   != cell.fg.red   ||
           //self.pen.fg.green != cell.fg.green ||
           //self.pen.fg.blue  != cell.fg.blue {
            ////trace!("changing fg color: prev {} {} {} cell {} {} {}",
                   ////self.pen.fg.red,
                   ////self.pen.fg.green,
                   ////self.pen.fg.blue,
                   ////self.pen.bg.red,
                   ////self.pen.bg.green,
                   ////self.pen.bg.blue);
            //let index = color_to_index(state, &cell.fg);
            //if index == -1 { sgrs.push(39); }
            //else if index < 8 { sgrs.push(30 + index); }
            //else if index < 16 { sgrs.push(90 + (index - 8)); }
            //else {
                //sgrs.push(38);
                //sgrs.push(5 | (1<<31));
                //sgrs.push(index | (1<<31));
            //}
        //}

        //if self.pen.bg.red   != cell.bg.red   ||
           //self.pen.bg.green != cell.bg.green ||
           //self.pen.bg.blue  != cell.bg.blue {
            //let index = color_to_index(state, &cell.bg);
            //if index == -1 { sgrs.push(49); }
            //else if index < 8 { sgrs.push(40 + index); }
            //else if index < 16 { sgrs.push(100 + (index - 8)); }
            //else {
                //sgrs.push(48);
                //sgrs.push(5 | (1<<31));
                //sgrs.push(index | (1<<31));
            //}
        //}

        if sgrs.len() != 0 {
            let mut sgr = "\x1b[".to_string();
            for (i, val) in sgrs.iter().enumerate() {
                let bare_val = val & !(1<<31);
                if i == 0 {
                    sgr.push_str(&format!("{}", bare_val));
                }
                else if val & (1<<31) != 0 {
                    sgr.push_str(&format!(":{}", bare_val));
                }
                else {
                    sgr.push_str(&format!(";{}", bare_val));
                }
            }
            sgr.push_str("m");
            io.write_all(sgr.as_bytes()).unwrap();
        }

        let pos = Pos {
            row: cell.pos.row + offset.row,
            col: cell.pos.col + offset.col,
        };

        if pos.row != self.pen.pos.row || pos.col != self.pen.pos.col {
            //trace!("moving cursor to row {:?} col {:?}", cell.pos.row, cell.pos.col);
            let ti = term::terminfo::TermInfo::from_env().unwrap();
            let cmd = ti.strings.get("cup").unwrap();
            let params = [ term::terminfo::parm::Param::Number(pos.row as i16),
                           term::terminfo::parm::Param::Number(pos.col as i16) ];
            let s = term::terminfo::parm::expand(&cmd, &params, &mut term::terminfo::parm::Variables::new()).unwrap();
            io.write_all(&s).unwrap();
        }

        io.write_all(&cell.chars_as_utf8_bytes()).ok().expect("failed to write");
        if cell.width > 1 { trace!("cell has width > 1 {:?}", cell) }
    }
}

mod tests {
    extern crate libvterm_sys;

    use libvterm_sys::*;
    use super::*;
    use std::io::prelude::*;
    use std::io;

    struct CaptureIO {
        pub cursor: usize,
        pub bytes:  Vec<u8>
    }

    impl CaptureIO {
        fn new() -> CaptureIO {
            CaptureIO { cursor: 0, bytes: vec!() }
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
            }
            else {
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
            }
            else {
                cell = None;
            }

            cell
        }
    }

    #[test]
    fn it_can_paint_something_simple() {
        let mut painter: TtyPainter = Default::default();
        let mut io = CaptureIO::new();

        // create some ScreenCells with stuff
        let cells = vec!(
            ScreenCell { pos: Pos { row: 0, col: 0 }, chars: vec!('h'), width: 1, .. Default::default() },
            ScreenCell { pos: Pos { row: 0, col: 1 }, chars: vec!('i'), width: 1, .. Default::default() },
            ScreenCell { pos: Pos { row: 1, col: 0 }, chars: vec!('h'), width: 1, .. Default::default() },
            ScreenCell { pos: Pos { row: 1, col: 1 }, chars: vec!('o'), width: 1, .. Default::default() },
        );

        // paint them into libvterm
        painter.draw_cells(&cells, &mut io, &Pos { row: 0, col: 0 });

        let mut vterm = VTerm::new(ScreenSize{ rows: 2, cols: 2 });
        vterm.write(&io.bytes);

        // get all damaged cells
        let iterator = CellsIterator::new(&vterm);
        let actual_cells: Vec<ScreenCell> = iterator.collect();

        // compare actual with expected
        let expected: Vec<char> = cells.iter().flat_map(|c| c.chars.clone()).collect();
        let actual: Vec<char>   = actual_cells.iter().flat_map(|c| c.chars.clone()).collect();
        assert_eq!(expected, actual);
    }
}
