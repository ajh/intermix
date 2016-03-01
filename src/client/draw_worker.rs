use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Arc, RwLock};
use super::*;
use super::tty_painter::*;
use super::layout::*;
use vterm_sys::{self, Pos, ScreenSize, ScreenCell, Rect};

/// # todos
/// * [ ] make message enum more specific
/// * [ ] Maybe build with initial state to avoid bootstrap data race?
/// * [ ] optimize drawing when pane is full width (csr)
/// * [ ] implement move cursor
pub struct DrawWorker<F: 'static + Write + Send> {
    rx: Receiver<ClientMsg>,
    painter: TtyPainter<F>,
    layout: Arc<RwLock<layout::Screen>>,
}

impl<F: 'static + Write + Send> DrawWorker<F> {
    pub fn spawn(io: F,
                 rx: Receiver<ClientMsg>,
                 layout: Arc<RwLock<layout::Screen>>)
                 -> thread::JoinHandle<()> {
        info!("spawning draw worker");
        thread::spawn(move || {
            let mut worker = DrawWorker::new(rx, io, layout);
            worker.enter_listen_loop();
            info!("exiting draw worker");
        })
    }

    fn new(rx: Receiver<ClientMsg>, io: F, layout: Arc<RwLock<layout::Screen>>) -> DrawWorker<F> {
        let size = { layout.read().unwrap().size.clone() };
        let mut painter = TtyPainter::new(io, size);

        DrawWorker {
            rx: rx,
            painter: painter,
            layout: layout,
        }
    }

    /// Start receiving messages from Receiver. Exits on a Quit message.
    fn enter_listen_loop(&mut self) {
        loop {
            let msg = match self.rx.recv() {
                Ok(msg) => msg,
                Err(_) => break,
            };

            match msg {
                ClientMsg::Quit => break,
                ClientMsg::ProgramDamage { program_id, cells, rect } => {
                    self.program_damage(program_id, cells, rect)
                }
                ClientMsg::Clear => self.clear(),
                ClientMsg::LayoutDamage => self.layout_damage(),
                ClientMsg::LayoutSwap { layout } => self.layout_swap(layout),
                ClientMsg::ProgramMoveCursor { program_id, old: _, new, is_visible } => {
                    self.move_cursor(program_id, new, is_visible)
                }
                _ => warn!("unhandled msg {:?}", msg),
            }
        }
    }

    fn program_damage(&mut self, program_id: String, cells: Vec<vterm_sys::ScreenCell>, rect: vterm_sys::Rect) {
        trace!("program_damage for {}", program_id);

        let layout = self.layout.read().unwrap();
        if let Some(wrap) = layout.tree().values().find(|w| *w.name() == program_id) {
            let rect_with_offset = Rect {
                start_row: rect.start_row + wrap.computed_y().unwrap(),
                end_row: rect.end_row + wrap.computed_y().unwrap(),
                start_col: rect.start_col + wrap.computed_x().unwrap(),
                end_col: rect.end_col + wrap.computed_x().unwrap(),
            };
            self.painter.draw_cells(&cells, &rect_with_offset);
        } else {
            warn!("didnt find node with value: {:?}", program_id);
        }
    }

    fn layout_damage(&mut self) {
        trace!("layout_damage");

        let layout = self.layout.read().unwrap();
        // trace!("{:#?}", layout.tree());

        for wrap in layout.tree().values() {
            DrawWorker::draw_border_for_node(&mut self.painter, wrap, &layout.size);
        }
    }

    fn draw_border_for_node(painter: &mut TtyPainter<F>,
                             wrap: &layout::Wrap,
                             size: &ScreenSize) {
        if wrap.has_border() {
            let mut top = wrap.border_y().unwrap();
            if top < 0 {
                top = 0
            }

            let mut bottom = wrap.border_y().unwrap() + wrap.border_height().unwrap() - 1;
            if bottom >= size.rows {
                bottom = size.rows - 1
            }

            let mut left = wrap.border_x().unwrap();
            if left < 0 {
                left = 0
            }

            let mut right = wrap.border_x().unwrap() + wrap.border_width().unwrap() - 1;
            if right >= size.cols {
                right = size.cols - 1
            }

            painter.draw_cells(&vec![
                                    ScreenCell {
                                        chars: "┌".to_string().into_bytes(),
                                        ..Default::default()
                                    }],
                                    &Rect {
                                        start_row: top,
                                        end_row: top + 1,
                                        start_col: left,
                                        end_col: left + 1,
                                    });
            painter.draw_cells(&vec![
                                    ScreenCell {
                                        chars: "┐".to_string().into_bytes(),
                                        ..Default::default()
                                    }],
                                    &Rect {
                                        start_row: top,
                                        end_row: top + 1,
                                        start_col: right,
                                        end_col: right + 1,
                                    });
            painter.draw_cells(&vec![
            ScreenCell {
                chars: "└".to_string().into_bytes(),
                ..Default::default()
            }],
                &Rect {
                    start_row: bottom,
                    end_row: bottom + 1,
                    start_col: left,
                    end_col: left + 1,
                });
            painter.draw_cells(&vec![
            ScreenCell {
                chars: "┘".to_string().into_bytes(),
                ..Default::default()
            }],
                &Rect {
                    start_row: bottom,
                    end_row: bottom + 1,
                    start_col: right,
                    end_col: right + 1,
                });

            for x in left + 1..right {
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: "─".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect {
                            start_row: top,
                            end_row: top + 1,
                            start_col: x,
                            end_col: x + 1,
                    }
                    );
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: "─".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect {
                        start_row: bottom,
                        end_row: bottom + 1,
                        start_col: x,
                        end_col: x + 1,
                    });
            }

            for y in top + 1..bottom {
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: "│".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect {
                        start_row: y,
                        end_row: y + 1,
                        start_col: left,
                        end_col: left + 1,
                    });
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: "│".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect {
                        start_row: y,
                        end_row: y + 1,
                        start_col: right,
                        end_col: right + 1,
                    });
            }
        } else if wrap.margin() > 0 {
            let mut top = wrap.computed_y().unwrap() - wrap.padding() - 1;
            if top < 0 {
                top = 0
            }

            let mut bottom = wrap.computed_y().unwrap() + wrap.computed_height().unwrap() +
                             wrap.padding();
            if bottom >= size.rows {
                bottom = size.rows - 1
            }

            let mut left = wrap.computed_x().unwrap() - wrap.padding() - 1;
            if left < 0 {
                left = 0
            }

            let mut right = wrap.computed_x().unwrap() + wrap.computed_width().unwrap() +
                            wrap.padding();
            if right >= size.cols {
                right = size.cols - 1
            }

            for x in left..right + 1 {
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: " ".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect {
                        start_row: top,
                        end_row: top + 1,
                        start_col: x,
                        end_col: x + 1,
                    });
                painter.draw_cells(&vec![
                ScreenCell {
                    chars: " ".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect {
                        start_row: bottom,
                        end_row: bottom + 1,
                        start_col: x,
                        end_col: x + 1,
                    });
            }

            for y in top + 1..bottom {
            painter.draw_cells(&vec![
                ScreenCell {
                    chars: " ".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect {
                        start_row: y,
                        end_row: y + 1,
                        start_col: left,
                        end_col: left + 1,
                    });
            painter.draw_cells(&vec![
                ScreenCell {
                    chars: " ".to_string().into_bytes(),
                    ..Default::default()
                }],
                    &Rect {
                        start_row: y,
                        end_row: y + 1,
                        start_col: right,
                        end_col: right + 1,
                    });
            }
        }
    }

    fn clear(&mut self) {
        let layout = self.layout.read().unwrap();
        let mut cells: Vec<ScreenCell> = vec![];
        for _ in 0..layout.size.rows {
            for _ in 0..layout.size.cols {
                cells.push(ScreenCell {
                    chars: " ".to_string().into_bytes(),
                    ..Default::default()
                });
            }
        }

        self.painter.draw_cells(&cells, &Rect { start_row: 0, start_col: 0, end_row: layout.size.rows - 1, end_col: layout.size.cols - 1 });
    }

    fn move_cursor(&mut self, program_id: String, _: vterm_sys::Pos, _: bool) {
        trace!("move_cursor for program {}", program_id);
        // find offset from state
        // painter.move_cursor(pos, is_visible));
    }

    fn layout_swap(&mut self, layout: Arc<RwLock<layout::Screen>>) {
        self.layout = layout;
    }
}
