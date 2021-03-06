use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use super::*;

/// A worker that reads from a program's pty and sends msgs.
///
/// An alternative would be to have 1 thread run for all programs using mio.
pub struct PtyReader {
    #[allow(dead_code)]
    program_id: String,
    pty: File,
    vte_tx: mpsc::Sender<VteWorkerMsg>,
}

impl PtyReader {
    pub fn spawn(io: File,
                 vte_tx: mpsc::Sender<VteWorkerMsg>,
                 program_id: &str)
                 -> thread::JoinHandle<()> {
        let program_id = program_id.to_string();
        info!("spawning pty reader for program {}", program_id);
        thread::spawn(move || {
            let mut reader = PtyReader::new(io, vte_tx, &program_id);
            reader.enter_listen_loop();
            info!("exiting pty reader for program {}", program_id);
        })
    }

    fn new(io: File, vte_tx: mpsc::Sender<VteWorkerMsg>, program_id: &str) -> PtyReader {
        PtyReader {
            pty: io,
            vte_tx: vte_tx,
            program_id: program_id.to_string(),
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
                Err(e) => {
                    self.vte_tx.send(VteWorkerMsg::PtyReadError).unwrap();
                    error!("error reading from pty: {}", e.description());
                    break;
                }
            };

            let mut bytes_vec: Vec<u8> = vec![];
            bytes_vec.extend(bytes);
            let msg = VteWorkerMsg::PtyRead { bytes: bytes_vec };
            self.vte_tx.send(msg).unwrap();
        }
    }
}
