use std::io::prelude::*;
use std::io::BufWriter;
use term::terminfo::{parm, TermInfo};
use vterm_sys::{self, Size, Pos, ColorPalette, ScreenCell, Rect, RectAssist};

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
    pub bg: ColorPalette,
    pub blink: bool,
    pub bold: bool,
    pub dhl: u8,
    pub dwl: bool,
    pub fg: ColorPalette,
    pub font: u8,
    pub is_visible: bool,
    pub italic: bool,
    pub pos: Pos,
    pub reverse: bool,
    pub strike: bool,
    pub underline: u8,

    disp_bg: Option<ColorPalette>,
    disp_blink: Option<bool>,
    disp_bold: Option<bool>,
    disp_dhl: Option<u8>,
    disp_dwl: Option<bool>,
    disp_fg: Option<ColorPalette>,
    disp_font: Option<u8>,
    disp_is_visible: Option<bool>,
    disp_italic: Option<bool>,
    disp_pos: Option<Pos>,
    disp_reverse: Option<bool>,
    disp_strike: Option<bool>,
    disp_underline: Option<u8>,
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
            is_visible: true,
            italic: false,
            pos: Pos::new(0,0),
            reverse: false,
            strike: false,
            underline: 0,
            disp_bg: None,
            disp_blink: None,
            disp_bold: None,
            disp_dhl: None,
            disp_dwl: None,
            disp_fg: None,
            disp_font: None,
            disp_is_visible: None,
            disp_italic: None,
            disp_pos: None,
            disp_reverse: None,
            disp_strike: None,
            disp_underline: None,
        }
    }
}

impl Pen {
    pub fn flush<W: Write>(&mut self, io: &mut W) {
        let ti = TermInfo::from_env().unwrap();
        let mut vars = parm::Variables::new();
        let mut params: Vec<parm::Param> = vec![];

        if self.disp_pos.is_none() || *self.disp_pos.as_ref().unwrap() != self.pos {
            params.push(parm::Param::Number(self.pos.y as i32));
            params.push(parm::Param::Number(self.pos.x as i32));
            Pen::write("cup", io, &ti, &mut vars, &mut params);
        }

        // may have to reset here because there's no way to turn off stuff like bold through the
        // term crate?
        let need_reset_bold = *self.disp_bold.as_ref().unwrap_or(&true) && !self.bold;
        let need_reset_blink = *self.disp_blink.as_ref().unwrap_or(&true) && !self.blink;
        let need_reset_reverse = *self.disp_reverse.as_ref().unwrap_or(&true) && !self.reverse;

        if need_reset_bold || need_reset_blink || need_reset_reverse {
            self.reset(io, &ti, &mut vars);
        }

        if self.disp_bold.is_none() || *self.disp_bold.as_ref().unwrap() != self.bold {
            self.disp_bold = Some(self.bold);

            if self.bold {
                Pen::write("bold", io, &ti, &mut vars, &mut params);
            }
        }

        if self.disp_underline.is_none() || *self.disp_underline.as_ref().unwrap() != self.underline {
            self.disp_underline = Some(self.underline);

            if self.underline != 0 {
                Pen::write("smul", io, &ti, &mut vars, &mut params);
            } else {
                Pen::write("rmul", io, &ti, &mut vars, &mut params);
            };
        }

        if self.disp_italic.is_none() || *self.disp_italic.as_ref().unwrap() != self.italic {
            self.disp_italic = Some(self.italic);

            if self.italic {
                Pen::write("sitm", io, &ti, &mut vars, &mut params);
            } else {
                Pen::write("ritm", io, &ti, &mut vars, &mut params);
            };
        }

        if self.disp_blink.is_none() || *self.disp_blink.as_ref().unwrap() != self.blink {
            self.disp_blink = Some(self.blink);

            if self.blink {
                Pen::write("blink", io, &ti, &mut vars, &mut params);
            }
        }

        if self.disp_reverse.is_none() || *self.disp_reverse.as_ref().unwrap() != self.reverse {
            self.disp_reverse = Some(self.reverse);

            if self.reverse {
                Pen::write("rev", io, &ti, &mut vars, &mut params);
            }
        }

        if self.disp_fg.is_none() || *self.disp_fg.as_ref().unwrap() != self.fg {
            self.disp_fg = Some(self.fg);

            params.push(parm::Param::Number(self.fg as i32));
            Pen::write("setaf", io, &ti, &mut vars, &mut params);
        }

        if self.disp_bg.is_none() || *self.disp_bg.as_ref().unwrap() != self.bg {
            self.disp_bg = Some(self.bg);

            params.push(parm::Param::Number(self.bg as i32));
            Pen::write("setab", io, &ti, &mut vars, &mut params);
        }
    }

