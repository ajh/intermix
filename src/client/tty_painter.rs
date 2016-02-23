use std::io::prelude::*;
use std::io::BufWriter;
use term;
use vterm_sys::*;

/// TODO:
///
/// * Maybe all this state should be wrapped in Option to know when the state is unknown as it is
/// initially.
#[derive(Debug, Default, Clone)]
pub struct Pen {
    attrs: ScreenCellAttr,
    fg: ColorPalette,
    bg: ColorPalette,
    pos: Pos,
    is_visible: bool,
}

#[derive(Debug)]
pub struct TtyPainter<F: Write + Send> {
    // the physical state of the tty that is being painted
    pen: Pen,
    io: BufWriter<F>,
}

impl<F: Write + Send> TtyPainter<F> {
    pub fn new(io: F) -> TtyPainter<F> {
        TtyPainter {
            pen: Default::default(),
            io: BufWriter::new(io),
        }
    }

    /// Sync physical terminal state with pen state
    ///
    /// TODO: DRY with draw_cell
    pub fn reset(&mut self) {
        let mut sgrs: Vec<isize> = vec![];

        if self.pen.attrs.bold {
            sgrs.push(1);
        } else {
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
            self.io.write_all(sgr.as_bytes()).unwrap();
        }

        self.io.flush().unwrap();
    }

    /// TODO: make this take &self not &mut self because changing the pen is just an implementation
    /// detail. Use Cell or whatever for interior mutability.
    pub fn draw_cells(&mut self, cells: &Vec<ScreenCell>, offset: &Pos) {
        // trace!("draw_cells {:?}", cells);
        let restore_pen = self.pen.clone();

        if self.pen.is_visible {
            self.pen.is_visible = false;
            // make cursor invisible
            let ti = term::terminfo::TermInfo::from_env().unwrap();
            let cmd = ti.strings.get("civis").unwrap();
            let s = term::terminfo::parm::expand(&cmd,
                                                 &[],
                                                 &mut term::terminfo::parm::Variables::new())
                        .unwrap();
            self.io.write_all(&s).unwrap();
        }

        for cell in cells {
            self.draw_cell(cell, offset)
        }

        if restore_pen.is_visible {
            self.pen.is_visible = true;
            // make it visible again
            let ti = term::terminfo::TermInfo::from_env().unwrap();
            let cmd = ti.strings.get("cnorm").unwrap();
            let s = term::terminfo::parm::expand(&cmd,
                                                 &[],
                                                 &mut term::terminfo::parm::Variables::new())
                        .unwrap();
            self.io.write_all(&s).unwrap();
        }

        if restore_pen.pos != self.pen.pos {
            self.pen.pos = restore_pen.pos.clone();

            let ti = term::terminfo::TermInfo::from_env().unwrap();
            let cmd = ti.strings.get("cup").unwrap();
            let params = [term::terminfo::parm::Param::Number(self.pen.pos.row as i32),
                          term::terminfo::parm::Param::Number(self.pen.pos.col as i32)];
            let s = term::terminfo::parm::expand(&cmd,
                                                 &params,
                                                 &mut term::terminfo::parm::Variables::new())
                        .unwrap();
            self.io.write_all(&s).unwrap();
        }

        self.io.flush().unwrap();
    }

    fn draw_cell(&mut self, cell: &ScreenCell, offset: &Pos) {
        // trace!("draw_cell cell={:?}", cell);
        let mut sgrs: Vec<isize> = vec![];

        if self.pen.attrs.bold != cell.attrs.bold {
            if cell.attrs.bold {
                sgrs.push(1);
            } else {
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

        if self.pen.fg != cell.fg_palette {
            if cell.fg_palette < 8 {
                sgrs.push(30 + cell.fg_palette as isize);
            } else if cell.fg_palette < 16 {
                sgrs.push(90 + (cell.fg_palette as isize - 8));
            } else {
                sgrs.push(38);
                sgrs.push(5 | (1 << 31));
                sgrs.push(cell.fg_palette as isize | (1 << 31));
            }
        }

        if self.pen.bg != cell.bg_palette {
            if cell.bg_palette < 8 {
                sgrs.push(40 + cell.bg_palette as isize);
            } else if cell.bg_palette < 16 {
                sgrs.push(100 + (cell.bg_palette as isize - 8));
            } else {
                sgrs.push(48);
                sgrs.push(5 | (1 << 31));
                sgrs.push(cell.bg_palette as isize | (1 << 31));
            }
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
            self.io.write_all(sgr.as_bytes()).unwrap();
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
            let params = [term::terminfo::parm::Param::Number(pos.row as i32),
                          term::terminfo::parm::Param::Number(pos.col as i32)];
            let s = term::terminfo::parm::expand(&cmd,
                                                 &params,
                                                 &mut term::terminfo::parm::Variables::new())
                        .unwrap();
            self.io.write_all(&s).unwrap();
        }

        let bytes = cell.chars_as_utf8_bytes();

        // See tmux's tty.c:1155 function `tty_cell`
        if bytes.len() > 0 {
            // trace!("writing {:?}", &bytes);
            self.io.write_all(&bytes).ok().expect("failed to write");
        } else {
            // trace!("just writing space");
            // like tmux's tty_repeat_space
            self.io.write_all(&[b'\x20']).ok().expect("failed to write"); // space
        }

        // This is wrong. Really I need to know the user's screen size to know when wrap.
        self.pen.pos.col += 1;

        if cell.width > 1 {
            trace!("cell has width > 1 {:?}", cell)
        }
    }

    /// TODO: take a offset from the pane
    pub fn move_cursor(&mut self, pos: Pos, is_visible: bool) {
        let ti = term::terminfo::TermInfo::from_env().unwrap();

        if pos != self.pen.pos {
            // trace!("move_cursor to {:?}", pos);
            self.pen.pos = pos;

            let cmd = ti.strings.get("cup").unwrap();
            let params = [term::terminfo::parm::Param::Number(self.pen.pos.row as i32),
                          term::terminfo::parm::Param::Number(self.pen.pos.col as i32)];
            let s = term::terminfo::parm::expand(&cmd,
                                                 &params,
                                                 &mut term::terminfo::parm::Variables::new())
                        .unwrap();
            self.io.write_all(&s).unwrap();
        }

        if is_visible != self.pen.is_visible {
            // trace!("move_cursor visible? {:?}", is_visible);
            self.pen.is_visible = is_visible;

            let cap = if self.pen.is_visible {
                "cnorm"
            } else {
                "civis"
            };
            let cmd = ti.strings.get(cap).unwrap();
            let s = term::terminfo::parm::expand(&cmd,
                                                 &[],
                                                 &mut term::terminfo::parm::Variables::new())
                        .unwrap();
            self.io.write_all(&s).unwrap();
        }

        self.io.flush().unwrap();
    }

    /// Implemented like tmux's tty_redraw_region
    ///
    /// If the pane is the full width of the physical terminal this can be optimized by using
    /// scroll regions, but that isn't implemented.
    ///
    /// Tmux also has an optimization where it'll no-op this if the effected region is >= 50% of
    /// the pane, but will instead schedule a "pane redraw". That is also not implemented.
    /// (&mut self, scroll_region_size: &ScreenSize, scroll_region_pos: &Pos) {
    pub fn insert_line(&mut self, _: &ScreenSize, _: &Pos) {
        // I'd like to iterate through all the cells in the pane. Can I get access to this?
    }

    // pub fn delete_line<F: Write>(&mut self, pane: &Pane, io: &mut F) {
    // /deleteLine: CSR(top, bottom) + CUP(y, 0) + DL(1) + CSR(0, height)
    // }
}
