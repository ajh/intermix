extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;
extern crate libvterm_sys;

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
    // offset within its window
    pub offset: libvterm_sys::Pos,
}

impl Pane {
    pub fn new(offset: &libvterm_sys::Pos) -> Pane {
        Pane { offset: offset.clone() }
    }
}
