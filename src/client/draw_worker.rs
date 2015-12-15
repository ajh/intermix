use std::io;
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Arc, RwLock};
use super::*;
use super::state::*;
use super::tty_painter::*;
use super::grid::*;
use vterm_sys;

/// # todos
/// * [ ] make message enum more specific
/// * [ ] Maybe build with initial state to avoid bootstrap data race?
/// * [ ] optimize drawing when pane is full width (csr)
/// * [ ] implement move cursor
pub struct DrawWorker<F: 'static + Write + Send> {
    rx: Receiver<ClientMsg>,
    painter: TtyPainter<F>,
    windows: Windows,
    layout: Arc<RwLock<Screen>>,
}

impl <F: 'static + Write + Send> DrawWorker<F> {
    pub fn spawn(io: F, rx: Receiver<ClientMsg>, layout: Arc<RwLock<Screen>>) -> thread::JoinHandle<()> {
        info!("spawning draw worker");
        thread::spawn(move || {
            let mut worker = DrawWorker::new(rx, io, layout);
            worker.enter_listen_loop();
            info!("exiting draw worker");
        })
    }

    fn new(rx: Receiver<ClientMsg>, io: F, layout: Arc<RwLock<Screen>>) -> DrawWorker<F> {
        DrawWorker {
            rx: rx,
            windows: Default::default(),
            painter: TtyPainter::new(io),
            layout: layout
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
                ClientMsg::ProgramDamage { program_id, cells } => self.damage(program_id, cells),
                ClientMsg::ProgramMoveCursor { program_id, old, new, is_visible } => self.move_cursor(program_id, new, is_visible),
                _ => warn!("unhandled msg {:?}", msg)
            }
        }
    }

    fn damage(&mut self, program_id: String, cells: Vec<vterm_sys::ScreenCell>) {
        trace!("damage for program {}", program_id);

        let screen = self.layout.read().unwrap();
        if let Some(widget) = screen.root.as_ref().unwrap().widgets().find(|w| w.program_id == program_id) {
            self.painter.draw_cells(&cells, &widget.pos);
        }
        else {
            trace!("didnt find widget {}", program_id);
        }
    }

    fn move_cursor(&mut self, program_id: String, pos: vterm_sys::Pos, is_visible: bool) {
        trace!("move_cursor for program {}", program_id);
        // find offset from state
        // painter.move_cursor(pos, is_visible));
    }
}