    fn reset<W: Write>(&mut self, io: &mut W, ti: &TermInfo, vars: &mut parm::Variables) {
        self.disp_bg = Some(0);
        self.disp_blink = Some(false);
        self.disp_bold = Some(false);
        self.disp_dhl = Some(0);
        self.disp_dwl = Some(false);
        self.disp_fg = Some(7);
        self.disp_font = Some(0);
        self.disp_is_visible = Some(true);
        self.disp_italic = Some(false);
        self.disp_reverse = Some(false);
        self.disp_strike = Some(false);
        self.disp_underline = Some(0);
        Pen::write("sgr", io, ti, vars, &mut vec![parm::Param::Number(0)]);
    }

    fn write<W: Write>(cap: &str, io: &mut W, ti: &TermInfo, vars: &mut parm::Variables, params: &mut Vec<parm::Param>) {
        let bytes = apply_cap(cap, ti, vars, params);
        io.write_all(&bytes).unwrap();
        params.clear();
    }
}

#[derive(Debug)]
pub struct TtyPainter<F: Write + Send> {
    // the physical state of the tty that is being painted
    pen: Pen,
    io: BufWriter<F>,
    size: Size,
}

impl<F: Write + Send> TtyPainter<F> {
    pub fn new(io: F, size: Size) -> TtyPainter<F> {
        TtyPainter {
            pen: Default::default(),
            io: BufWriter::new(io),
            size: size,
        }
    }

    // Draw the cells in the list starting at the given position
    pub fn draw_cells(&mut self, cells: &Vec<ScreenCell>, rect: &Rect) {
        trace!("draw_cells rect={:?}", rect);

        for (cell, pos) in cells.iter().zip(rect.positions()) {
            if pos.x >= self.size.width || pos.y >= self.size.height {
                // Not sure this is the right thing to do. How do terminals handle wrapping?
                warn!("skipping draw of cell because its position is outside of our rect");
                continue;
            }

            self.pen.pos = pos;
            self.draw_cell(cell);
        }

        self.io.flush().unwrap();
    }

    // This method assumes the pen is already at the correct position
    fn draw_cell(&mut self, cell: &ScreenCell) {
        //trace!("draw_cell cell={:?} pen.pos={:?}", cell, self.pen.pos);

        self.pen.bg = cell.bg_palette;
        self.pen.blink = cell.attrs.blink;
        self.pen.bold = cell.attrs.bold;
        self.pen.fg = cell.fg_palette;
        self.pen.font = cell.attrs.font;
        self.pen.italic = cell.attrs.italic;
        self.pen.reverse = cell.attrs.reverse;
        self.pen.strike = cell.attrs.strike;
        self.pen.underline = cell.attrs.underline;

        self.pen.flush(&mut self.io);

        // See tmux's tty.c:1155 function `tty_cell`
        if cell.chars.len() > 0 {
            self.io.write_all(&cell.chars).ok().expect("failed to write");
        } else {
            // like tmux's tty_repeat_space
            self.io.write_all(&[b'\x20']).ok().expect("failed to write"); // space
        }

        if self.pen.pos.x + 1 < self.size.width {
            self.pen.pos.x += 1;
        } else if self.pen.pos.y + 1 < self.size.height {
            self.pen.pos.x = 0;
            self.pen.pos.y += 1;
        } else {
            warn!("cursor is beyond screen size? What is supposed to happen here?");
            // for now just wedge it into the bottom right corner
            self.pen.pos.x = self.size.width - 1;
            self.pen.pos.y = self.size.height - 1;
        }

        if cell.width > 1 {
            warn!("cell has width > 1 {:?}, but acting on this information isnt implemented",
                  cell)
        }
    }

    /// TODO: take a offset from the pane
    pub fn move_cursor(&mut self, pos: &Pos) {
        trace!("move_cursor pos={:?}", pos);
        self.pen.pos = pos.clone();
    }

    /// Implemented like tmux's tty_redraw_region
    ///
    /// If the pane is the full width of the physical terminal this can be optimized by using
    /// scroll regions, but that isn't implemented.
    ///
    /// Tmux also has an optimization where it'll no-op this if the effected region is >= 50% of
    /// the pane, but will instead schedule a "pane redraw". That is also not implemented.
    /// (&mut self, scroll_region_size: &Size, scroll_region_pos: &Pos) {
    pub fn insert_line(&mut self, _: &Size, _: &Pos) {
        // I'd like to iterate through all the cells in the pane. Can I get access to this?
    }

    pub fn flush(&mut self) {
        self.pen.flush(&mut self.io);
    }

    pub fn clear(&mut self) {
        apply_cap("clear", )
    }

    // pub fn delete_line<F: Write>(&mut self, pane: &Pane, io: &mut F) {
    // /deleteLine: CSR(top, bottom) + CUP(y, 0) + DL(1) + CSR(0, height)
    // }
}

fn apply_cap(cap: &str, terminfo: &TermInfo, vars: &mut parm::Variables, params: &Vec<parm::Param>) -> Vec<u8> {
    let cmd = terminfo.strings.get(cap).unwrap();
    parm::expand(&cmd, params.as_slice(), vars).unwrap()
}
