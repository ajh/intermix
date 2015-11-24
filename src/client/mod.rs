mod draw_worker;
mod input_worker;
mod state;
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

    StateWindowAdd { window: Window },
    StateWindowUpdate { window: Window },
    StateWindowRemove { window_id: String },

    StatePaneAdd { window_id: String, pane: Pane },
    StatePaneUpdate { window_id: String, pane: Pane },
    StatePaneRemove { window_id: String, pane_id: String },

    StateServerAdd { server: Server },
    StateServerUpdate { server: Server },
    StateServerRemove { server_id: String },

    StateProgramAdd { server_id: String, program: Program },
    StateProgramUpdate { server_id: String, program: Window },
    StateProgramRemove { server_id: String, program_id: String },

    InputBytes { bytes: Vec<u8> },

    ProgramDamage { program_id: String, cells: Vec<libvterm_sys::ScreenCell> },
    ProgramMoveCursor { program_id: String, new: libvterm_sys::Pos, old: libvterm_sys::Pos, is_visible: bool },
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
    pub fn spawn() -> JoinHandle<()> {
        info!("spawning client");
        thread::spawn(move || {
            let mut client = Client::new();
            client.spawn_workers();
            client.enter_listener_loop();
            info!("exiting client");
        })
    }

    fn new() -> Client {
        let (tx, rx) = channel::<ClientMsg>();

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
    }
}
