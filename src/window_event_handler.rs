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
pub struct WindowEventHandler {
    // deal with Program Events for now, until we have window events implemented
    rx: Receiver<::program::ProgramEvent>
}

impl WindowEventHandler {
    pub fn new(rx: Receiver<::program::ProgramEvent>) -> WindowEventHandler {
        WindowEventHandler {rx: rx}
    }

    // just loop over the one receiver, deal with multiple receivers and changes to what receivers
    // we have later
    pub fn spawn(self) {
        // assume only one rx for now
        thread::spawn(move || {
            let mut painter: ::tty_painter::TtyPainter = Default::default();

            loop {
                match self.rx.recv().unwrap() {
                    ::program::ProgramEvent::Damage{cells} => painter.draw_cells(&cells, &mut io::stdout(), &libvterm_sys::Pos { row: 10, col: 5 }),
                }
            }
        });
    }
}
