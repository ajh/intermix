#![feature(mpsc_select)]
#![feature(convert)]

extern crate docopt;
extern crate libc;
extern crate libvterm_sys;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate pty;
extern crate rustc_serialize;
extern crate term;
extern crate termios;
extern crate uuid;

use libvterm_sys::*;
use std::io;
use std::os::unix::io::RawFd;
use std::sync::mpsc::*;
use std::thread;
use term::terminfo::*;


mod client;
mod server;

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

    let (server_tx, server_handle) = server::Server::spawn();
    let (client_tx, client_handle) = client::Client::spawn();

    client_tx.send(client::ClientMsg::ServerAdd {
        server: ::client::state::Server {
            id: "some server".to_string(),
            tx: server_tx.clone(),
            programs: vec![],
        }
    });
    add_initial_window_to_client(&client_tx);

    server_tx.send(server::ServerMsg::ClientAdd {
        client: ::server::Client {
            id: "some client".to_string(),
            tx: client_tx.clone(),
        }
    });

    pretend_a_mode_starts_a_program(&client_tx, &server_tx);

    let threads = vec![server_handle, client_handle];
    for thr in threads {
        thr.join().unwrap();
    }

    set_cooked_mode(0);
}

fn add_initial_window_to_client(client_tx: &Sender<client::ClientMsg>) {
    client_tx.send(client::ClientMsg::WindowAdd {
        window: client::state::Window {
            id: "initial window".to_string(),
            .. Default::default()
        }
    }).unwrap();
}

// TODO: Move this code into a mode
fn pretend_a_mode_starts_a_program(client_tx: &Sender<client::ClientMsg>, server_tx: &Sender<server::ServerMsg>) {
    let command_and_args: Vec<String> = vec!["bash".to_string()];
    server_tx.send(server::ServerMsg::ProgramStart {
        command_and_args: command_and_args,
        program_id: "bash-123".to_string(),
    }).unwrap();

    client_tx.send(client::ClientMsg::PaneAdd {
        window_id: "initial window".to_string(),
        pane: client::state::Pane {
            id: "pane-bash-123".to_string(),
            size: libvterm_sys::ScreenSize { rows: 24, cols: 80 },
            offset: libvterm_sys::Pos { row: 0, col: 10 },
            program_id: "bash-123".to_string(),
        }
    });

    client_tx.send(client::ClientMsg::ProgramAdd {
        server_id: "some server".to_string(),
        program: client::state::Program {
            id: "bash-123".to_string(),
            is_subscribed: true,
        }
    });
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
