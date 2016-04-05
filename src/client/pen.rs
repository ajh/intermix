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

    terminfo: TermInfo,
    vars: parm::Variables,
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
            terminfo: TermInfo::from_name("xterm").unwrap(),
            vars: parm::Variables::new(),
        }
    }
}

impl Pen {
    pub fn new(terminfo: TermInfo) -> Pen {
        Pen { terminfo: terminfo, .. Default::default() }
    }

    pub fn flush<W: Write>(&mut self, io: &mut W) {
        let mut vars = parm::Variables::new();
        let mut params: Vec<parm::Param> = vec![];

        if self.disp_pos.is_none() || *self.disp_pos.as_ref().unwrap() != self.pos {
            self.disp_pos = Some(self.pos);

            params.push(parm::Param::Number(self.pos.y as i32));
            params.push(parm::Param::Number(self.pos.x as i32));
            Pen::write("cup", io, &self.terminfo, &mut vars, &mut params);
        }

        // may have to reset here because there's no way to turn off stuff like bold through the
        // term crate?
        let need_reset_bold = *self.disp_bold.as_ref().unwrap_or(&true) && !self.bold;
        let need_reset_blink = *self.disp_blink.as_ref().unwrap_or(&true) && !self.blink;
        let need_reset_reverse = *self.disp_reverse.as_ref().unwrap_or(&true) && !self.reverse;

        if need_reset_bold || need_reset_blink || need_reset_reverse {
            self.reset(io, &mut vars);
        }

        if self.disp_bold.is_none() || *self.disp_bold.as_ref().unwrap() != self.bold {
            self.disp_bold = Some(self.bold);

            if self.bold {
                Pen::write("bold", io, &self.terminfo, &mut vars, &mut params);
            }
        }

        if self.disp_underline.is_none() || *self.disp_underline.as_ref().unwrap() != self.underline {
            self.disp_underline = Some(self.underline);

            if self.underline != 0 {
                Pen::write("smul", io, &self.terminfo, &mut vars, &mut params);
            } else {
                Pen::write("rmul", io, &self.terminfo, &mut vars, &mut params);
            };
        }

        if self.disp_italic.is_none() || *self.disp_italic.as_ref().unwrap() != self.italic {
            self.disp_italic = Some(self.italic);

            if self.italic {
                Pen::write("sitm", io, &self.terminfo, &mut vars, &mut params);
            } else {
                Pen::write("ritm", io, &self.terminfo, &mut vars, &mut params);
            };
        }

        if self.disp_blink.is_none() || *self.disp_blink.as_ref().unwrap() != self.blink {
            self.disp_blink = Some(self.blink);

            if self.blink {
                Pen::write("blink", io, &self.terminfo, &mut vars, &mut params);
            }
        }

        if self.disp_reverse.is_none() || *self.disp_reverse.as_ref().unwrap() != self.reverse {
            self.disp_reverse = Some(self.reverse);

            if self.reverse {
                Pen::write("rev", io, &self.terminfo, &mut vars, &mut params);
            }
        }

        if self.disp_fg.is_none() || *self.disp_fg.as_ref().unwrap() != self.fg {
            self.disp_fg = Some(self.fg);

            params.push(parm::Param::Number(self.fg as i32));
            Pen::write("setaf", io, &self.terminfo, &mut vars, &mut params);
        }

        if self.disp_bg.is_none() || *self.disp_bg.as_ref().unwrap() != self.bg {
            self.disp_bg = Some(self.bg);

            params.push(parm::Param::Number(self.bg as i32));
            Pen::write("setab", io, &self.terminfo, &mut vars, &mut params);
        }
    }

    fn reset<W: Write>(&mut self, io: &mut W, vars: &mut parm::Variables) {
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
        Pen::write("sgr0", io, &self.terminfo, vars, &mut vec![]);
    }

    fn write<W: Write>(cap: &str, io: &mut W, ti: &TermInfo, vars: &mut parm::Variables, params: &mut Vec<parm::Param>) {
        let bytes = apply_cap(cap, ti, vars, params);
        io.write_all(&bytes).unwrap();
        params.clear();
    }
}

impl ::std::fmt::Debug for Pen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "debug not implemented")
    }
}

fn apply_cap(cap: &str, terminfo: &TermInfo, vars: &mut parm::Variables, params: &Vec<parm::Param>) -> Vec<u8> {
    let cmd = terminfo.strings.get(cap).unwrap();
    parm::expand(&cmd, params.as_slice(), vars).unwrap()
}
