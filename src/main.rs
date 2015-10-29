#![feature(unicode)]
extern crate log4rs;
#[macro_use]
extern crate log;
extern crate pty;
extern crate termios;
extern crate tsm_sys;
extern crate term;
extern crate libc;

mod window;
mod terminfo;

use std::ffi::CString;
use std::io::{Read, Write, BufReader, BufWriter};
use std::io;
use std::ptr;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use term::terminfo::*;
use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd};

fn fork() -> pty::Child {
    match pty::fork() {
        Ok(child) => {
            if child.pid() == 0 {
                let mut ptrs = [
                    CString::new("vim").unwrap().as_ptr(),
                    CString::new("Cargo.toml").unwrap().as_ptr(),
                    ptr::null()
                ];

                print!(" "); // mysterious but pty uses it too

                let ret = unsafe { libc::execvp(*ptrs.as_ptr(), ptrs.as_mut_ptr()) };
                panic!("error {} in execvp {}", ret, io::Error::last_os_error());
            }
            else {
                info!("got vim child process");
                child
            }
        },
        Err(e) => {
            panic!("pty::fork error: {}", e);
        }
    }
}

fn spawn_stdin_thr(child: &pty::Child) {
    // thread for sending stdin to pty
    let mut pty = child.pty().unwrap().clone();
    thread::spawn(move || {
        let mut buf = [0 as u8; 1024];
        info!("starting stdin -> pty thread");
        loop {
            match io::stdin().read(&mut buf) {
                Ok(num_bytes) => {
                    if num_bytes == 0 { break };

                    //if buf.iter().find(|&x| *x == terminfo::CTRL_C).is_some() {
                        //info!("CTRL_C detected");
                        //exit();
                    //}

                    pty.write(&buf[0..num_bytes]);
                },
                Err(_) => break,
            }
        }
        info!("ending stdin -> pty thread");
    });
}

fn read_bytes_from_pty<'a, F: Read>(io: &mut F, buf: &'a mut [u8]) -> Result<&'a [u8], String> {
    // block waiting to read
    match io.read(buf) {
        Ok(num_bytes) => {
            if num_bytes == 0 {
                return Err("zero bytes reading from pty".to_string());
            }
            info!("read {} bytes", num_bytes);
            Ok(&buf[0..num_bytes])
        },
        Err(_) => Err("error reading from pty".to_string())
    }
}

fn draw_from_vte<F: Write>(bytes: &[u8], vte: &mut tsm_sys::Vte, io: &F, last_age: u32) -> u32 {

    // feed vte
    vte.input(bytes);

    // update the screen
    let age = vte.screen.borrow_mut().draw(|_, ch, _, _, x, y, age| {
        if last_age >= age {
            return;
        }

        if (ch as u32) < 32 {
            // unprintable
            return;
        }

        // move cursor
        let params = [ parm::Param::Number(y as i16), parm::Param::Number(x as i16) ];
        let mut tty = TerminfoTerminal::new(io::stdout()).unwrap();
        tty.apply_cap("cup", &params);

        // write character
        let mut buf = [0 as u8; 4];
        match ch.encode_utf8(&mut buf) {
            Some(num_bytes) => {
                io::stdout().write(&buf[0..num_bytes]);
            },
            None => {}
        }
    });

    age
}

fn draw_direct<F: Write>(bytes: &[u8], io: &mut F) {
    io.write(bytes);
    io.flush();
}

fn main() {
    log4rs::init_file(
        &std::env::current_dir().unwrap().join("log4rs.toml"),
        log4rs::toml::Creator::default()
    ).unwrap();
    info!("starting up");

    let window = window::Window::new();
    window.start();

    let vim_process = fork();
    spawn_stdin_thr(&vim_process);

    let mut buf = [0 as u8, 1024];
    let mut reader = unsafe { File::from_raw_fd(vim_process.pty().unwrap().as_raw_fd()) };
    let mut reader = BufReader::new(reader);

    let mut current_age: u32 = 0;
    let mut vte = tsm_sys::Vte::new(80, 24).unwrap();

    let mut writer = io::stdout();
    let mut writer = BufWriter::new(writer);

    info!("starting main loop");
    loop {
        let result = read_bytes_from_pty(&mut reader, &mut buf);
        if result.is_err() {
            error!("{}", result.err().unwrap());
            break;
        }
        let bytes = result.unwrap();

        if false {
            current_age = draw_from_vte(bytes, &mut vte, &writer, current_age);
        }

        else {
            draw_direct(bytes, &mut writer);
        }
    }

    window.stop();
}
