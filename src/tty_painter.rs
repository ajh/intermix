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
    pub fn draw_cells<F: Write>(&self, cells: &Vec<ScreenCell>, io: &mut F) {
        for cell in cells { self.draw_cell(cell, io) }
    }

    fn draw_cell<F: Write>(&self, cell: &ScreenCell, io: &mut F) {
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

        if cell.pos.row != self.pen.pos.row || cell.pos.col != self.pen.pos.col {
            //trace!("moving cursor to row {:?} col {:?}", cell.pos.row, cell.pos.col);
            let ti = term::terminfo::TermInfo::from_env().unwrap();
            let cmd = ti.strings.get("cup").unwrap();
            let params = [ term::terminfo::parm::Param::Number(cell.pos.row as i16),
                           term::terminfo::parm::Param::Number(cell.pos.col as i16) ];
            let s = term::terminfo::parm::expand(&cmd, &params, &mut term::terminfo::parm::Variables::new()).unwrap();
            io.write_all(&s).unwrap();
        }

        io.write_all(&cell.chars_as_utf8_bytes()).ok().expect("failed to write");
        if cell.width > 1 { trace!("cell has width > 1 {:?}", cell) }
    }
}
