extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;
extern crate libvterm_sys;

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
            match self.rx.recv() {
                Ok(msg) => self.handle(msg),
                Err(_) => break,
            }
        }
    }

    fn handle(&mut self, event: ClientMsg) {
        match event {
            ClientMsg::ProgramDamage{program_id, cells} => self.damage(program_id, cells),
            ClientMsg::ProgramMoveCursor{program_id, old, new, is_visible} => self.move_cursor(program_id, new, is_visible),
            _ => {},
        }
        // when ClientMsg::InputBytes, send the bytes to the selected mode.
    }

    fn damage(&mut self, program_id: String, cells: Vec<libvterm_sys::ScreenCell>) {
        // find offset from state
        // self.painter.draw_cells(&cells, &mut io::stdout(), &offset);
    }

    fn move_cursor(&mut self, program_id: String, pos: libvterm_sys::Pos, is_visible: bool) {
        // find offset from state
        // painter.move_cursor(pos, is_visible));
    }
}