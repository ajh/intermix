extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;

use std::os::unix::io::RawFd;
use std::io;
use term::terminfo::*;

pub struct Window {
    something: bool,
}

impl Window {
    pub fn new() -> Window {
        Window { something: true }
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
