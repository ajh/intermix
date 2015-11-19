mod event_handler;
mod program;
mod pty_reader;

pub use self::event_handler::*;
pub use self::program::*;

pub enum ProgramMsg {
    PtyRead { bytes: Vec<u8> },
    PtyReadZero,
    PtyReadError,
}
