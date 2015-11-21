mod msg_listener;
mod window;

pub use self::msg_listener::*;
pub use self::window::*;

use libvterm_sys::*;
use std::sync::mpsc::Receiver;

pub enum WindowMsg {
    Damage {
        program_id: String,
        cells: Vec<ScreenCell>,
    },
    MoveCursor {
        program_id: String,
        new: Pos,
        old: Pos,
        is_visible: bool,
    },
    SbPushLine {
        program_id: String,
        cells: Vec<ScreenCell>,
    },
    AddProgram {
        program_id: String,
        rx: Receiver<WindowMsg>,
    },
}
