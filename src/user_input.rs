extern crate log;
extern crate pty;
extern crate termios;
extern crate libvterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;
extern crate uuid;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::thread;
use std::os::unix::prelude::*;

/// This thread should really read from a receiver, not from stdin
pub fn spawn_stdin_to_pty_thr(pty: RawFd) -> thread::JoinHandle<()> {
    // thread for sending stdin to pty
    thread::spawn(move || {
        let mut file = unsafe { File::from_raw_fd(pty) };
        let mut buf = [0 as u8; 4096];
        info!("starting stdin -> pty thread");
        loop {
            match io::stdin().read(&mut buf) {
                Ok(num_bytes) => {
                    if num_bytes == 0 { break };

                    //if buf.iter().find(|&x| *x == terminfo::CTRL_C).is_some() {
                        //info!("CTRL_C detected");
                        //exit();
                    //}

                    match file.write_all(&buf[0..num_bytes]) {
                        Ok(_) => {},
                        Err(msg) => {
                            error!("{}", msg);
                            break;
                        },
                    }
                },
                Err(msg) => {
                    error!("{}", msg);
                    break;
                },
            }
        }
        info!("ending stdin -> pty thread");
    })
}
