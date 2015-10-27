#![feature(unicode)]
extern crate log4rs;
#[macro_use]
extern crate log;
extern crate pty;
extern crate termios;
extern crate tsm_sys;
extern crate term;

mod program;
mod terminfo;
mod screen;
mod window;

use program::*;
use std::io::{Read, Write};
use std::io;
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

    // TODO: move this to window
    //let (rows_count, cols_count) = terminfo::get_win_size(0).unwrap();

    let mut program = Program::new(
        "Some name".to_string(),
        "not implemented".to_string(),
        24 as usize,
        80 as usize
    );
    let (program_tx, program_rx) = program.run().unwrap();
    let (control_tx, control_rx) = channel::<usize>();

    // Spawn thread to display program output
    thread::spawn(move || {
        loop {
            if control_rx.try_recv().is_ok() {
                info!("shutdown signal in channel -> pty thread");
                break;
            }

            match program_rx.recv() {
                Ok(_) => {
                    let mut screen = program.screen.lock().unwrap();
                    screen::tty_painter::draw_screen(&mut screen, &mut io::stdout());
                },
                Err(_) => break,
            };
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
    control_tx.send(1).unwrap();
    window.stop();
}
