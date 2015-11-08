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

pub struct Window {
    event_receivers: Vec<Receiver<::program::ProgramEvent>>,
}

impl Window {
    pub fn new() -> Window {
        Window {
            event_receivers: vec!(),
        }
    }

    // just loop over the one receiver, deal with multiple receivers and changes to what receivers
    // we have later
    pub fn spawn_thr(&self) {
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

fn handle_event(event: ::program::ProgramEvent, painter: &mut ::tty_painter::TtyPainter) {
    match event {
        ::program::ProgramEvent::Damage{cells} => painter.draw_cells(&cells, &mut io::stdout()),
    }
}
