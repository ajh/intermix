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
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;
use super::*;

pub struct PtyReader {
    pty: File,
    tx: mpsc::Sender<ProgramMsg>,
}

impl PtyReader {
    pub fn new(io: File, tx: mpsc::Sender<ProgramMsg>) -> PtyReader {
        PtyReader {
            pty: io,
            tx: tx,
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        info!("spawning pty reader");
        thread::spawn(move || {
            let mut buf = [0 as u8; 4096];
            let mut reader = BufReader::new(&self.pty);

            info!("starting pty -> stdout thread");
            loop {
                // block until read
                let bytes = match reader.read(&mut buf) {
                    Ok(num_bytes) => {
                        if num_bytes == 0 {
                            self.tx.send(ProgramMsg::PtyReadZero).unwrap();
                            error!("zero bytes reading from pty");
                            break;
                        }
                        &buf[0..num_bytes]
                    }
                    Err(_) => {
                        self.tx.send(ProgramMsg::PtyReadError).unwrap();
                        error!("error reading from pty");
                        break;
                    }
                };

                let mut bytes_vec: Vec<u8> = vec!();
                // TODO: fix this lameness
                for byte in bytes { bytes_vec.push(byte.clone()) };
                let event = ProgramMsg::PtyRead { bytes: bytes_vec };
                self.tx.send(event).unwrap();
            }
            info!("ending pty -> stdout thr");
        })
    }
}
