extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;

use std::os::unix::io::RawFd;
use std::io;

pub const CTRL_C: u8 = 0x03;

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

#[repr(C)]
struct WinSize {
    row_size: libc::c_ushort,     /* rows, in characters */
    col_size: libc::c_ushort,     /* columns, in characters */
    xpixel: libc::c_ushort,  /* horizontal size, pixels */
    ypixel: libc::c_ushort   /* vertical size, pixels */
}

/// Return (row_size: u32, col_size: u32) of the given file descriptors window size, using ioctl.
pub fn get_win_size(fd: RawFd) -> Result<(u32, u32), io::Error> {
    let mut win_size = WinSize { row_size: 0, col_size: 0, xpixel: 0, ypixel: 0 };
    let res = unsafe { ioctl::ioctl(fd, ioctl::TIOCGWINSZ, &mut win_size) };
    match res {
        0 => Ok((win_size.row_size as u32, win_size.col_size as u32)),
        _ => Err(io::Error::last_os_error())
    }
}

/// Set the given file descriptors window size, using ioctl.
pub fn set_win_size(fd: RawFd, row_size: u32, col_size: u32) -> Result<(), io::Error> {
    trace!("set_win_size({}, {}, {})", fd, row_size, col_size);
    let win_size = WinSize { row_size: row_size as libc::c_ushort, col_size: col_size as libc::c_ushort, xpixel: 0, ypixel: 0 };
    let res = unsafe { ioctl::ioctl(fd, ioctl::TIOCSWINSZ, &win_size) };
    match res {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error())
    }
}
