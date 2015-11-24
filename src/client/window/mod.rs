extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;

use std::os::unix::io::RawFd;
use std::io;
use term::terminfo::*;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use ::client::pane::*;

use self::msg_listener::*;

use libvterm_sys::*;
use std::sync::mpsc::Receiver;

mod msg_listener;

pub enum WindowMsg {
    Damage {
        program_id: String,
        cells: Vec<ScreenCell>,
    },
    MoveCursor {
        program_id: String,
        new: Pos,
        old: Pos,
        is_visible: bool,
    },
    SbPushLine {
        program_id: String,
        cells: Vec<ScreenCell>,
    },
    AddProgram {
        program_id: String,
        rx: Receiver<WindowMsg>,
    },
}
/// A window has panes, each of which can have a program
///
/// For now, we'll setup all the panes first, then call spawn so we don't have to deal with
/// selecting on a changable list of channel receivers.
pub struct Window {
    pub panes: Vec<Pane>,
    // This'll be WindowEvents at some point
    pub tx: mpsc::Sender<WindowMsg>,
}

impl Window {
    pub fn new() -> (Arc<Mutex<Window>>, Vec<thread::JoinHandle<()>>) {
        let (tx, rx) = mpsc::channel();
        let mut threads = vec![];

        let window = Arc::new(Mutex::new(Window {
            panes: vec![],
            tx: tx,
        }));

        let mut msg_listener = msg_listener::MsgListener::new(Arc::downgrade(&window.clone()));
        msg_listener.receivers.push(Box::new(rx));
        threads.push(msg_listener.spawn());

        (window, threads)
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
        t.c_iflag &= !(termios::IGNBRK | termios::BRKINT | termios::PARMRK | termios::ISTRIP |
                       termios::INLCR |
                       termios::IGNCR | termios::ICRNL | termios::IXON);
        t.c_oflag &= !termios::OPOST;
        t.c_lflag &= !(termios::ECHO | termios::ECHOE | termios::ECHOK | termios::ECHONL |
                       termios::ICANON | termios::ISIG | termios::IEXTEN);
        t.c_cflag &= !(termios::CSIZE | termios::PARENB);
        t.c_cflag |= termios::CS8;
        termios::tcsetattr(fd, termios::TCSANOW, &t).unwrap();
    }

    fn set_cooked_mode(&self, fd: RawFd) {
        let mut t = termios::Termios::from_fd(fd).unwrap();
        t.c_iflag |= termios::BRKINT | termios::ISTRIP | termios::ICRNL | termios::IXON;
        t.c_oflag |= termios::OPOST;
        t.c_lflag |= termios::ECHO | termios::ECHOE | termios::ECHOK | termios::ECHONL |
                     termios::ICANON | termios::ISIG | termios::IEXTEN;
        termios::tcsetattr(fd, termios::TCSANOW, &t).unwrap();
    }
}
