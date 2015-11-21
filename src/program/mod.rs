mod msg_listener;
mod program;
mod pty_reader;

use self::msg_listener::*;
pub use self::program::*;
use libvterm_sys::*;

pub enum ProgramMsg {
    PtyRead { bytes: Vec<u8> },
    PtyReadError,
    PtyReadZero,
    RequestRedrawRect { rect: Rect },
}
