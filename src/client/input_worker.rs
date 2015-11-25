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

pub struct InputWorker {
    rx: Receiver<ClientMsg>,
    tx: Sender<ClientMsg>,
    client_tx: Sender<ClientMsg>,
    state: State,
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
        InputWorker {
            rx: rx,
            tx: tx,
            client_tx: client_tx,
            state: Default::default(),
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
                ClientMsg::UserInput { bytes } => {
                    // TODO: send the bytes to the selected mode

                    // for now, send it to the first program
                    if let Some(server) = self.state.servers.first() {
                        if let Some(program) = server.programs.first() {
                            trace!("sending input to program {}", program.id);
                            server.tx.send(::server::ServerMsg::ProgramInput {
                                program_id: program.id.clone(),
                                bytes: bytes,
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
