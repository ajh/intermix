extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;

use std::os::unix::io::RawFd;

// https://github.com/ruby/ruby/blob/trunk/ext/io/console/console.c
pub fn set_raw_mode(fd: RawFd) {
    let mut t = termios::Termios::from_fd(fd).unwrap();
    t.c_iflag &= !(termios::IGNBRK|termios::BRKINT|termios::PARMRK|termios::ISTRIP|termios::INLCR|termios::IGNCR|termios::ICRNL|termios::IXON);
    t.c_oflag &= !termios::OPOST;
    t.c_lflag &= !(termios::ECHO|termios::ECHOE|termios::ECHOK|termios::ECHONL|termios::ICANON|termios::ISIG|termios::IEXTEN);
    t.c_cflag &= !(termios::CSIZE|termios::PARENB);
    t.c_cflag |= termios::CS8;
    termios::tcsetattr(fd, termios::TCSANOW, &t).unwrap();
}

pub fn set_cooked_mode(fd: RawFd) {
    let mut t = termios::Termios::from_fd(fd).unwrap();
    t.c_iflag |= termios::BRKINT|termios::ISTRIP|termios::ICRNL|termios::IXON;
    t.c_oflag |= termios::OPOST;
    t.c_lflag |= termios::ECHO|termios::ECHOE|termios::ECHOK|termios::ECHONL|termios::ICANON|termios::ISIG|termios::IEXTEN;
    termios::tcsetattr(fd, termios::TCSANOW, &t).unwrap();
}
