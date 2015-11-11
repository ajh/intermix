extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;

mod event_handler;

use std::os::unix::io::RawFd;
use std::io::prelude::*;
use std::io;
use term::terminfo::*;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};

pub use self::event_handler::*;

/// A window has panes, each of which can have a program
///
/// For now, we'll setup all the panes first, then call spawn so we don't have to deal with
/// selecting on a changable list of channel receivers.
pub struct Window {
    pub panes: Vec<::pane::Pane>,
}

impl Window {
    pub fn new() -> Window {
        Window {
            panes: vec!(),
        }
    }

    pub fn start(&self) {
        self.set_raw_mode(0);
        let mut tty = TerminfoTerminal::new(io::stdout()).unwrap();
        tty.apply_cap("smcup", &[]).unwrap();
    }

    pub fn stop(&self) {
        let mut tty = TerminfoTerminal::new(io::stdout()).unwrap();
        tty.apply_cap("rmcup", &[]).unwrap();
        self.set_cooked_mode(0);
    }

    // https://github.com/ruby/ruby/blob/trunk/ext/io/console/console.c
    fn set_raw_mode(&self, fd: RawFd) {
        let mut t = termios::Termios::from_fd(fd).unwrap();
        t.c_iflag &= !(termios::IGNBRK|termios::BRKINT|termios::PARMRK|termios::ISTRIP|termios::INLCR|termios::IGNCR|termios::ICRNL|termios::IXON);
        t.c_oflag &= !termios::OPOST;
        t.c_lflag &= !(termios::ECHO|termios::ECHOE|termios::ECHOK|termios::ECHONL|termios::ICANON|termios::ISIG|termios::IEXTEN);
        t.c_cflag &= !(termios::CSIZE|termios::PARENB);
        t.c_cflag |= termios::CS8;
        termios::tcsetattr(fd, termios::TCSANOW, &t).unwrap();
    }

    fn set_cooked_mode(&self, fd: RawFd) {
        let mut t = termios::Termios::from_fd(fd).unwrap();
        t.c_iflag |= termios::BRKINT|termios::ISTRIP|termios::ICRNL|termios::IXON;
        t.c_oflag |= termios::OPOST;
        t.c_lflag |= termios::ECHO|termios::ECHOE|termios::ECHOK|termios::ECHONL|termios::ICANON|termios::ISIG|termios::IEXTEN;
        termios::tcsetattr(fd, termios::TCSANOW, &t).unwrap();
    }
}
