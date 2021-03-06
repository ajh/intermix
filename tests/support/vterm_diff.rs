use vterm_sys::*;
use std::io::prelude::*;

pub struct VTermDiff<'a, 'b> {
    expected: &'a VTerm,
    actual: &'b VTerm,
}

const BORDER: char = '·';

impl<'a, 'b> VTermDiff<'a, 'b> {
    pub fn new(expected: &'a VTerm, actual: &'b VTerm) -> VTermDiff<'a, 'b> {
        VTermDiff {
            expected: expected,
            actual: actual,
        }
    }

    pub fn has_diff(&self) -> bool {
        self.diff().is_some()
    }

    pub fn diff(&self) -> Option<String> {
        let mut diff = String::new();

        let a_printables = VTermDiff::glyphs(self.expected);
        let b_printables = VTermDiff::glyphs(self.actual);
        if a_printables != b_printables {
            diff.push_str(&VTermDiff::diff_string("glyphs", &a_printables, &b_printables));
        }

        let a_bolds = VTermDiff::bolds(self.expected);
        let b_bolds = VTermDiff::bolds(self.actual);
        if a_bolds != b_bolds {
            diff.push_str(&VTermDiff::diff_string("bolds", &a_bolds, &b_bolds));
        }

        let a_underlines = VTermDiff::underlines(self.expected);
        let b_underlines = VTermDiff::underlines(self.actual);
        if a_underlines != b_underlines {
            diff.push_str(&VTermDiff::diff_string("underlines", &a_underlines, &b_underlines));
        }

        let a_italics = VTermDiff::italics(self.expected);
        let b_italics = VTermDiff::italics(self.actual);
        if a_italics != b_italics {
            diff.push_str(&VTermDiff::diff_string("italics", &a_italics, &b_italics));
        }

        let a_blinks = VTermDiff::blinks(self.expected);
        let b_blinks = VTermDiff::blinks(self.actual);
        if a_blinks != b_blinks {
            diff.push_str(&VTermDiff::diff_string("blinks", &a_blinks, &b_blinks));
        }

        let a_reverses = VTermDiff::reverses(self.expected);
        let b_reverses = VTermDiff::reverses(self.actual);
        if a_reverses != b_reverses {
            diff.push_str(&VTermDiff::diff_string("reverses", &a_reverses, &b_reverses));
        }

        let a_strikes = VTermDiff::strikes(self.expected);
        let b_strikes = VTermDiff::strikes(self.actual);
        if a_strikes != b_strikes {
            diff.push_str(&VTermDiff::diff_string("strikes", &a_strikes, &b_strikes));
        }

        let a_fonts = VTermDiff::fonts(self.expected);
        let b_fonts = VTermDiff::fonts(self.actual);
        if a_fonts != b_fonts {
            diff.push_str(&VTermDiff::diff_string("fonts", &a_fonts, &b_fonts));
        }

        let a_dwls = VTermDiff::dwls(self.expected);
        let b_dwls = VTermDiff::dwls(self.actual);
        if a_dwls != b_dwls {
            diff.push_str(&VTermDiff::diff_string("dwls", &a_dwls, &b_dwls));
        }

        let a_dhls = VTermDiff::dhls(self.expected);
        let b_dhls = VTermDiff::dhls(self.actual);
        if a_dhls != b_dhls {
            diff.push_str(&VTermDiff::diff_string("dhls", &a_dhls, &b_dhls));
        }

        let a_fg_rgbs = VTermDiff::fg_rgbs(self.expected);
        let b_fg_rgbs = VTermDiff::fg_rgbs(self.actual);
        if a_fg_rgbs != b_fg_rgbs {
            diff.push_str(&VTermDiff::diff_string("fg_rgbs", &a_fg_rgbs, &b_fg_rgbs));
        }

        let a_bg_rgbs = VTermDiff::bg_rgbs(self.expected);
        let b_bg_rgbs = VTermDiff::bg_rgbs(self.actual);
        if a_bg_rgbs != b_bg_rgbs {
            diff.push_str(&VTermDiff::diff_string("bg_rgbs", &a_bg_rgbs, &b_bg_rgbs));
        }

        if diff.len() > 0 {
            Some(diff)
        } else {
            None
        }
    }

