#[derive(Debug)]
pub struct CellAttr {
	pub fccode:     i8, 	// foreground color code or <0 for rgb
	pub bccode:     i8, 	// background color code or <0 for rgb
	pub fr:         u8, 	// foreground red
	pub fg:         u8, 	// foreground green
	pub fb:         u8, 	// foreground blue
	pub br:         u8, 	// background red
	pub bg:         u8, 	// background green
	pub bb:         u8, 	// background blue
	pub bold:       bool,	/* bold character */
	pub underline:  bool,	/* underlined character */
	pub inverse:    bool,	/* inverse colors */
	pub protect:    bool,	/* cannot be erased */
	pub blink:      bool	/* blinking character */
}

impl Default for CellAttr {
    fn default() -> CellAttr {
        CellAttr {
            fccode:     0,
            bccode:     0,
            fr:         0,
            fg:         0,
            fb:         0,
            br:         0,
            bg:         0,
            bb:         0,
            bold:       false,
            underline:  false,
            inverse:    false,
            protect:    false,
            blink:      false,
        }
    }
}

impl Clone for CellAttr {
    fn clone(&self) -> Self {
        CellAttr {
            fccode:     self.fccode,
            bccode:     self.bccode,
            fr:         self.fr,
            fg:         self.fg,
            fb:         self.fb,
            br:         self.br,
            bg:         self.bg,
            bb:         self.bb,
            bold:       self.bold,
            underline:  self.underline,
            inverse:    self.inverse,
            protect:    self.protect,
            blink:      self.blink,
        }
    }
}

#[derive(Debug)]
pub struct Cell {
    pub x:      usize,
    pub y:      usize,
	pub ch:     char,
	pub width:  usize,
    pub attr:   CellAttr,
    pub age:    u32
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            x:      0,
            y:      0,
            ch:     '\x00' as char,
            width:  1,
            attr:   Default::default(),
            age:    0
        }
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            x:      self.x,
            y:      self.y,
            ch:     self.ch,
            width:  self.width,
            attr:   self.attr.clone(),
            age:    self.age
        }
    }
}
