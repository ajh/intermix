#[macro_use] extern crate log;
extern crate docopt;
extern crate libc;
extern crate libintermix;
extern crate log4rs;
extern crate rustc_serialize;
extern crate term;
extern crate termios;

use libc::c_ushort;
use std::io;
use std::os::unix::io::RawFd;

const USAGE: &'static str = "
intermix - a terminal emulator multiplexer

Usage:
intermix [<command>...]
intermix -h | --help

Options:
-h --help      Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_command: Vec<String>,
}

fn setup_logging() {
    log4rs::init_file(&std::env::current_dir().unwrap().join("log4rs.toml"),
                      log4rs::toml::Creator::default())
        .unwrap();
}

fn parse_args() -> Args {
    docopt::Docopt::new(USAGE).and_then(|d| d.decode())
                              .unwrap_or_else(|e| e.exit())
}

fn main() {
    setup_logging();
    parse_args();
    set_raw_mode(0);

    let (server_tx, server_handle) = libintermix::server::Server::spawn();


    let tty_ioctl_config: libintermix::client::TtyIoCtlConfig;
    unsafe {
        let mut size: WinSize = std::mem::zeroed();
        libc::ioctl(1, TIOCGWINSZ, &mut size);
        tty_ioctl_config = libintermix::client::TtyIoCtlConfig { rows: size.rows, cols: size.cols, ..Default::default() };
    }
    let (client_tx, _) = libintermix::client::Client::spawn(io::stdin(), io::stdout(), tty_ioctl_config);

    client_tx.send(libintermix::client::ClientMsg::ServerAdd {
        server: libintermix::client::servers::Server {
            id: "some server".to_string(),
            tx: server_tx.clone(),
            programs: vec![],
        }
    }).unwrap();

    server_tx.send(libintermix::server::ServerMsg::ClientAdd {
        client: libintermix::server::Client {
            id: "some client".to_string(),
            tx: client_tx.clone(),
        }
    }).unwrap();

    let threads = vec![server_handle];
    for thr in threads {
        thr.join().unwrap();
    }
}

// https://github.com/ruby/ruby/blob/trunk/ext/io/console/console.c
fn set_raw_mode(fd: RawFd) {
    let mut t = termios::Termios::from_fd(fd).unwrap();
    termios::cfmakeraw(&mut t);
    termios::tcsetattr(fd, termios::TCSADRAIN, &t).unwrap();
}

#[derive(Debug)]
#[repr(C)]
struct WinSize {
    rows: c_ushort,
    cols: c_ushort,
    x_pixels: c_ushort,
    y_pixels: c_ushort
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
const TIOCGWINSZ: libc::c_ulong = 0x40087468;
#[cfg(any(target_os = "linux", target_os = "android"))]
const TIOCGWINSZ: libc::c_ulong = 0x5413;
