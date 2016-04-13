use vterm_sys::{Pos, ScreenCell, ColorPalette, ColorRGB};

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub bg_palette: ColorPalette,
    pub bg_rgb: ColorRGB,
    pub blink: bool,
    pub bold: bool,

    // TODO: This should be a fixed size array, and method should exist to return slices.
    pub chars: Vec<u8>,

    pub dhl: u8, // On a DECDHL line (1=top 2=bottom)
    pub dirty: bool,
    pub dwl: bool, // On a DECDWL or DECDHL line
    pub fg_palette: ColorPalette,
    pub fg_rgb: ColorRGB,
    pub font: u8, // 0 to 9
    pub italic: bool,

    // NOTE: I don't love that this is here, but it makes returning iterators in CellBuffer
    // possible. Maybe having pos here will work better this time.
    pub pos: Pos,

    pub reverse: bool,
    pub strike: bool,
    pub underline: u8, // 0 to 3
    pub width: u8,
}

impl Cell {
    pub fn new(pos: Pos) -> Cell {
        Cell {
            bg_palette: Default::default(),
            bg_rgb: Default::default(),
            blink: Default::default(),
            bold: Default::default(),
            chars: Default::default(),
            dhl: Default::default(),
            dirty: Default::default(),
            dwl: Default::default(),
            fg_palette: Default::default(),
            fg_rgb: Default::default(),
            font: Default::default(),
            italic: Default::default(),
            pos: pos,
            reverse: Default::default(),
            strike: Default::default(),
            underline: Default::default(),
            width: Default::default(),
        }

    }

    pub fn update_from_vterm_cell(&mut self, vterm_cell: &ScreenCell) {
      self.bg_palette = vterm_cell.bg_palette;
      self.bg_rgb = vterm_cell.bg_rgb.clone();
      self.blink = vterm_cell.attrs.blink;
      self.bold = vterm_cell.attrs.bold;
      self.chars = vterm_cell.chars.clone();
      self.dhl = vterm_cell.attrs.dhl;
      self.dwl = vterm_cell.attrs.dwl;
      self.fg_palette = vterm_cell.fg_palette;
      self.fg_rgb = vterm_cell.fg_rgb.clone();
      self.font = vterm_cell.attrs.font;
      self.italic = vterm_cell.attrs.italic;
      self.reverse = vterm_cell.attrs.reverse;
      self.strike = vterm_cell.attrs.strike;
      self.underline = vterm_cell.attrs.underline;
      self.width = vterm_cell.width;
    }
}
