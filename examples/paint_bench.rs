#[macro_use]
extern crate log;
extern crate docopt;
extern crate libc;
extern crate libintermix;
extern crate log4rs;
extern crate rustc_serialize;
extern crate term;
extern crate termios;
extern crate vterm_sys;

// cargo run --example paint_bench --release

use libc::c_ushort;
use libintermix::client::paint::TtyPainter;
use std::io::prelude::*;
use vterm_sys::{Size, Rect, Pos};

const USAGE: &'static str = "
paint_bench - benchmark for terminal painting

Usage:
paint_bench FILE
paint_bench -h | --help

Options:
-h --help      Show this screen
";

#[derive(Debug, RustcDecodable)]
#[allow(non_snake_case)]
struct Args {
    arg_FILE: String,
}

fn setup_logging() {
    log4rs::init_file(&std::env::current_dir().unwrap().join("log4rs.yaml"),
                      Default::default())
        .unwrap();
}

fn parse_args() -> Args {
    docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}

#[derive(Debug)]
#[repr(C)]
struct WinSize {
    rows: c_ushort,
    cols: c_ushort,
    x_pixels: c_ushort,
    y_pixels: c_ushort,
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
const TIOCGWINSZ: libc::c_ulong = 0x40087468;
#[cfg(any(target_os = "linux", target_os = "android"))]
const TIOCGWINSZ: libc::c_ulong = 0x5413;

fn create_cells<F: Read>(io: &mut F) -> Vec<vterm_sys::ScreenCell> {
    io.bytes().map(|byte| {
        // need to convert ln to cr
        vterm_sys::ScreenCell { chars: vec![byte.unwrap()], .. Default::default() }
    }).collect()
}

/// Things to benchmark
///
/// * drawing characters
/// * scrolling
///
fn main() {
    setup_logging();
    let args = parse_args();

    let size: Size = unsafe {
        let mut s: WinSize = std::mem::zeroed();
        libc::ioctl(1, TIOCGWINSZ, &mut s);
        Size {
            width: s.cols as usize,
            height: s.rows as usize,
        }
    };

    println!("{:?}", size);
    let mut painter = TtyPainter::new(::std::io::stdout(), size.clone());

    painter.flush();

    println!("{:?}", args.arg_FILE);

    let mut file = ::std::fs::File::open(args.arg_FILE).unwrap();
    let cells = create_cells(&mut file);

    painter.draw_cells(&cells, &Rect::new(Pos::new(0,0), size.clone()));
}
