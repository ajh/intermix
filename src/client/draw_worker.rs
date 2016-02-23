use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Arc, RwLock};
use super::*;
use super::tty_painter::*;
use super::layout::*;
use vterm_sys::{self, Pos, ScreenSize};

type Cell = vterm_sys::ScreenCell;

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
        DrawWorker {
            rx: rx,
            painter: TtyPainter::new(io),
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
                ClientMsg::ProgramDamage { program_id, cells } => {
                    self.program_damage(program_id, cells)
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

    fn program_damage(&mut self, program_id: String, cells: Vec<vterm_sys::ScreenCell>) {
        trace!("program_damage for {}", program_id);

        let layout = self.layout.read().unwrap();
        if let Some(wrap) = layout.tree().values().find(|w| *w.name() == program_id) {
            self.painter.draw_cells(&cells,
                                    &Pos {
                                        row: wrap.computed_y().unwrap(),
                                        col: wrap.computed_x().unwrap(),
                                    });
        } else {
            warn!("didnt find node with value: {:?}", program_id);
        }
    }

    fn layout_damage(&mut self) {
        trace!("layout_damage");

        let layout = self.layout.read().unwrap();
        // trace!("{:#?}", layout.tree());

        let mut cells: Vec<Cell> = vec![];

        for wrap in layout.tree().values() {
            self.border_cells_for_node(&mut cells, wrap, &layout.size);
        }

        self.painter.draw_cells(&cells, &Pos { row: 0, col: 0 });
    }

    fn border_cells_for_node(&self,
                             cells: &mut Vec<Cell>,
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

            cells.push(Cell {
                pos: Pos {
                    row: top,
                    col: left,
                },
                chars: vec!['┌'],
                ..Default::default()
            });
            cells.push(Cell {
                pos: Pos {
                    row: top,
                    col: right,
                },
                chars: vec!['┐'],
                ..Default::default()
            });
            cells.push(Cell {
                pos: Pos {
                    row: bottom,
                    col: left,
                },
                chars: vec!['└'],
                ..Default::default()
            });
            cells.push(Cell {
                pos: Pos {
                    row: bottom,
                    col: right,
                },
                chars: vec!['┘'],
                ..Default::default()
            });

            for x in left + 1..right {
                cells.push(Cell {
                    pos: Pos { row: top, col: x },
                    chars: vec!['─'],
                    ..Default::default()
                });
                cells.push(Cell {
                    pos: Pos {
                        row: bottom,
                        col: x,
                    },
                    chars: vec!['─'],
                    ..Default::default()
                });
            }

            for y in top + 1..bottom {
                cells.push(Cell {
                    pos: Pos {
                        row: y,
                        col: left,
                    },
                    chars: vec!['│'],
                    ..Default::default()
                });
                cells.push(Cell {
                    pos: Pos {
                        row: y,
                        col: right,
                    },
                    chars: vec!['│'],
                    ..Default::default()
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
                cells.push(Cell {
                    pos: Pos { row: top, col: x },
                    chars: vec![' '],
                    ..Default::default()
                });
                cells.push(Cell {
                    pos: Pos {
                        row: bottom,
                        col: x,
                    },
                    chars: vec![' '],
                    ..Default::default()
                });
            }

            for y in top + 1..bottom {
                cells.push(Cell {
                    pos: Pos {
                        row: y,
                        col: left,
                    },
                    chars: vec![' '],
                    ..Default::default()
                });
                cells.push(Cell {
                    pos: Pos {
                        row: y,
                        col: right,
                    },
                    chars: vec![' '],
                    ..Default::default()
                });
            }
        }
    }

    fn clear(&mut self) {
        let layout = self.layout.read().unwrap();
        let mut cells: Vec<Cell> = vec![];
        for row in 0..layout.size.rows {
            for col in 0..layout.size.cols {
                cells.push(Cell {
                    pos: Pos {
                        row: row,
                        col: col,
                    },
                    chars: vec![' '],
                    ..Default::default()
                });
            }
        }

        self.painter.draw_cells(&cells, &Pos { row: 0, col: 0 });
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
