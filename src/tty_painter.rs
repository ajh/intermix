
pub struct Pen {
    attrs: ScreenCellAttr,
    fg: Color,
    bg: Color,
    row: u16,
    col: u16,
}

pub struct TtyPainter {
    pen: Pen,
}

impl TtyPainter {
    pub fn new() -> TtyPainter {
        TtyPainter { };
    }

    pub fn draw_cells<F: Write>(cells: &[Cell], io: &mut F) {
    }
}
