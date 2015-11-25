mod draw_worker;
mod input_worker;
mod modal;
mod stdin_read_worker;
mod tty_painter;
pub mod state;

use vterm_sys;
use self::draw_worker::*;
use self::input_worker::*;
use self::state::*;
use self::stdin_read_worker::*;
use std::sync::mpsc::*;
use std::thread::{self, JoinHandle};

#[derive(Clone)]
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
    ProgramDamage { program_id: String, cells: Vec<vterm_sys::ScreenCell> },
    ProgramMoveCursor { program_id: String, new: vterm_sys::Pos, old: vterm_sys::Pos, is_visible: bool },

    ModeUpdate { mode: Mode },

    UserInput { bytes: Vec<u8> },
}

pub struct Client {
    rx: Receiver<ClientMsg>,
    tx: Sender<ClientMsg>,
    state: State,

    /// We copy any received client state msgs to our threads, so they can keep their state in
    /// sync. This is the list of Senders for the threads.
    worker_txs: Vec<Sender<ClientMsg>>,
}

impl Client {
    pub fn spawn() -> (Sender<ClientMsg>, JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();
        let tx_clone = tx.clone();

        info!("spawning client");
        let handle = thread::spawn(move || {
            let mut client = Client::new(tx, rx);
            client.spawn_workers();
            client.init();
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
            worker_txs: vec!(),
        }
    }

    fn spawn_workers(&mut self) {
        // spawn input thr
        let (input_worker_tx, handle) = InputWorker::spawn(self.tx.clone());
        self.worker_txs.push(input_worker_tx.clone());

        // spawn stdin reader
        let handle = StdinReadWorker::spawn(input_worker_tx);

        // spawn draw thr
        let(draw_worker_tx, handle) = DrawWorker::spawn(self.tx.clone());
        self.worker_txs.push(draw_worker_tx);
    }

    fn enter_listener_loop(&mut self) {
        loop {
            let msg = match self.rx.recv() {
                Ok(msg) => msg,
                Err(_) => break,
            };

            match msg {
                ClientMsg::Quit => {
                    self.broadcast_msg(msg, false);
                    break;
                },
                ClientMsg::UserInput { bytes: _ } => {},

                // this catches ProgramDamage etc, which the draw worker needs but the input worker
                // doesnt
                _ => self.broadcast_msg(msg, true),
            }
        }
    }

    fn broadcast_msg(&self, msg: ClientMsg, hard: bool) {
        trace!("broadcasting (non-debuggable ClientMsg) to {} worker senders", self.worker_txs.len());
        for tx in &self.worker_txs {
            let result = tx.send(msg.clone());
            if hard { result.expect("didnt send") }
        }
    }

    fn init(&self) {
        // This'll send to ourselves to be picked up in the event listen loop
        self.tx.send(ClientMsg::WindowAdd {
            window: Window {
                id: "initial_window".to_string(),
                .. Default::default()
            }
        }).unwrap();

        self.tx.send(ClientMsg::PaneAdd {
            window_id: "initial_window".to_string(),
            pane: Pane {
                id: "status_line".to_string(),
                size: vterm_sys::ScreenSize { rows: 1, cols: 80 },
                offset: vterm_sys::Pos { row: 24, col: 0 },
                program_id: "".to_string(),
            }
        });

        self.tx.send(ClientMsg::ModeUpdate { mode: Mode { id: "program".to_string() } }).unwrap();
    }
}
