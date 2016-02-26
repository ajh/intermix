use std::io::prelude::*;
use std::io::BufWriter;
use term;
use vterm_sys::{self, ScreenSize, Pos, ColorPalette, ScreenCell, Rect};

// TODO:
//
// * [ ] use terminfo for all writing
// * [ ] set cursor visiblity in reset method
// * [ ] audit other things missed in reset method
// * [ ] DRY sgr code
//
// # Idea to DRY sgr code
//
// 1. Have constant defaults defined for attrs and stuff like BOLD = false;
// 2. all values in Pen are Option
// 3. pull out an sgr function that takes a pen, and writes bytes for Some(_) values
// 4. reset adds Some(_) values for all attrs and writes sgr for everything
// 5. draw cell builds up a pen with Some(_) values that are different than the current pen, then
//    passes it to the sgr fn. Then updates the current pen.

#[derive(Debug, Clone)]
pub struct Pen {
    bg: ColorPalette,
    blink: bool,
    bold: bool,
    dhl: u8,
    dwl: bool,
    fg: ColorPalette,
    font: u8,
    is_visible: bool,
    italic: bool,
    pos: Pos,
    reverse: bool,
    strike: bool,
    underline: u8,
}

impl Default for Pen {
    fn default() -> Pen {
        Pen {
            bg: 0,
            blink: false,
            bold: false,
            dhl: 0,
            dwl: false,
            fg: 7,
            font: 0,
            is_visible: false,
            italic: false,
            pos: Pos { row: 0, col: 0 },
            reverse: false,
            strike: false,
            underline: 0,
        }
    }
}

#[derive(Debug)]
pub struct TtyPainter<F: Write + Send> {
    // the physical state of the tty that is being painted
    pen: Pen,
    io: BufWriter<F>,
    size: ScreenSize,
}

impl<F: Write + Send> TtyPainter<F> {
    pub fn new(io: F, size: ScreenSize) -> TtyPainter<F> {
        TtyPainter {
            pen: Default::default(),
            io: BufWriter::new(io),
            size: size,
        }
    }

