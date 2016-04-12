use term::terminfo::{parm, TermInfo};
use vterm_sys::{Size, Pos, ColorPalette, ScreenCell};

/// An object that tracks the state of the physical screen and collects desired changes to it. Can
/// help make the desired changes reality by flushing them as a terminfo byte string.
pub struct Pen {
    pub bg: ColorPalette,
    pub blink: bool,
    pub bold: bool,
    pub dhl: u8,
    pub dwl: bool,
    pub fg: ColorPalette,
    pub font: u8,
    pub visible: bool,
    pub italic: bool,
    pub pos: Pos,
    pub reverse: bool,
    pub strike: bool,
    pub underline: u8,

    disp_bg: Option<ColorPalette>,
    disp_blink: Option<bool>,
    disp_bold: Option<bool>,
    #[allow(dead_code)]
    disp_dhl: Option<u8>,
    #[allow(dead_code)]
    disp_dwl: Option<bool>,
    disp_fg: Option<ColorPalette>,
    #[allow(dead_code)]
    disp_font: Option<u8>,
    disp_visible: Option<bool>,
    disp_italic: Option<bool>,
    disp_pos: Option<Pos>,
    disp_reverse: Option<bool>,
    #[allow(dead_code)]
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
            visible: true,
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
            disp_visible: None,
            disp_italic: None,
            disp_pos: None,
            disp_reverse: None,
            disp_strike: None,
            disp_underline: None,
        }
    }
}

impl Pen {
    pub fn new() -> Pen {
        Default::default()
    }

    /// flush any accumulated changes and return a byte vec with the terminfo sequences
    pub fn flush(&mut self, terminfo: &TermInfo, vars: &mut parm::Variables) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        if (self.disp_visible.is_none() || *self.disp_visible.as_ref().unwrap()) && !self.visible {
            self.disp_visible = Some(self.visible);
            self.apply_cap("civis", &mut bytes, terminfo, vars, &vec![]);
        }

        if self.disp_pos.is_none() || *self.disp_pos.as_ref().unwrap() != self.pos {
            self.disp_pos = Some(self.pos);

            let params = vec![ parm::Param::Number(self.pos.y as i32),
                 parm::Param::Number(self.pos.x as i32)];
            self.apply_cap("cup", &mut bytes, terminfo, vars, &params);
        }

        // may have to reset here because there's no way to turn off stuff like bold through the
        // term crate?
        let need_reset_bold = *self.disp_bold.as_ref().unwrap_or(&true) && !self.bold;
        let need_reset_blink = *self.disp_blink.as_ref().unwrap_or(&true) && !self.blink;
        let need_reset_reverse = *self.disp_reverse.as_ref().unwrap_or(&true) && !self.reverse;

        if need_reset_bold || need_reset_blink || need_reset_reverse {
            self.sgr0(&mut bytes, terminfo, vars);
        }

        if self.disp_bold.is_none() || *self.disp_bold.as_ref().unwrap() != self.bold {
            self.disp_bold = Some(self.bold);

            if self.bold {
                self.apply_cap("bold", &mut bytes, terminfo, vars, &vec![]);
            }
        }

        if self.disp_underline.is_none() || *self.disp_underline.as_ref().unwrap() != self.underline {
            self.disp_underline = Some(self.underline);

            if self.underline != 0 {
                self.apply_cap("smul", &mut bytes, terminfo, vars, &vec![]);
            } else {
                self.apply_cap("rmul", &mut bytes, terminfo, vars, &vec![]);
            };
        }

        if self.is_cap_supported("sitm", terminfo) && self.is_cap_supported("ritm", terminfo) {
            if self.disp_italic.is_none() || *self.disp_italic.as_ref().unwrap() != self.italic {
                self.disp_italic = Some(self.italic);

                if self.italic {
                    self.apply_cap("sitm", &mut bytes, terminfo, vars, &vec![]);
                } else {
                    self.apply_cap("ritm", &mut bytes, terminfo, vars, &vec![]);
                };
            }
        }

        if self.disp_blink.is_none() || *self.disp_blink.as_ref().unwrap() != self.blink {
            self.disp_blink = Some(self.blink);

            if self.blink {
                self.apply_cap("blink", &mut bytes, terminfo, vars, &vec![]);
            }
        }

        if self.disp_reverse.is_none() || *self.disp_reverse.as_ref().unwrap() != self.reverse {
            self.disp_reverse = Some(self.reverse);

            if self.reverse {
                self.apply_cap("rev", &mut bytes, terminfo, vars, &vec![]);
            }
        }

        if self.disp_fg.is_none() || *self.disp_fg.as_ref().unwrap() != self.fg {
            self.disp_fg = Some(self.fg);

            let params = vec![ parm::Param::Number(self.fg as i32) ];
            self.apply_cap("setaf", &mut bytes, terminfo, vars, &params);
        }

        if self.disp_bg.is_none() || *self.disp_bg.as_ref().unwrap() != self.bg {
            self.disp_bg = Some(self.bg);

            let params = vec![ parm::Param::Number(self.bg as i32) ];
            self.apply_cap("setab", &mut bytes, terminfo, vars, &params);
        }

        if (self.disp_visible.is_none() || !*self.disp_visible.as_ref().unwrap()) && self.visible {
            self.disp_visible = Some(self.visible);
            self.apply_cap("cnorm", &mut bytes, terminfo, vars, &vec![]);
        }

        bytes
    }

    fn sgr0(&mut self, bytes: &mut Vec<u8>, terminfo: &TermInfo, vars: &mut parm::Variables) {
        self.disp_blink = Some(false);
        self.disp_bold = Some(false);
        self.disp_reverse = Some(false);
        self.disp_underline = Some(0);
        self.apply_cap("sgr0", bytes, terminfo, vars, &vec![]);
    }

    /// Updates many pen attributes from the given cell's attributes
    pub fn update_attrs_from_cell(&mut self, cell: &ScreenCell) {
        self.bg = cell.bg_palette;
        self.blink = cell.attrs.blink;
        self.bold = cell.attrs.bold;
        self.fg = cell.fg_palette;
        self.font = cell.attrs.font;
        self.italic = cell.attrs.italic;
        self.reverse = cell.attrs.reverse;
        self.strike = cell.attrs.strike;
        self.underline = cell.attrs.underline;
    }

    /// Let the pen know that the physical display advanced the cursor one position
    pub fn notify_of_advanced_pos(&mut self, screen_size: &Size) {
        if self.pos.x + 1 < screen_size.width {
            self.pos.x += 1;
        } else if self.pos.y + 1 < screen_size.height {
            self.pos.x = 0;
            self.pos.y += 1;
        } else {
            warn!("cursor is beyond screen size? What is supposed to happen here?");
            self.pos.x = screen_size.width - 1;
            self.pos.y = screen_size.height - 1;
            self.disp_pos = None;
        }
    }

    /// appends the cap sequence to the given bytes vec
    fn apply_cap(&self, cap: &str, bytes: &mut Vec<u8>, terminfo: &TermInfo, vars: &mut parm::Variables, params: &Vec<parm::Param>) {
        let cmd = terminfo.strings.get(cap).unwrap();
        let mut sequence = parm::expand(&cmd, params.as_slice(), vars).unwrap();
        bytes.append(&mut sequence);
    }

    fn is_cap_supported(&self, cap: &str, terminfo: &TermInfo) -> bool {
        terminfo.strings.get(cap).is_some()
    }
}
