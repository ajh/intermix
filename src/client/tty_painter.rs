use std::io::prelude::*;
use std::io::BufWriter;
use term::terminfo::{parm, TermInfo};
use vterm_sys::{self, Size, Pos, ColorPalette, ScreenCell, Rect, RectAssist};
use super::pen::*;

// TODO:
//
// * [ ] use terminfo for all writing
// * [ ] set cursor visiblity in reset method
// * [ ] audit other things missed in reset method
// * [ ] DRY sgr code
//
// # Idea to DRY sgr code
//
// 1. Have constant defaults defined for attrs and stuff like BOLD = false;
// 2. all values in Pen are Option
// 3. pull out an sgr function that takes a pen, and writes bytes for Some(_) values
// 4. reset adds Some(_) values for all attrs and writes sgr for everything
// 5. draw cell builds up a pen with Some(_) values that are different than the current pen, then
//    passes it to the sgr fn. Then updates the current pen.

#[derive(Debug)]
pub struct TtyPainter<F: Write + Send> {
    // the physical state of the tty that is being painted
    pen: Pen,
    io: BufWriter<F>,
    size: Size,
}

impl<F: Write + Send> TtyPainter<F> {
    pub fn new(io: F, size: Size) -> TtyPainter<F> {
        TtyPainter {
            io: BufWriter::new(io),
            pen: Pen::new(TermInfo::from_env().unwrap()),
            size: size,
        }
    }

    // Draw the cells in the list starting at the given position
    pub fn draw_cells(&mut self, cells: &Vec<ScreenCell>, rect: &Rect) {
        trace!("draw_cells start rect={:?}", rect);

        for (cell, pos) in cells.iter().zip(rect.positions()) {
            if pos.x >= self.size.width || pos.y >= self.size.height {
                // Not sure this is the right thing to do. How do terminals handle wrapping?
                warn!("skipping draw of cell because its position is outside of our rect");
                continue;
            }

            self.pen.pos = pos;
            self.draw_cell(cell);
        }

        self.io.flush().unwrap();
        trace!("draw_cells finish");
    }

    // This method assumes the pen is already at the correct position
    fn draw_cell(&mut self, cell: &ScreenCell) {
        //trace!("draw_cell cell={:?} pen.pos={:?}", cell, self.pen.pos);

        self.pen.bg = cell.bg_palette;
        self.pen.blink = cell.attrs.blink;
        self.pen.bold = cell.attrs.bold;
        self.pen.fg = cell.fg_palette;
        self.pen.font = cell.attrs.font;
        self.pen.italic = cell.attrs.italic;
        self.pen.reverse = cell.attrs.reverse;
        self.pen.strike = cell.attrs.strike;
        self.pen.underline = cell.attrs.underline;

        self.pen.flush(&mut self.io);

        // See tmux's tty.c:1155 function `tty_cell`
        if cell.chars.len() > 0 {
            self.io.write_all(&cell.chars).ok().expect("failed to write");
        } else {
            // like tmux's tty_repeat_space
            self.io.write_all(&[b'\x20']).ok().expect("failed to write"); // space
        }

        if self.pen.pos.x + 1 < self.size.width {
            self.pen.pos.x += 1;
        } else if self.pen.pos.y + 1 < self.size.height {
            self.pen.pos.x = 0;
            self.pen.pos.y += 1;
        } else {
            warn!("cursor is beyond screen size? What is supposed to happen here?");
            // for now just wedge it into the bottom right corner
            self.pen.pos.x = self.size.width - 1;
            self.pen.pos.y = self.size.height - 1;
        }

        if cell.width > 1 {
            warn!("cell has width > 1 {:?}, but acting on this information isnt implemented",
                  cell)
        }
    }

    /// TODO: take a offset from the pane
    pub fn move_cursor(&mut self, pos: &Pos) {
        trace!("move_cursor pos={:?}", pos);
        self.pen.pos = pos.clone();
    }

    /// Implemented like tmux's tty_redraw_region
    ///
    /// If the pane is the full width of the physical terminal this can be optimized by using
    /// scroll regions, but that isn't implemented.
    ///
    /// Tmux also has an optimization where it'll no-op this if the effected region is >= 50% of
    /// the pane, but will instead schedule a "pane redraw". That is also not implemented.
    /// (&mut self, scroll_region_size: &Size, scroll_region_pos: &Pos) {
    pub fn insert_line(&mut self, _: &Size, _: &Pos) {
        // I'd like to iterate through all the cells in the pane. Can I get access to this?
    }

    pub fn flush(&mut self) {
        self.pen.flush(&mut self.io);
        self.io.flush();
    }

    // pub fn delete_line<F: Write>(&mut self, pane: &Pane, io: &mut F) {
    // /deleteLine: CSR(top, bottom) + CUP(y, 0) + DL(1) + CSR(0, height)
    // }
}
