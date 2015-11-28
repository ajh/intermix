extern crate libintermix;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rustc_serialize;
extern crate term;
extern crate termios;

use std::io;
use std::os::unix::io::RawFd;
use term::terminfo::*;

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
    let args: Args = parse_args();
    set_raw_mode(0);

    let (server_tx, server_handle) = libintermix::server::Server::spawn();
    let (client_tx, client_handle) = libintermix::client::Client::spawn();

    client_tx.send(libintermix::client::ClientMsg::ServerAdd {
        server: libintermix::client::state::Server {
            id: "some server".to_string(),
            tx: server_tx.clone(),
            programs: vec![],
        }
    });

    server_tx.send(libintermix::server::ServerMsg::ClientAdd {
        client: libintermix::server::Client {
            id: "some client".to_string(),
            tx: client_tx.clone(),
        }
    });

    let threads = vec![server_handle, client_handle];
    for thr in threads {
        thr.join().unwrap();
    }

    set_cooked_mode(0);
}

// https://github.com/ruby/ruby/blob/trunk/ext/io/console/console.c
fn set_raw_mode(fd: RawFd) {
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

fn set_cooked_mode(fd: RawFd) {
    let mut t = termios::Termios::from_fd(fd).unwrap();
    t.c_iflag |= termios::BRKINT | termios::ISTRIP | termios::ICRNL | termios::IXON;
    t.c_oflag |= termios::OPOST;
    t.c_lflag |= termios::ECHO | termios::ECHOE | termios::ECHOK | termios::ECHONL |
                 termios::ICANON | termios::ISIG | termios::IEXTEN;
    termios::tcsetattr(fd, termios::TCSANOW, &t).unwrap();
}
