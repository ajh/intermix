extern crate log4rs;
#[macro_use]
extern crate log;
extern crate pty;
extern crate termios;
extern crate libvterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;

use std::thread;

mod window;
mod program;
mod pane;
mod tty_painter;

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

fn main() {
    log4rs::init_file(
        &std::env::current_dir().unwrap().join("log4rs.toml"),
        log4rs::toml::Creator::default()
    ).unwrap();

    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    info!("{:?}", args);

    info!("starting window");
    let mut window = window::Window::new();
    window.start();

    info!("starting program");
    let mut command_and_args = args.arg_command.clone();
    // TODO: use env to get SHELL variable here
    if command_and_args.len() == 0 { command_and_args.push("bash".to_string()); }
    let (program, attachments) = program::Program::new(&command_and_args);

    let pane = pane::Pane::new(0, 0, attachments.event_rx);

    window.panes.push(pane);
    window.spawn_drawing_thr();

    info!("joining threads");
    for thr in attachments.thread_handles {
        thr.join().unwrap();
    }

    info!("stopping window");
    // This doesn't really reset the terminal when using direct draw, because the program being run
    // will have done whatever random stuff to it
    window.stop();
}
