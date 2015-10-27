#![feature(unicode)]
extern crate log4rs;
#[macro_use]
extern crate log;
extern crate pty;
extern crate termios;
extern crate tsm_sys;
extern crate term;
extern crate time;

mod program;
mod terminfo;
mod screen;
mod window;

use program::*;
use std::io::{self, Read, Write};
use std::sync::mpsc::channel;
use std::thread;

fn setup_logging() {
    log4rs::init_file(
        &std::env::current_dir().unwrap().join("log4rs.toml"),
        log4rs::toml::Creator::default()
    ).unwrap();
}

fn main() {
    setup_logging();
    info!("starting up");

    let window = window::Window::new();
    window.start();

    let mut program = Program::new(
        "Some name".to_string(),
        "not implemented".to_string(),
        window.rows_count(),
        window.cols_count()
    );
    let (program_tx, program_rx) = program.run().unwrap();
    // Spawn thread to display program output
    thread::spawn(move || {
        // debug performance issue
        thread::sleep_ms(5000);

        let mut last_age: u32 = 0;

        loop {
            // block till we see something
            let result = program_rx.recv();
            if result.is_err() {
                break;
            }

            // Drain the receiver since we'll be drawing the must uptodate stuff
            loop { if program_rx.try_recv().is_err() { break; } }

            let start = time::now();
            let screen = program.screen.lock().unwrap();
            let stop = time::now();
            trace!("spent time getting lock {}", stop - start);
            last_age = screen::tty_painter::draw_screen(&screen, &mut io::stdout(), last_age);
        }
        info!("leaving program -> stdout thread");
    });

    // Main loop which blocks on user input
    info!("Starting main loop");
    let mut buf = [0 as u8; 1024];
    loop {
        match io::stdin().read(&mut buf) {
            Ok(num_bytes) => {
                if num_bytes == 0 { break };

                if buf.iter().find(|&x| *x == terminfo::CTRL_C).is_some() {
                    info!("CTRL_C detected");
                    break;
                }
                for byte in buf[0..num_bytes].into_iter() { program_tx.send(*byte).unwrap() }
            },
            Err(_) => break,
        }
    }

    info!("Ended main loop");
    window.stop();
}
