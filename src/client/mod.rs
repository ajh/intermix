pub mod paint;
pub mod layout;
pub mod main_worker;
pub mod modal;
pub mod servers;
pub mod stdin_read_worker;

use self::main_worker::*;
use self::servers::*;
use self::stdin_read_worker::*;
use std::io::prelude::*;
use std::sync::mpsc::*;
use vterm_sys;

#[derive(Clone, Debug)]
pub enum ClientMsg {
    Quit,

    ServerAdd {
        server: Server,
    },
    ServerUpdate {
        server: Server,
    },
    ServerRemove {
        server_id: String,
    },

    ProgramAdd {
        server_id: String,
        program_id: String,
    },
    ProgramUpdate {
        server_id: String,
        program_id: String,
    },
    ProgramRemove {
        server_id: String,
        program_id: String,
    },
    ProgramDamage {
        program_id: String,
        cells: Vec<vterm_sys::ScreenCell>,
        rect: vterm_sys::Rect,
    },
    ProgramMoveCursor {
        program_id: String,
        new: vterm_sys::Pos,
        old: vterm_sys::Pos,
        is_visible: bool,
    },

    UserInput {
        bytes: Vec<u8>,
    },

    Clear,

    LayoutDamage,
    LayoutSwap {
        layout: layout::Screen,
    },

    StatusLineDamage,
}

/// other settings from `man tty_ioctl` could live here
#[derive(Clone, PartialEq)]
pub struct TtyIoCtlConfig {
    pub rows: usize,
    pub cols: usize,
}

impl Default for TtyIoCtlConfig {
    fn default() -> TtyIoCtlConfig {
        TtyIoCtlConfig {
            rows: 24,
            cols: 80,
        }
    }
}

/// # TODO
/// * [ ] derive useful traits on stuff here
pub struct Client {
    main_tx: Sender<ClientMsg>,
}

impl Client {
    /// Create and start a client instance. Returns a Sender which can be used to send messages to
    /// the client, and a client instance which can be used to stop the client.
    ///
    /// All the action takes place in threads so message passing is the client api more or less.
    pub fn spawn<I, O>(input: I,
                       output: O,
                       tty_ioctl_config: TtyIoCtlConfig)
                       -> (Sender<ClientMsg>, Client)
        where I: 'static + Read + Send,
              O: 'static + Write + Send
    {
        let (main_tx, _) = MainWorker::spawn(tty_ioctl_config, output);
        StdinReadWorker::spawn(input, main_tx.clone());

        let client = Client {
            main_tx: main_tx.clone(),
        };

        (main_tx, client)
    }

    /// Stop the client. It consumes the client so it can't be restarted.
    pub fn stop(self) {
        self.main_tx.send(ClientMsg::Quit).unwrap();
    }

    pub fn tx(&self) -> &Sender<ClientMsg> {
        &self.main_tx
    }
}
