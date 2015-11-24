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
//mod server;

//pub use client::window::Window;
//pub use server::program::Program;
//pub use server::*;
//pub use client::user_input::*;

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

    let mut threads: Vec<thread::JoinHandle<()>> = vec![];

    //let mut server = Server::new();
    //let mut thrs = server.start_new_window();
    //threads.append(&mut thrs);

    //let screen_size = ScreenSize {
        //rows: 24,
        //cols: 80,
    //};

    //info!("starting program");
    //let mut command_and_args = args.arg_command.clone();
    //// TODO: use env to get SHELL variable here
    //if command_and_args.len() == 0 {
        //command_and_args.push("bash".to_string());
    //}
    //let mut more_threads = server.start_program_in_new_pane(&command_and_args,
                                                            //&screen_size,
                                                            //&Pos { row: 0, col: 0 });
    //threads.append(&mut more_threads);

    //info!("starting another program");
    //let mut more_threads = server.start_program_in_new_pane(&vec!["bash".to_string()],
                                                            //&screen_size,
                                                            //&Pos { row: 24, col: 0 });
    //threads.append(&mut more_threads);

    //spawn_stdin_to_pty_thr(server.first_program_pty_fd());

    //info!("joining threads");
    //for thr in threads {
        //thr.join().unwrap();
    //}

    //server.stop();
}
