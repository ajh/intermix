use std::io::prelude::*;
use std::io::BufWriter;
use term::terminfo::{parm, TermInfo};
use vterm_sys::{Size, Pos, ScreenCell, Rect, RectAssist};
use super::pen::*;
use ::cell_buffer::*;

// TODO:
// * [ ] clean up error handling and all the expects on write_all

pub struct TtyPainter<F: Write + Send> {
    // the physical state of the tty that is being painted
    pen: Pen,
    io: BufWriter<F>,
    size: Size,
    terminfo: TermInfo,
    vars: parm::Variables,
}

impl<F: Write + Send> TtyPainter<F> {
    pub fn new(io: F, size: Size) -> TtyPainter<F> {
        TtyPainter {
            io: BufWriter::new(io),
            pen: Pen::new(),
            size: size,
            // Note: a better idea would be to have the caller choose the terminal type
            terminfo: TermInfo::from_env().unwrap(),
            vars: parm::Variables::new(),
        }
    }

    // Draw the cells in the given rectangle
    pub fn draw_cells(&mut self, cells: &Vec<ScreenCell>, rect: &Rect) {
        trace!("draw_cells start rect={:?}", rect);

        let old_visible = self.pen.visible;
        self.pen.visible = false;
        let bytes = self.pen.flush(&self.terminfo, &mut self.vars);
        self.io.write_all(&bytes).ok().expect("failed to write");

        self.write_cap("sc", &vec![]);

        for (cell, pos) in cells.iter().zip(rect.positions()) {
            if pos.x >= self.size.width || pos.y >= self.size.height {
                // Not sure this is the right thing to do. How do terminals handle wrapping?
                warn!("skipping draw of cell because its position is outside of our rect");
                continue;
            }

            self.pen.pos = pos;
            self.pen.update_attrs_from_cell(&cell);
            let bytes = self.pen.flush(&self.terminfo, &mut self.vars);
            self.io.write_all(&bytes).ok().expect("failed to write");

            // See tmux's tty.c:1155 function `tty_cell`
            if cell.chars.len() > 0 {
                self.io.write_all(&cell.chars).ok().expect("failed to write");
            } else {
                // like tmux's tty_repeat_space
                self.io.write_all(&[b'\x20']).ok().expect("failed to write"); // space
            }

            self.pen.notify_of_advanced_pos(&self.size);

            if cell.width > 1 {
                warn!("cell has width > 1 {:?}, but acting on this information isnt implemented",
                      cell)
            }
        }

        self.write_cap("rc", &vec![]);

        self.pen.visible = old_visible;
        let bytes = self.pen.flush(&self.terminfo, &mut self.vars);
        self.io.write_all(&bytes).ok().expect("failed to write");

        self.io.flush().unwrap();
        trace!("draw_cells finish");
    }

    pub fn draw_screen(&mut self, screen: &mut CellBuffer) {
        trace!("draw_screen start");

        let old_visible = self.pen.visible;
        self.pen.visible = false;
        let bytes = self.pen.flush(&self.terminfo, &mut self.vars);
        self.io.write_all(&bytes).ok().expect("failed to write");

        self.write_cap("sc", &vec![]);
        for cell in screen.iter_mut().filter(|c| c.dirty) {
            cell.dirty = false;

            if cell.pos.x >= self.size.width || cell.pos.y >= self.size.height {
                // Not sure this is the right thing to do. How do terminals handle wrapping?
                warn!("skipping draw of cell because its position is outside of our rect");
                continue;
            }

            self.pen.pos = cell.pos.clone();
            self.pen.update_attrs_from_cell2(cell);
            let bytes = self.pen.flush(&self.terminfo, &mut self.vars);
            self.io.write_all(&bytes).ok().expect("failed to write");

            // See tmux's tty.c:1155 function `tty_cell`
            if cell.chars.len() > 0 {
                self.io.write_all(&cell.chars).ok().expect("failed to write");
            } else {
                // like tmux's tty_repeat_space
                self.io.write_all(&[b'\x20']).ok().expect("failed to write"); // space
            }

            self.pen.notify_of_advanced_pos(&self.size);

            if cell.width > 1 {
                warn!("cell has width > 1 {:?}, but acting on this information isnt implemented",
                      cell)
            }
        }

        self.write_cap("rc", &vec![]);

        self.pen.visible = old_visible;
        let bytes = self.pen.flush(&self.terminfo, &mut self.vars);
        self.io.write_all(&bytes).ok().expect("failed to write");

        self.io.flush().unwrap();
        trace!("draw_cells finish");
    }

    pub fn move_cursor(&mut self, pos: Pos, is_visible: bool) {
        trace!("move_cursor pos={:?} is_visible={:?}", pos, is_visible);
        self.pen.pos = pos;
        self.pen.visible = is_visible;
        self.io.write_all(&self.pen.flush(&self.terminfo, &mut self.vars)).ok().expect("failed to write");
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
        self.io.write_all(&self.pen.flush(&self.terminfo, &mut self.vars)).ok().expect("failed to write");
        self.io.flush().unwrap();
    }

    pub fn reset(&mut self) {
        self.pen = Pen::new();
    }

    // pub fn delete_line<F: Write>(&mut self, pane: &Pane, io: &mut F) {
    // /deleteLine: CSR(top, bottom) + CUP(y, 0) + DL(1) + CSR(0, height)
    // }

    fn write_cap(&mut self, cap: &str, params: &Vec<parm::Param>) {
        let cmd = self.terminfo.strings.get(cap).unwrap();
        let bytes = parm::expand(&cmd, params.as_slice(), &mut self.vars).unwrap();
        self.io.write_all(&bytes).ok().expect("failed to write");
    }
}
