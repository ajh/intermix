extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;
extern crate vterm_sys;

use std::io;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Weak, Mutex};
use super::*;
use super::state::*;
use super::tty_painter::*;

pub struct DrawWorker {
    rx: Receiver<ClientMsg>,
    painter: TtyPainter,
    state: State,
}

impl DrawWorker {
    pub fn spawn() -> (Sender<ClientMsg>, thread::JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();

        info!("spawning draw worker");
        let handle = thread::spawn(move || {
            let mut worker = DrawWorker::new(rx);
            worker.enter_listen_loop();
            info!("exiting draw worker");
        });

        (tx, handle)
    }

    fn new(rx: Receiver<ClientMsg>) -> DrawWorker {
        DrawWorker {
            rx: rx,
            state: Default::default(),
            painter: Default::default(),
        }
    }

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
                ClientMsg::WindowAdd { window } => self.state.add_window(window),
                ClientMsg::PaneAdd { window_id, pane } => self.state.add_pane(&window_id, pane),
                _ => warn!("unhandled msg {:?}", msg)
            }
        }
    }

    fn damage(&mut self, program_id: String, cells: Vec<vterm_sys::ScreenCell>) {
        trace!("damage for program {}", program_id);

        let mut panes = self.state.windows.iter().flat_map(|w| w.panes.iter());
        if let Some(pane) = panes.find(|p| p.program_id == program_id) {
            self.painter.draw_cells(&cells, &mut io::stdout(), &pane.offset);
        } else {
            trace!("no pane for program {:?}", program_id);
        }
    }

    fn move_cursor(&mut self, program_id: String, pos: vterm_sys::Pos, is_visible: bool) {
        trace!("move_cursor for program {}", program_id);
        // find offset from state
        // painter.move_cursor(pos, is_visible));
    }
}
