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
    tx: Sender<ClientMsg>,
    client_tx: Sender<ClientMsg>,
    state: State,
    painter: TtyPainter,
}

impl DrawWorker {
    pub fn spawn(client_tx: Sender<ClientMsg>) -> (Sender<ClientMsg>, thread::JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();
        let tx_clone = tx.clone();

        info!("spawning draw worker");
        let handle = thread::spawn(move || {
            let mut worker = DrawWorker::new(client_tx, tx, rx);
            worker.enter_listen_loop();
            info!("exiting draw worker");
        });

        (tx_clone, handle)
    }

    fn new(client_tx: Sender<ClientMsg>, tx: Sender<ClientMsg>, rx: Receiver<ClientMsg>) -> DrawWorker {
        DrawWorker {
            rx: rx,
            tx: tx,
            client_tx: client_tx,
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
                ClientMsg::WindowAdd { window } => self.state.add_window(window),
                ClientMsg::PaneAdd { window_id , pane } => self.state.add_pane(&window_id, pane),
                ClientMsg::ServerAdd { server } => self.state.add_server(server),
                ClientMsg::ProgramAdd { server_id, program } => self.state.add_program(&server_id, program),
                ClientMsg::ProgramDamage { program_id, cells } => self.damage(program_id, cells),
                ClientMsg::ProgramMoveCursor { program_id, old, new, is_visible } => self.move_cursor(program_id, new, is_visible),

                _ => {}
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
