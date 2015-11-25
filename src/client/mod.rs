mod draw_worker;
mod input_worker;
pub mod state;
mod stdin_read_worker;
mod tty_painter;

use libvterm_sys;
use self::draw_worker::*;
use self::input_worker::*;
use self::state::*;
use self::stdin_read_worker::*;
use std::sync::mpsc::*;
use std::thread::{self, JoinHandle};

pub enum ClientMsg {
    Quit,

    WindowAdd { window: Window },
    WindowUpdate { window: Window },
    WindowRemove { window_id: String },

    PaneAdd { window_id: String, pane: Pane },
    PaneUpdate { window_id: String, pane: Pane },
    PaneRemove { window_id: String, pane_id: String },

    ServerAdd { server: Server },
    ServerUpdate { server: Server },
    ServerRemove { server_id: String },

    ProgramAdd { server_id: String, program: Program },
    ProgramUpdate { server_id: String, program: Window },
    ProgramRemove { server_id: String, program_id: String },
    ProgramDamage { program_id: String, cells: Vec<libvterm_sys::ScreenCell> },
    ProgramMoveCursor { program_id: String, new: libvterm_sys::Pos, old: libvterm_sys::Pos, is_visible: bool },

    InputBytes { bytes: Vec<u8> },
}

pub struct Client {
    rx: Receiver<ClientMsg>,
    tx: Sender<ClientMsg>,
    state: State,

    /// We copy any received client state msgs to our threads, so they can keep their state in
    /// sync. This is the list of Senders for the threads.
    thr_txs: Vec<Sender<ClientMsg>>,
}

impl Client {
    pub fn spawn() -> (Sender<ClientMsg>, JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();
        let tx_clone = tx.clone();

        info!("spawning client");
        let handle = thread::spawn(move || {
            let mut client = Client::new(tx, rx);
            client.spawn_workers();
            client.enter_listener_loop();
            info!("exiting client");
        });

        (tx_clone, handle)
    }

    fn new(tx: Sender<ClientMsg>, rx: Receiver<ClientMsg>) -> Client {
        Client {
            rx: rx,
            tx: tx,
            state: Default::default(),
            thr_txs: vec!(),
        }
    }

    fn spawn_workers(&mut self) {
        // spawn input thr
        let (input_worker_tx, handle) = InputWorker::spawn(self.tx.clone());

        // spawn stdin reader
        let handle = StdinReadWorker::spawn(input_worker_tx);

        // spawn draw thr
    }

    fn enter_listener_loop(&mut self) {
        loop {
            match self.rx.recv() {
                Ok(msg) => self.handle(msg),
                Err(_) => break,
            }
        }
    }

    fn handle(&self, msg: ClientMsg) {
    }
}
