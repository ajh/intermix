extern crate pty;
extern crate tsm_sys;
extern crate termios;

mod program;

use program::*;
use std::io;
use std::io::Write;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

// https://github.com/ruby/ruby/blob/trunk/ext/io/console/console.c
fn set_raw_mode(fd: RawFd) {
    let mut t = termios::Termios::from_fd(fd).unwrap();
    t.c_iflag &= !(termios::IGNBRK|termios::BRKINT|termios::PARMRK|termios::ISTRIP|termios::INLCR|termios::IGNCR|termios::ICRNL|termios::IXON);
    t.c_oflag &= !termios::OPOST;
    t.c_lflag &= !(termios::ECHO|termios::ECHOE|termios::ECHOK|termios::ECHONL|termios::ICANON|termios::ISIG|termios::IEXTEN);
    t.c_cflag &= !(termios::CSIZE|termios::PARENB);
    t.c_cflag |= termios::CS8;
    termios::tcsetattr(fd, termios::TCSANOW, &t);
}

fn set_cooked_mode(fd: RawFd) {
    let mut t = termios::Termios::from_fd(fd).unwrap();
    t.c_iflag |= termios::BRKINT|termios::ISTRIP|termios::ICRNL|termios::IXON;
    t.c_oflag |= termios::OPOST;
    t.c_lflag |= termios::ECHO|termios::ECHOE|termios::ECHOK|termios::ECHONL|termios::ICANON|termios::ISIG|termios::IEXTEN;
    termios::tcsetattr(fd, termios::TCSANOW, &t);
}

const CtrlC: u8 = 0x03;

fn main() {
    set_raw_mode(0);

    let mut program = Program::new("date program".to_string(), "date".to_string());
    program.run().unwrap();

    loop {
            let mut output_buf = [0 as u8; 10];
            program.read(&mut output_buf);
            //println!("read from program {:?}", output_buf);

            io::stdout().write(&output_buf);
            io::stdout().flush().ok().expect("Could not flush stdout");

            let mut input_buf  = [0 as u8; 10];
            io::stdin().read(&mut input_buf);
            //println!("read from stdin {:?}", input_buf);
            if input_buf.iter().find(|&x| *x == CtrlC).is_some() {
                break;
            }
            program.write(&input_buf);
    }

    set_cooked_mode(0);
}
