pub mod draw_worker;
pub mod modal;
pub mod stdin_read_worker;
pub mod tty_painter;
pub mod server_worker;
pub mod main_worker;
pub mod state;
pub mod grid;

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

/// other settings from `man tty_ioctl` could live here
#[derive(Clone, PartialEq)]
pub struct TtyIoCtlConfig {
    pub rows: u16,
    pub cols: u16,
}

impl Default for TtyIoCtlConfig {
    fn default() -> TtyIoCtlConfig {
        TtyIoCtlConfig { rows: 24, cols: 80 }
    }
}

/// # TODO
/// * [ ] derive useful traits on stuff here
pub struct Client {
    draw_tx: Sender<ClientMsg>,
    main_tx: Sender<ClientMsg>,
    server_tx: Sender<ClientMsg>,
}

impl Client {
    /// Create and start a client instance. Returns a Sender which can be used to send messages to
    /// the client, and a client instance which can be used to stop the client.
    ///
    /// All the action takes place in threads so message passing is the client api more or less.
    pub fn spawn<I, O>(input: I, output: O, tty_ioctl_config: TtyIoCtlConfig) -> (Sender<ClientMsg>, Client)
        where I: 'static + Read + Send, O: 'static + Write + Send {
        let (draw_tx, draw_rx) = channel::<ClientMsg>();
        let (main_tx, layout, main_handle) = MainWorker::spawn(draw_tx.clone(), tty_ioctl_config);
        DrawWorker::spawn(output, draw_rx, layout);
        let (server_tx, _) = ServerWorker::spawn(main_tx.clone(), draw_tx.clone());
        StdinReadWorker::spawn(input, main_tx.clone());

        let client = Client {
            draw_tx: draw_tx,
            main_tx: main_tx,
            server_tx: server_tx.clone(),
        };

        (server_tx, client)
    }

    /// Stop the client. It consumes the client so it can't be restarted.
    pub fn stop(self) {
        self.draw_tx.send(ClientMsg::Quit);
        self.main_tx.send(ClientMsg::Quit);
        self.server_tx.send(ClientMsg::Quit);
    }
}
