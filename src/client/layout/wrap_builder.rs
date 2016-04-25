use super::*;

pub struct WrapBuilder {
    align: Option<Align>,
    grid_width: Option<usize>,
    has_border: Option<bool>,
    height: Option<usize>,
    margin: Option<usize>,
    name: Option<String>,
    padding: Option<usize>,
    vertical_align: Option<VerticalAlign>,
    width: Option<usize>,
}

macro_rules! fn_writer {
    ($field_name:ident, $type_name:ident) => {
        pub fn $field_name(mut self, val: $type_name) -> WrapBuilder {
            self.$field_name = Some(val);
            self
        }
    }
}

impl WrapBuilder {
    /// call this to create a column
    pub fn col(val: usize) -> WrapBuilder {
        WrapBuilder {
            align: None,
            grid_width: Some(val),
            has_border: None,
            height: None,
            margin: None,
            name: None,
            padding: None,
            vertical_align: None,
            width: None,
        }
    }

    /// call this to create a row
    pub fn row() -> WrapBuilder {
        WrapBuilder {
            align: None,
            grid_width: Some(GRID_COLUMNS_COUNT),
            has_border: None,
            height: None,
            margin: None,
            name: None,
            padding: None,
            vertical_align: None,
            width: None,
        }
    }

    fn_writer!(align, Align);
    fn_writer!(grid_width, usize);
    fn_writer!(has_border, bool);
    fn_writer!(height, usize);
    fn_writer!(margin, usize);
    fn_writer!(name, String);
    fn_writer!(padding, usize);
    fn_writer!(vertical_align, VerticalAlign);
    fn_writer!(width, usize);

    pub fn build(self) -> Wrap {
        let mut wrap = Wrap::new();

        if self.align.is_some() {
            wrap.set_align(self.align.unwrap())
        }
        if self.grid_width.is_some() {
            wrap.set_grid_width(self.grid_width)
        }
        if self.has_border.is_some() {
            wrap.set_has_border(self.has_border.unwrap())
        }
        if self.height.is_some() {
            wrap.set_height(self.height)
        }
        if self.margin.is_some() {
            wrap.set_margin(self.margin.unwrap())
        }
        if self.name.is_some() {
            wrap.set_name(self.name.unwrap())
        }
        if self.padding.is_some() {
            wrap.set_padding(self.padding.unwrap())
        }
        if self.vertical_align.is_some() {
            wrap.set_vertical_align(self.vertical_align.unwrap())
        }
        if self.width.is_some() {
            wrap.set_width(self.width)
        }

        wrap
    }
}
