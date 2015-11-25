extern crate log;
extern crate pty;
extern crate termios;
extern crate vterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;
extern crate uuid;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::thread::{self, JoinHandle};
use std::os::unix::prelude::*;
use std::sync::mpsc::*;
use super::*;

// The way this works on the server side, is there is a special enum for passing the io bytes,
// called VteWorkerMsg. Maybe I should reuse that instead of ClientMsg, because only the input
// worker cares or is expected to handle an InputByte message.
pub struct StdinReadWorker {
    client_tx: Sender<ClientMsg>,
}

impl StdinReadWorker {
    pub fn spawn(tx: Sender<ClientMsg>) -> JoinHandle<()> {
        info!("spawning stdin reader thread");
        thread::spawn(move || {
            let mut worker = StdinReadWorker {
                client_tx: tx,
            };
            worker.enter_read_loop();
            info!("exiting stdin reader thread");
        })
    }

    fn enter_read_loop(&mut self) {
        let mut buf = [0 as u8; 4096];
        let mut io = io::stdin();

        loop {
            match io.read(&mut buf) {
                Ok(num_bytes) => {
                    if num_bytes == 0 {
                        break;
                    };

                    // This could be the worst way to do this
                    let mut bytes: Vec<u8> = vec!();
                    for byte in &buf[0..num_bytes] {
                        bytes.push(*byte)
                    }

                    let msg = ClientMsg::UserInput { bytes: bytes };

                    let result = self.client_tx.send(msg);
                    if result.is_err() {
                        error!("Sender failed to send");
                        break;
                    }
                }
                Err(msg) => {
                    error!("{}", msg);
                    break;
                }
            }
        }
    }
}