    fn glyphs(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.chars.len() > 0 {
                let s = String::from_utf8_lossy(&cell.chars);
                line.push(s.chars().next().unwrap());
            } else {
                line.push('\x20');
            }
        })
    }

    fn bolds(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.bold {
                line.push('b');
            } else {
                line.push('\x20');
            }
        })
    }

    fn underlines(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&(cell.attrs.underline % 10).to_string());
        })
    }

    fn italics(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.italic {
                line.push('i');
            } else {
                line.push('\x20');
            }
        })
    }

    fn blinks(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.blink {
                line.push('b');
            } else {
                line.push('\x20');
            }
        })
    }

    fn reverses(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.reverse {
                line.push('r');
            } else {
                line.push('\x20');
            }
        })
    }

    fn strikes(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.strike {
                line.push('s');
            } else {
                line.push('\x20');
            }
        })
    }

    fn fonts(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&(cell.attrs.underline % 10).to_string());
        })
    }

    fn dwls(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.dwl {
                line.push('d');
            } else {
                line.push('\x20');
            }
        })
    }

    fn dhls(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&(cell.attrs.underline % 10).to_string());
        })
    }

    fn fg_rgbs(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&format!(" ({:3},{:3},{:3}) ",
                                   cell.fg_rgb.red,
                                   cell.fg_rgb.green,
                                   cell.fg_rgb.blue));
        })
    }

    fn bg_rgbs(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&format!(" ({:3},{:3},{:3}) ",
                                   cell.bg_rgb.red,
                                   cell.bg_rgb.green,
                                   cell.bg_rgb.blue));
        })
    }

    fn scene_drawer<F>(vterm: &VTerm, mut f: F) -> String
        where F: FnMut(ScreenCell, &mut String)
    {
        let size = vterm.get_size();
        let mut lines: Vec<String> = vec![];
        let mut pos = Pos { x: 0, y: 0 };
        for y in 0..size.height {
            let mut line: String = format!("{}", BORDER);
            pos.y = y;
            for x in 0..size.width {
                pos.x = x;
                let cell = vterm.screen_get_cell(&pos);
                f(cell, &mut line);
            }

            line.push(BORDER);
            lines.push(line);
        }

        if lines.len() > 0 {
            let mut top_bottom = String::new();
            for _ in 0..lines[0].chars().count() {
                top_bottom.push(BORDER);
            }
            lines.insert(0, top_bottom.clone());
            lines.push(top_bottom);
        }

        lines.join("\n")
    }

    fn diff_string(field: &str, expected: &String, actual: &String) -> String {
        format!("{} not equal. expected:\n\n{}\n\nbut got:\n\n{}\n",
                field,
                expected,
                actual)
    }
}

impl<'a, 'b> ::std::fmt::Display for VTermDiff<'a, 'b> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self.diff() {
            Some(d) => write!(f, "VTermDiff {{ diff: {} }}", d),
            None => write!(f, "VTermDiff {{ diff: None }}"),
        }
    }
}

mod tests {
    use super::*;
    use vterm_sys::*;
    use regex;
    use std::io::prelude::*;

    #[test]
    fn has_no_diff_when_vterms_are_the_same() {
        let size = Size { height: 1, width: 1 };
        let vterm = VTerm::new(&size);
        let diff = VTermDiff::new(&vterm, &vterm);
        assert!(!diff.has_diff());
    }

    #[test]
    fn has_diff_when_printables_are_different() {
        let size = Size { height: 1, width: 1 };
        let mut a = VTerm::new(&size);
        let b = VTerm::new(&size);

        a.write(b"a").unwrap();

        let diff = VTermDiff::new(&a, &b);
        assert!(diff.has_diff());
        assert!(regex::is_match("glyphs", &format!("{}", diff)).unwrap());
    }

    #[test]
    fn has_diff_when_bolds_are_different() {
        let size = Size { height: 1, width: 1 };
        let mut a = VTerm::new(&size);
        let mut b = VTerm::new(&size);

        a.write(b"\x1b[1mo").unwrap();
        b.write(b"o").unwrap();

        let diff = VTermDiff::new(&a, &b);
        assert!(diff.has_diff());
        assert!(regex::is_match("bolds", &format!("{}", diff)).unwrap());
    }

    #[test]
    fn has_diff_when_underlines_are_different() {
        let size = Size { height: 1, width: 1 };
        let mut a = VTerm::new(&size);
        let mut b = VTerm::new(&size);

        a.write(b"\x1b[4mo").unwrap();
        b.write(b"o").unwrap();

        let diff = VTermDiff::new(&a, &b);
        assert!(diff.has_diff());
        assert!(regex::is_match("underlines", &format!("{}", diff)).unwrap());
    }

    #[test]
    fn has_diff_when_fg_rbgs_are_different() {
        let size = Size { height: 1, width: 1 };
        let mut a = VTerm::new(&size);
        let mut b = VTerm::new(&size);

        a.write(b"\x1b[31mo").unwrap();
        b.write(b"o").unwrap();

        let diff = VTermDiff::new(&a, &b);
        assert!(diff.has_diff());
        assert!(regex::is_match("fg_rgbs", &format!("{}", diff)).unwrap(),
                format!("expected {} to match", diff));
    }
}
