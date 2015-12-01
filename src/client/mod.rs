pub mod draw_worker;
pub mod modal;
pub mod stdin_read_worker;
pub mod tty_painter;
pub mod server_worker;
pub mod main_worker;
pub mod state;

use self::draw_worker::*;
use self::main_worker::*;
use self::server_worker::*;
use self::state::*;
use self::stdin_read_worker::*;
use std::sync::mpsc::*;
use std::thread::{self, JoinHandle};
use std::io::prelude::*;
use vterm_sys;

#[derive(Clone, Debug)]
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
    ProgramUpdate { server_id: String, program: Program },
    ProgramRemove { server_id: String, program_id: String },
    ProgramDamage { program_id: String, cells: Vec<vterm_sys::ScreenCell> },
    ProgramMoveCursor { program_id: String, new: vterm_sys::Pos, old: vterm_sys::Pos, is_visible: bool },

    UserInput { bytes: Vec<u8> },
}

pub struct Client {
    /// TODO: remove this
    is_or_is_not_true: bool
}

impl Client {
    pub fn spawn<F: 'static + Write + Send>(io: F) -> (Sender<ClientMsg>, JoinHandle<()>) {
        let (draw_tx, _) = DrawWorker::spawn(io);
        let (main_tx, main_handle) = MainWorker::spawn(draw_tx.clone());
        let (server_tx, _) = ServerWorker::spawn(main_tx.clone(), draw_tx.clone());
        StdinReadWorker::spawn(main_tx.clone());

        (server_tx, main_handle)
    }

    fn new(rx: Receiver<ClientMsg>) -> Client {
        Client { is_or_is_not_true: true }
    }
}
