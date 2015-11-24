use libvterm_sys;
use std::sync::mpsc::*;

/// Represents the state of the client. Each thread will maintain their own representation which
/// will stay in sync through message passing.
#[derive(Default)]
pub struct State {
    pub windows: Vec<Window>,
    pub servers: Vec<Server>,
    pub mode_name: String,
}

/// The window or tty that the user sees
#[derive(Default)]
pub struct Window {
    pub id: String,
    pub panes: Vec<Pane>,
    pub size: libvterm_sys::ScreenSize,
}

/// a rectange within the window that displays output from a program
#[derive(Default)]
pub struct Pane {
    pub id: String,
    pub size: libvterm_sys::ScreenSize,
    pub offset: libvterm_sys::Pos,
    pub program_id: String,
}

/// A connection to an intermix server
pub struct Server {
    pub id: String,
    pub programs: Vec<Program>,

    /// replace with with cap'n proto or whatever
    pub tx: Sender<::server::ServerMsg>,
}

/// A program running on the server
#[derive(Default)]
pub struct Program {
    pub id: String,
    /// Whether the client is interested in msgs about this program. If its not visible, the answer
    /// is probably no.
    pub is_subscribed: bool
}

mod tests {
    use super::*;

    //#[test]
    //fn truth() {
        //assert!(true);
    //}
}
