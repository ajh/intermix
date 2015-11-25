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

pub struct InputWorker {
    rx: Receiver<ClientMsg>,
    tx: Sender<ClientMsg>,
    client_tx: Sender<ClientMsg>,
    state: State,
    mode: Box<::client::modal::Mode>,
}

impl InputWorker {
    pub fn spawn(client_tx: Sender<ClientMsg>) -> (Sender<ClientMsg>, thread::JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();
        let tx_clone = tx.clone();

        info!("spawning input worker");
        let handle = thread::spawn(move || {
            let mut worker = InputWorker::new(client_tx, tx, rx);
            worker.enter_listen_loop();
            info!("exiting input worker");
        });

        (tx_clone, handle)
    }

    fn new(client_tx: Sender<ClientMsg>, tx: Sender<ClientMsg>, rx: Receiver<ClientMsg>) -> InputWorker {
        let mode = Box::new(::client::modal::ProgramMode {
            program_id: "todo".to_string()
        });

        InputWorker {
            rx: rx,
            tx: tx,
            client_tx: client_tx,
            state: Default::default(),
            mode: mode,
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
                ClientMsg::ProgramAdd { server_id, program } => self.program_add(server_id, program),
                ClientMsg::UserInput { bytes } => self.mode.input(bytes, &self.state),
                _ => {}
            }
        }
    }

    fn program_add(&mut self, server_id: String, program: Program) {
        self.state.add_program(&server_id, program);
    }
}
