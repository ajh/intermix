mod layout;
mod line_container;
mod wrap;
mod wrap_builder;

pub use self::layout::*;
pub use self::line_container::*;
pub use self::wrap::*;
pub use self::wrap_builder::*;

pub const GRID_COLUMNS_COUNT: usize = 12;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Default for Align {
    fn default() -> Align {
        Align::Left
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}
impl Default for VerticalAlign {
    fn default() -> VerticalAlign {
        VerticalAlign::Top
    }
}
