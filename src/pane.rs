extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;

use std::os::unix::io::RawFd;
use std::io::prelude::*;
use std::io;
use term::terminfo::*;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};

/// A window has panes, each of which can have a program
///
/// For now, we'll setup all the panes first, then call spawn so we don't have to deal with
/// selecting on a changable list of channel receivers.
pub struct Pane {
    // row offset relative to its window
    pub row: u16,
    // col offset relative to its window
    pub col: u16,
    pub program_event_rx: Option<Receiver<::program::ProgramEvent>>,
}

impl Pane {
    pub fn new(row: u16, col: u16, rx: Receiver<::program::ProgramEvent>) -> Pane {
        Pane {
            row: row,
            col: col,
            program_event_rx: Some(rx)
        }
    }
}
