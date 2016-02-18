use vterm_sys::*;

pub struct VTermDiff<'a, 'b> {
    a: &'a VTerm,
    b: &'b VTerm,
}

const BORDER: char = '·';

impl<'a, 'b> VTermDiff<'a, 'b> {
    pub fn new(a: &'a VTerm, b: &'b VTerm) -> VTermDiff<'a, 'b> {
        VTermDiff {
            a: a,
            b: b,
        }
    }

    pub fn has_diff(&self) -> bool {
        self.diff().is_some()
    }

    pub fn diff(&self) -> Option<String> {
        let mut diff = String::new();

        let a_printables = VTermDiff::printables(self.a);
        let b_printables = VTermDiff::printables(self.b);
        if a_printables != b_printables {
            diff.push_str(&VTermDiff::diff_string("printables", &a_printables, &b_printables));
        }

        let a_unprintables = VTermDiff::unprintables(self.a);
        let b_unprintables = VTermDiff::unprintables(self.b);
        if a_unprintables != b_unprintables {
            diff.push_str(&VTermDiff::diff_string("unprintables", &a_unprintables, &b_unprintables));
        }

        let a_bolds = VTermDiff::bolds(self.a);
        let b_bolds = VTermDiff::bolds(self.b);
        if a_bolds != b_bolds {
            diff.push_str(&VTermDiff::diff_string("bolds", &a_bolds, &b_bolds));
        }

        let a_underlines = VTermDiff::underlines(self.a);
        let b_underlines = VTermDiff::underlines(self.b);
        if a_underlines != b_underlines {
            diff.push_str(&VTermDiff::diff_string("underlines", &a_underlines, &b_underlines));
        }

        let a_italics = VTermDiff::italics(self.a);
        let b_italics = VTermDiff::italics(self.b);
        if a_italics != b_italics {
            diff.push_str(&VTermDiff::diff_string("italics", &a_italics, &b_italics));
        }

        let a_blinks = VTermDiff::blinks(self.a);
        let b_blinks = VTermDiff::blinks(self.b);
        if a_blinks != b_blinks {
            diff.push_str(&VTermDiff::diff_string("blinks", &a_blinks, &b_blinks));
        }

        let a_reverses = VTermDiff::reverses(self.a);
        let b_reverses = VTermDiff::reverses(self.b);
        if a_reverses != b_reverses {
            diff.push_str(&VTermDiff::diff_string("reverses", &a_reverses, &b_reverses));
        }

        let a_strikes = VTermDiff::strikes(self.a);
        let b_strikes = VTermDiff::strikes(self.b);
        if a_strikes != b_strikes {
            diff.push_str(&VTermDiff::diff_string("strikes", &a_strikes, &b_strikes));
        }

        let a_fonts = VTermDiff::fonts(self.a);
        let b_fonts = VTermDiff::fonts(self.b);
        if a_fonts != b_fonts {
            diff.push_str(&VTermDiff::diff_string("fonts", &a_fonts, &b_fonts));
        }

        let a_dwls = VTermDiff::dwls(self.a);
        let b_dwls = VTermDiff::dwls(self.b);
        if a_dwls != b_dwls {
            diff.push_str(&VTermDiff::diff_string("dwls", &a_dwls, &b_dwls));
        }

        let a_dhls = VTermDiff::dhls(self.a);
        let b_dhls = VTermDiff::dhls(self.b);
        if a_dhls != b_dhls {
            diff.push_str(&VTermDiff::diff_string("dhls", &a_dhls, &b_dhls));
        }

        // My code isn't ready for this yet
        //let a_fg_rgbs = VTermDiff::fg_rgbs(self.a);
        //let b_fg_rgbs = VTermDiff::fg_rgbs(self.b);
        //if a_fg_rgbs != b_fg_rgbs {
            //diff.push_str(&VTermDiff::diff_string("fg_rgbs", &a_fg_rgbs, &b_fg_rgbs));
        //}

        //let a_bg_rgbs = VTermDiff::bg_rgbs(self.a);
        //let b_bg_rgbs = VTermDiff::bg_rgbs(self.b);
        //if a_bg_rgbs != b_bg_rgbs {
            //diff.push_str(&VTermDiff::diff_string("bg_rgbs", &a_bg_rgbs, &b_bg_rgbs));
        //}

        if diff.len() > 0 { Some(diff) } else { None }
    }

