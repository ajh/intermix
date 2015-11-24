use std::rc::*;
use libvterm_sys::{ScreenSize, Pos};

/// Represents the state of the client. Each thread will maintain their own representation which
/// will stay in sync through message passing.
#[derive(Default)]
pub struct State {
    windows: Vec<Window>,
    servers: Vec<Server>,
    mode_name: String,
}

/// The window or tty that the user sees
pub struct Window {
    id: String,
    panes: Vec<Pane>,
    size: ScreenSize,
}

/// a rectange within the window that displays output from a program
pub struct Pane {
    id: String,
    size: ScreenSize,
    offset: Pos,
    program_id: String,
}

/// A connection to an intermix server
pub struct Server {
    id: String,
    programs: Vec<Program>,
}

/// A program running on the server
pub struct Program {
    id: String,
    /// Whether the client is interested in msgs about this program. If its not visible, the answer
    /// is probably no.
    is_subscribed: bool
}

mod tests {
    use super::*;

    //#[test]
    //fn truth() {
        //assert!(true);
    //}
}