    /// Sync physical terminal state with pen state
    ///
    /// TODO: DRY with draw_cell
    pub fn reset(&mut self) {
        trace!("reset");
        let mut sgrs: Vec<usize> = vec![];

        if self.pen.bold {
            sgrs.push(1);
        } else {
            sgrs.push(22);
        }

        if self.pen.underline != 0 {
            sgrs.push(4);
        } else {
            sgrs.push(24);
        }

        if self.pen.italic {
            sgrs.push(3);
        } else {
            sgrs.push(23);
        }

        if self.pen.blink {
            sgrs.push(5);
        } else {
            sgrs.push(25);
        }

        if self.pen.reverse {
            sgrs.push(7);
        } else {
            sgrs.push(27);
        }

        if self.pen.strike {
            sgrs.push(9);
        } else {
            sgrs.push(29);
        }

        if self.pen.font != 0 {
            sgrs.push(10 + self.pen.font as usize);
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

    // Draw the cells in the list starting at the given position
    pub fn draw_cells(&mut self, cells: &Vec<ScreenCell>, rect: &Rect) {
        trace!("draw_cells rect={:?}", rect);
        //let restore_pen = self.pen.clone();

        //if self.pen.is_visible {
            //self.pen.is_visible = false;
            //// make cursor invisible
            //let ti = term::terminfo::TermInfo::from_env().unwrap();
            //let cmd = ti.strings.get("civis").unwrap();
            //let s = term::terminfo::parm::expand(&cmd,
                                                 //&[],
                                                 //&mut term::terminfo::parm::Variables::new())
                        //.unwrap();
            //self.io.write_all(&s).unwrap();
        //}

        let mut pos = Pos {
            row: rect.start_row,
            col: rect.start_col,
        };

        for (i, cell) in cells.iter().enumerate() {
            let width = rect.end_col - rect.start_col;
            pos.col = rect.start_col + (i % width);
            pos.row = rect.start_row + (i as f32 / width as f32).floor() as usize;

            if pos.col >= self.size.cols || pos.row >= self.size.rows {
                // Not sure this is the right thing to do. How do terminals handle wrapping?
                continue;
            }

            if self.pen.pos != pos {
                self.move_cursor(&pos);
            }
            self.draw_cell(cell);
        }

        //if restore_pen.is_visible {
            //self.pen.is_visible = true;
            //// make it visible again
            //let ti = term::terminfo::TermInfo::from_env().unwrap();
            //let cmd = ti.strings.get("cnorm").unwrap();
            //let s = term::terminfo::parm::expand(&cmd,
                                                 //&[],
                                                 //&mut term::terminfo::parm::Variables::new())
                        //.unwrap();
            //self.io.write_all(&s).unwrap();
        //}

        //if restore_pen.pos != self.pen.pos {
            //self.pen.pos = restore_pen.pos.clone();

            //let ti = term::terminfo::TermInfo::from_env().unwrap();
            //let cmd = ti.strings.get("cup").unwrap();
            //let params = [term::terminfo::parm::Param::Number(self.pen.pos.row as i32),
                          //term::terminfo::parm::Param::Number(self.pen.pos.col as i32)];
            //let s = term::terminfo::parm::expand(&cmd,
                                                 //&params,
                                                 //&mut term::terminfo::parm::Variables::new())
                        //.unwrap();
            //self.io.write_all(&s).unwrap();
        //}

        self.io.flush().unwrap();
    }

    fn draw_cell(&mut self, cell: &ScreenCell) {
        //trace!("draw_cell cell={:?} pen.pos={:?}", cell, self.pen.pos);
        let mut sgrs: Vec<usize> = vec![];

        if self.pen.bold != cell.attrs.bold {
            if cell.attrs.bold {
                sgrs.push(1);
            } else {
                sgrs.push(22);
            }
            self.pen.bold = cell.attrs.bold;
        }

        if self.pen.underline != cell.attrs.underline {
            if cell.attrs.underline != 0 {
                sgrs.push(4);
            } else {
                sgrs.push(24);
            }
            self.pen.underline = cell.attrs.underline;
        }

        if self.pen.italic != cell.attrs.italic {
            if cell.attrs.italic {
                sgrs.push(3);
            } else {
                sgrs.push(23);
            }
            self.pen.italic = cell.attrs.italic;
        }

        if self.pen.blink != cell.attrs.blink {
            if cell.attrs.blink {
                sgrs.push(5);
            } else {
                sgrs.push(25);
            }
            self.pen.blink = cell.attrs.blink;
        }

        if self.pen.reverse != cell.attrs.reverse {
            if cell.attrs.reverse {
                sgrs.push(7);
            } else {
                sgrs.push(27);
            }
            self.pen.reverse = cell.attrs.reverse;
        }

        if self.pen.strike != cell.attrs.strike {
            if cell.attrs.strike {
                sgrs.push(9);
            } else {
                sgrs.push(29);
            }
            self.pen.strike = cell.attrs.strike;
        }

        if self.pen.font != cell.attrs.font {
            if cell.attrs.font != 0 {
                sgrs.push(10 + cell.attrs.font as usize);
            } else {
                sgrs.push(10);
            }
            self.pen.font = cell.attrs.font;
        }

        if self.pen.fg != cell.fg_palette {
            if cell.fg_palette < 8 {
                sgrs.push(30 + cell.fg_palette as usize);
            } else if cell.fg_palette < 16 {
                sgrs.push(90 + (cell.fg_palette as usize - 8));
            } else {
                sgrs.push(38);
                sgrs.push(5 | (1 << 31));
                sgrs.push(cell.fg_palette as usize | (1 << 31));
            }
            self.pen.fg = cell.fg_palette;
        }

        if self.pen.bg != cell.bg_palette {
            if cell.bg_palette < 8 {
                sgrs.push(40 + cell.bg_palette as usize);
            } else if cell.bg_palette < 16 {
                sgrs.push(100 + (cell.bg_palette as usize - 8));
            } else {
                sgrs.push(48);
                sgrs.push(5 | (1 << 31));
                sgrs.push(cell.bg_palette as usize | (1 << 31));
            }

            self.pen.bg = cell.bg_palette;
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

        let bytes = cell.chars_as_utf8_bytes();

        // See tmux's tty.c:1155 function `tty_cell`
        if bytes.len() > 0 {
            //trace!("writing {:?}", &bytes);
            self.io.write_all(&bytes).ok().expect("failed to write");
        } else {
            //trace!("just writing space");
            // like tmux's tty_repeat_space
            self.io.write_all(&[b'\x20']).ok().expect("failed to write"); // space
        }

        if self.pen.pos.col + 1 < self.size.cols {
            self.pen.pos.col += 1;
        } else if self.pen.pos.row + 1 < self.size.rows {
            self.pen.pos.col = 0;
            self.pen.pos.row += 1;
        } else {
            warn!("cursor is beyond screen size? What is supposed to happen here?");
            // for now just wedge it into the bottom right corner
            self.pen.pos.col = self.size.cols - 1;
            self.pen.pos.row = self.size.rows - 1;
        }

        if cell.width > 1 {
            warn!("cell has width > 1 {:?}, but acting on this information isnt implemented",
                  cell)
        }
    }

    /// TODO: take a offset from the pane
    pub fn move_cursor(&mut self, pos: &Pos) {
        trace!("move_cursor pos={:?}", pos);
        let ti = term::terminfo::TermInfo::from_env().unwrap();

        if *pos != self.pen.pos {
            self.pen.pos = pos.clone();

            let cmd = ti.strings.get("cup").unwrap();
            let params = [term::terminfo::parm::Param::Number(self.pen.pos.row as i32),
                          term::terminfo::parm::Param::Number(self.pen.pos.col as i32)];
            let s = term::terminfo::parm::expand(&cmd,
                                                 &params,
                                                 &mut term::terminfo::parm::Variables::new())
                        .unwrap();
            self.io.write_all(&s).unwrap();
        }
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
