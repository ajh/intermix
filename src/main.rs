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

use std::thread;
use libvterm_sys::*;

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
    let args: Args = docopt::Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());

    info!("{:?}", args);
    args
}

fn main() {
    setup_logging();
    let args: Args = parse_args();

    let (server_tx, server_handle) = server::Server::spawn();
    let (client_tx, client_handle) = client::Client::spawn();

    client_tx.send(client::ClientMsg::ServerAdd {
        server: ::client::state::Server {
            id: "todo".to_string(),
            tx: server_tx.clone(),
            programs: vec![],
        }
    });

    server_tx.send(server::ServerMsg::ClientAdd {
        client: ::server::Client {
            id: "todo".to_string(),
            tx: client_tx,
        }
    });

    let threads = vec![server_handle, client_handle];
    for thr in threads {
        thr.join().unwrap();
    }
}
