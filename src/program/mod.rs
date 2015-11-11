mod event_handler;
mod program;
pub use self::event_handler::*;
pub use self::program::*;
use libvterm_sys::ScreenCell;
use std::sync::mpsc::Receiver;

pub enum ProgramEvent {
    Damage { program_id: String, cells: Vec<ScreenCell> },
    AddProgram { program_id: String, rx: Receiver<ProgramEvent> }
}
