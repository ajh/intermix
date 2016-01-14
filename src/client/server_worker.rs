use std::io;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Weak, Mutex};
use super::*;
use super::servers::*;
use super::tty_painter::*;

/// A worker that listens for msgs from a server, and dispatches them to either the main worker or
/// the draw worker.
pub struct ServerWorker {
    rx: Receiver<ClientMsg>,
    main_tx: Sender<ClientMsg>,
    draw_tx: Sender<ClientMsg>,
}

impl ServerWorker {
    pub fn spawn(main_tx: Sender<ClientMsg>, draw_tx: Sender<ClientMsg>) -> (Sender<ClientMsg>, thread::JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();
        let tx_clone = tx.clone();

        info!("spawning server worker");
        let handle = thread::spawn(move || {
            let mut worker = ServerWorker::new(main_tx, draw_tx, rx);
            worker.enter_listen_loop();
            info!("exiting server worker");
        });

        (tx_clone, handle)
    }

    fn new(main_tx: Sender<ClientMsg>, draw_tx: Sender<ClientMsg>, rx: Receiver<ClientMsg>) -> ServerWorker {
        ServerWorker {
            rx: rx,
            main_tx: main_tx,
            draw_tx: draw_tx,
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
                ClientMsg::ServerAdd { .. } => self.forward_to_main_worker(msg),
                ClientMsg::ServerUpdate { .. } => self.forward_to_main_worker(msg),
                ClientMsg::ServerRemove { .. } => self.forward_to_main_worker(msg),

                ClientMsg::ProgramAdd { .. } => self.forward_to_main_worker(msg),
                ClientMsg::ProgramUpdate { .. } => self.forward_to_main_worker(msg),
                ClientMsg::ProgramRemove { .. } => self.forward_to_main_worker(msg),

                ClientMsg::ProgramDamage { .. } => self.forward_to_draw_worker(msg),
                ClientMsg::ProgramMoveCursor { .. } => self.forward_to_draw_worker(msg),

                _ => warn!("unhandled msg {:?}", msg),
            }
        }
    }

    fn forward_to_main_worker(&self, msg: ClientMsg) {
        self.main_tx.send(msg).unwrap();
    }

    fn forward_to_draw_worker(&self, msg: ClientMsg) {
        self.draw_tx.send(msg).unwrap();
    }
}
