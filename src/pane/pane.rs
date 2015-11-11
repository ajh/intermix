extern crate libvterm_sys;

use libvterm_sys::*;

/// A window has panes, each of which can have a program
///
/// For now, we'll setup all the panes first, then call spawn so we don't have to deal with
/// selecting on a changable list of channel receivers.
pub struct Pane {
    // The size of this pane
    pub size: ScreenSize,

    /// offset within its window. Really, the window should now the pane's offsets. The pane should
    /// just know its size.
    pub offset: Pos,

    /// This is temporary, really it should have a reference to the program
    pub program_id: String,
}

impl Pane {
    pub fn new(size: &ScreenSize, offset: &Pos, program_id: &str) -> Pane {
        Pane { size: size.clone(), offset: offset.clone(), program_id: program_id.to_string() }
    }
}
