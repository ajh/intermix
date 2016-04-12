use vterm_sys::{ColorPalette, ColorRGB};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Cell {
    pub bg_palette: ColorPalette,
    pub bg_rgb: ColorRGB,
    pub blink: bool,
    pub bold: bool,
    pub chars: Vec<u8>,
    pub dhl: u8, // On a DECDHL line (1=top 2=bottom)
    pub dwl: bool, // On a DECDWL or DECDHL line
    pub fg_palette: ColorPalette,
    pub fg_rgb: ColorRGB,
    pub font: u8, // 0 to 9
    pub dirty: bool,
    pub italic: bool,
    pub reverse: bool,
    pub strike: bool,
    pub underline: u8, // 0 to 3
    pub width: u8,
}

impl Cell {
    pub fn new() -> Cell {
        Default::default()
    }
}
