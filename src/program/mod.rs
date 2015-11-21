mod msg_listener;
mod program;
mod pty_reader;

pub use self::msg_listener::*;
pub use self::program::*;

pub enum ProgramMsg {
    PtyRead { bytes: Vec<u8> },
    PtyReadZero,
    PtyReadError,
}
