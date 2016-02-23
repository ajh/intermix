use std::io::prelude::*;
use std::thread::{self, JoinHandle};
use std::os::unix::prelude::*;
use std::sync::mpsc::*;
use super::*;

// The way this works on the server side, is there is a special enum for passing the io bytes,
// called VteWorkerMsg. Maybe I should reuse that instead of ClientMsg, because only the input
// worker cares or is expected to handle an InputByte message.
pub struct StdinReadWorker<F: 'static + Read + Send> {
    client_tx: Sender<ClientMsg>,
    io: F,
}

impl<F: 'static + Read + Send> StdinReadWorker<F> {
    pub fn spawn(io: F, tx: Sender<ClientMsg>) -> JoinHandle<()> {
        info!("spawning stdin reader thread");
        thread::spawn(move || {
            let mut worker = StdinReadWorker {
                client_tx: tx,
                io: io,
            };
            worker.enter_read_loop();
            info!("exiting stdin reader thread");
        })
    }

    /// start reading from stdin. Exits when a read fails.
    fn enter_read_loop(&mut self) {
        let mut buf = [0 as u8; 4096];
        loop {
            match self.io.read(&mut buf) {
                Ok(num_bytes) => {
                    if num_bytes == 0 {
                        break;
                    };

                    let mut bytes: Vec<u8> = vec![];
                    bytes.extend(&buf[0..num_bytes]);

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
