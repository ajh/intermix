use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Wrap {
    align: Align,
    computed_grid_width: Option<usize>,
    computed_height: Option<usize>,
    computed_width: Option<usize>,
    computed_x: Option<usize>,
    computed_y: Option<usize>,
    grid_width: Option<usize>,
    has_border: bool,
    height: Option<usize>,
    is_new_line: bool,
    margin: usize,
    name: String,
    padding: usize,
    vertical_align: VerticalAlign,
    width: Option<usize>,
}

macro_rules! fn_option_accessor {
    // Can this be less redundent?
    ($field_name:ident, $setter_name:ident, $type_name:ident) => {
        pub fn $field_name(&self) -> Option<$type_name> {
            self.$field_name
        }

        pub fn $setter_name(&mut self, val: Option<$type_name>) {
            self.$field_name = val
        }
    }
}

macro_rules! fn_accessor {
    // Can this be less redundent?
    ($field_name:ident, $writer_name:ident, $type_name:ident) => {
        pub fn $field_name(&self) -> $type_name {
            self.$field_name
        }

        pub fn $writer_name(&mut self, val: $type_name) {
            self.$field_name = val
        }
    }
}

impl Wrap {
    pub fn new() -> Wrap {
        Default::default()
    }

    fn_option_accessor!(computed_grid_width, set_computed_grid_width, usize);
    fn_option_accessor!(computed_height, set_computed_height, usize);
    fn_option_accessor!(computed_width, set_computed_width, usize);
    fn_option_accessor!(computed_x, set_computed_x, usize);
    fn_option_accessor!(computed_y, set_computed_y, usize);
    fn_option_accessor!(grid_width, set_grid_width, usize);
    fn_option_accessor!(height, set_height, usize);
    fn_option_accessor!(width, set_width, usize);

    fn_accessor!(align, set_align, Align);
    fn_accessor!(has_border, set_has_border, bool);
    fn_accessor!(is_new_line, set_is_new_line, bool);
    fn_accessor!(margin, set_margin, usize);
    fn_accessor!(padding, set_padding, usize);
    fn_accessor!(vertical_align, set_vertical_align, VerticalAlign);

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, val: String) {
        self.name = val
    }

    pub fn outside_height(&self) -> Option<usize> {
        if let Some(mut h) = self.computed_height() {
            h += 2 *
                 (self.margin + self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            Some(h)
        } else {
            None
        }
    }

    pub fn set_outside_height(&mut self, val: Option<usize>) {
        if let Some(mut v) = val {
            v -= 2 *
                 (self.margin + self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            self.set_computed_height(Some(v));
        } else {
            self.set_computed_height(None);
        }
    }

    pub fn outside_width(&self) -> Option<usize> {
        if let Some(mut w) = self.computed_width() {
            w += 2 *
                 (self.margin + self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            Some(w)
        } else {
            None
        }
    }

    pub fn set_outside_width(&mut self, val: Option<usize>) {
        if let Some(mut v) = val {
            v -= 2 *
                 (self.margin + self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            self.set_computed_width(Some(v));
        } else {
            self.set_computed_width(None);
        }
    }

    pub fn outside_x(&self) -> Option<usize> {
        if let Some(mut x) = self.computed_x() {
            x -= self.margin + self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            Some(x)
        } else {
            None
        }
    }

    pub fn set_outside_x(&mut self, val: Option<usize>) {
        if let Some(mut v) = val {
            v += self.margin + self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            self.set_computed_x(Some(v));
        } else {
            self.set_computed_x(None);
        }
    }

    pub fn outside_y(&self) -> Option<usize> {
        if let Some(mut y) = self.computed_y() {
            y -= self.margin + self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            Some(y)
        } else {
            None
        }
    }

    pub fn set_outside_y(&mut self, val: Option<usize>) {
        if let Some(mut v) = val {
            v += self.margin + self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            self.set_computed_y(Some(v));
        } else {
            self.set_computed_y(None);
        }
    }

    pub fn border_height(&self) -> Option<usize> {
        if let Some(mut h) = self.computed_height() {
            h += 2 *
                 (self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            Some(h)
        } else {
            None
        }
    }

    pub fn border_width(&self) -> Option<usize> {
        if let Some(mut w) = self.computed_width() {
            w += 2 *
                 (self.padding +
                  if self.has_border {
                1
            } else {
                0
            });
            Some(w)
        } else {
            None
        }
    }

    pub fn border_x(&self) -> Option<usize> {
        if let Some(mut x) = self.computed_x() {
            x -= self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            Some(x)
        } else {
            None
        }
    }

    pub fn border_y(&self) -> Option<usize> {
        if let Some(mut y) = self.computed_y() {
            y -= self.padding +
                 if self.has_border {
                1
            } else {
                0
            };
            Some(y)
        } else {
            None
        }
    }
}

impl Default for Wrap {
    fn default() -> Wrap {
        Wrap {
            align: Default::default(),
            computed_grid_width: None,
            computed_height: None,
            computed_width: None,
            computed_x: None,
            computed_y: None,
            grid_width: None,
            has_border: false,
            height: None,
            is_new_line: false,
            margin: 0,
            name: String::new(), // maybe a uuid?
            padding: 0,
            vertical_align: Default::default(),
            width: None,
        }
    }
}