    fn printables(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            let chars = cell.chars;
            if chars.len() > 0 && !chars[0].is_control() {
                line.push(chars[0]);
            }
            else {
                line.push('\x20');
            }
        })
    }

    fn unprintables(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            let chars = cell.chars;
            if chars.len() > 0 && chars[0].is_control() {
                line.push('x'); // x marks the spot
            }
            else {
                line.push('\x20');
            }
        })
    }

    fn bolds(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.bold { line.push('b'); } else { line.push('\x20'); }
        })
    }

    fn underlines(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&(cell.attrs.underline % 10).to_string());
        })
    }

    fn italics(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.italic { line.push('i'); } else { line.push('\x20'); }
        })
    }

    fn blinks(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.blink { line.push('b'); } else { line.push('\x20'); }
        })
    }

    fn reverses(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.reverse { line.push('r'); } else { line.push('\x20'); }
        })
    }

    fn strikes(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.strike { line.push('s'); } else { line.push('\x20'); }
        })
    }

    fn fonts(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&(cell.attrs.underline % 10).to_string());
        })
    }

    fn dwls(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            if cell.attrs.dwl { line.push('d'); } else { line.push('\x20'); }
        })
    }

    fn dhls(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&(cell.attrs.underline % 10).to_string());
        })
    }

    fn fg_rgbs(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&format!("({:3}, {:3}, {:3})", cell.fg_rgb.red, cell.fg_rgb.green, cell.fg_rgb.blue));
        })
    }

    fn bg_rgbs(vterm: &VTerm) -> String {
        VTermDiff::scene_drawer(vterm, |cell, line| {
            line.push_str(&format!("({:3}, {:3}, {:3})", cell.bg_rgb.red, cell.bg_rgb.green, cell.bg_rgb.blue));
        })
    }

    fn scene_drawer<F>(vterm: &VTerm, mut f: F) -> String where F: FnMut(ScreenCell, &mut String) {
        let size = vterm.get_size();
        let mut lines: Vec<String> = vec![];
        let mut top_bottom = String::new();
        for _ in 0..size.cols + 2 {
            top_bottom.push(BORDER);
        }
        lines.push(top_bottom.clone());

        let mut pos: Pos = Default::default();
        for y in 0..size.rows {
            let mut line: String = format!("{}", BORDER);
            pos.row = y as i16;
            for x in 0..size.cols {
                pos.col = x as i16;
                let cell = vterm.screen_get_cell(&pos);
                f(cell, &mut line);
            }

            line.push(BORDER);
            lines.push(line);
        }

        lines.push(top_bottom);

        lines.join("\n")
    }

    fn diff_string(field: &str, a: &String, b: &String) -> String {
        format!("{} not equal:\n\n{}\n\nvs:\n\n{}\n", field, a, b)
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

    #[test]
    fn has_no_diff_when_vterms_are_the_same() {
        let size = ScreenSize {
            rows: 1,
            cols: 1,
        };
        let vterm = VTerm::new(size.clone());
        let diff = VTermDiff::new(&vterm, &vterm);
        assert!(!diff.has_diff());
    }

    #[test]
    fn has_diff_when_printables_are_different() {
        let size = ScreenSize {
            rows: 1,
            cols: 1,
        };
        let mut a = VTerm::new(size.clone());
        let b = VTerm::new(size.clone());

        a.write(b"a");

        let diff = VTermDiff::new(&a, &b);
        assert!(diff.has_diff());
        assert!(regex::is_match("printables", &format!("{}", diff)).unwrap());
    }

    // Not sure how to test these, or if VTerm even responds when writing unprintables?
    //#[test]
    //fn has_diff_when_unprintables_are_different() {
        //let size = ScreenSize {
            //rows: 1,
            //cols: 1,
        //};
        //let mut a = VTerm::new(size.clone());
        //let mut b = VTerm::new(size.clone());

        //a.write(b"\x00");

        //let diff = VTermDiff::new(&a, &b);
        //println!("{}", diff);
        //assert!(diff.has_diff());
    //}

    //#[test]
    //fn displays_diff_when_unprintables_are_different() {
        //let size = ScreenSize {
            //rows: 1,
            //cols: 1,
        //};
        //let mut a = VTerm::new(size.clone());
        //let mut b = VTerm::new(size.clone());

        //a.write(b"\x00");

        //let diff = VTermDiff::new(&a, &b);
        //assert!(regex::is_match("x", &format!("{}", diff)).unwrap());
    //}

    #[test]
    fn has_diff_when_bolds_are_different() {
        let size = ScreenSize {
            rows: 1,
            cols: 1,
        };
        let mut a = VTerm::new(size.clone());
        let mut b = VTerm::new(size.clone());

        a.write(b"\x1b[1mo");
        b.write(b"o");

        let diff = VTermDiff::new(&a, &b);
        assert!(diff.has_diff());
        assert!(regex::is_match("bolds", &format!("{}", diff)).unwrap());
    }

    #[test]
    fn has_diff_when_underlines_are_different() {
        let size = ScreenSize {
            rows: 1,
            cols: 1,
        };
        let mut a = VTerm::new(size.clone());
        let mut b = VTerm::new(size.clone());

        a.write(b"\x1b[4mo");
        b.write(b"o");

        let diff = VTermDiff::new(&a, &b);
        assert!(diff.has_diff());
        assert!(regex::is_match("underlines", &format!("{}", diff)).unwrap());
    }

    // disabled for now because turning on this feature breaks other tests. Need to fix those
    // first.
    //#[test]
    //fn has_diff_when_fg_rbgs_are_different() {
        //let size = ScreenSize {
            //rows: 1,
            //cols: 1,
        //};
        //let mut a = VTerm::new(size.clone());
        //let mut b = VTerm::new(size.clone());

        //a.write(b"\x1b[31mo");
        //b.write(b"o");

        //let diff = VTermDiff::new(&a, &b);
        //assert!(diff.has_diff());
        //assert!(regex::is_match("fg_rbgs", &format!("{}", diff)).unwrap(), format!("expected {} to match", diff));
    //}
}
