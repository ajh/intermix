use libvterm_sys::*;
use std::sync::mpsc::Receiver;

mod event_handler;
mod program;

pub use self::event_handler::*;
pub use self::program::*;

pub enum ProgramEvent {
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
    AddProgram {
        program_id: String,
        rx: Receiver<ProgramEvent>,
    },
}
