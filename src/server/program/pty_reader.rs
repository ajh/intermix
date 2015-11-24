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

/// A worker that reads from a program's pty and sends msgs.
///
/// An alternative would be to have 1 thread run for all programs using mio.
pub struct PtyReader {
    pty: File,
    vte_tx: mpsc::Sender<VteWorkerMsg>,
}

impl PtyReader {
    pub fn spawn(io: File, vte_tx: mpsc::Sender<VteWorkerMsg>) -> thread::JoinHandle<()> {
        info!("spawning pty reader");
        thread::spawn(move || {
            let mut reader = PtyReader::new(io, vte_tx);
            reader.enter_listen_loop();
            info!("exiting pty reader");
        })
    }

    fn new(io: File, vte_tx: mpsc::Sender<VteWorkerMsg>) -> PtyReader {
        PtyReader {
            pty: io,
            vte_tx: vte_tx,
        }
    }

    pub fn enter_listen_loop(&mut self) {
        let mut buf = [0 as u8; 4096];
        let mut reader = BufReader::new(&self.pty);

        loop {
            // block until read
            let bytes = match reader.read(&mut buf) {
                Ok(num_bytes) => {
                    if num_bytes == 0 {
                        self.vte_tx.send(VteWorkerMsg::PtyReadZero).unwrap();
                        error!("zero bytes reading from pty");
                        break;
                    }
                    &buf[0..num_bytes]
                }
                Err(_) => {
                    self.vte_tx.send(VteWorkerMsg::PtyReadError).unwrap();
                    error!("error reading from pty");
                    break;
                }
            };

            let mut bytes_vec: Vec<u8> = vec!();
            // TODO: fix this lameness
            for byte in bytes { bytes_vec.push(byte.clone()) };
            let msg = VteWorkerMsg::PtyRead { bytes: bytes_vec };
            self.vte_tx.send(msg).unwrap();
        }
    }
}
