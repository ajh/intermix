use std::io::prelude::*;
use std::io;
use std::sync::{Arc, Mutex};

pub struct CaptureIO {
    pub bytes: Arc<Mutex<Vec<u8>>>,
}

impl CaptureIO {
    pub fn new() -> (CaptureIO, Arc<Mutex<Vec<u8>>>) {
        let bytes = Arc::new(Mutex::new(Vec::new()));
        let io = CaptureIO { bytes: bytes.clone() };
        (io, bytes)
    }
}

impl Write for CaptureIO {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes = self.bytes.lock().unwrap();
        for byte in buf {
            bytes.push(*byte);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
