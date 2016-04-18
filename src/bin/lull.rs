extern crate docopt;
extern crate libintermix;
extern crate log4rs;
#[macro_use]
extern crate log;
extern crate rustc_serialize;

use libintermix::lull::*;

const USAGE: &'static str = "
lull - Terminal session management and persistence

Usage:
lull
lull -h | --help

Options:
-h --help      Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
}

fn setup_logging() {
    log4rs::init_file(&std::env::current_dir()
                           .expect("couldn't get current dir")
                           .join("log4rs.yaml"),
                      Default::default())
        .expect("log4rs couldn't init");
}

fn parse_args() -> Args {
    docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}

fn main() {
    setup_logging();
    parse_args();

    println!("hi there");

    LullImpl::new().start();
}
