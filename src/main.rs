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

mod window;
mod program;

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
    let window = window::Window::new();
    window.start();

    info!("forking");
    let chile = program::fork(&args.arg_command);

    info!("starting threads");
    let mut threads = vec!();
    threads.push(program::spawn_stdin_to_pty_thr(&chile));
    threads.push(program::spawn_pty_to_stdout_thr(&chile));

    info!("joining threads");
    for thr in threads {
        thr.join().unwrap();
    }

    info!("stopping window");
    // This doesn't really reset the terminal when using direct draw, because the program being run
    // will have done whatever random stuff to it
    window.stop();
}
