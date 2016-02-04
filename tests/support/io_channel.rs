use std::sync::mpsc::{channel, Sender, Receiver, SendError};
use std::io::prelude::*;
use std::io;

pub fn create() -> (Writer, Reader) {
    let (tx, rx) = channel();
    let writer = Writer { tx: tx };
    let reader = Reader { rx: rx };

    (writer, reader)
}

pub struct Reader {
    rx: Receiver<u8>,
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut i = 0;

        // block while receiving first byte
        match self.rx.recv() {
            Ok(byte) => {
                buf[i] = byte;
                i += 1;
            }
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, e));
            }
        }

        // without blocking try to receive additional bytes until we run out or buf is full
        while i < buf.len() {
            match self.rx.try_recv() {
                Ok(byte) => {
                    buf[i] = byte;
                    i += 1;
                }
                Err(_) => {
                    break;
                }
            }
        }

        Ok(i)
    }
}

#[derive(Clone)]
pub struct Writer {
    tx: Sender<u8>,
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut count = 0;
        let mut error: Option<SendError<u8>> = None;

        for byte in buf {
            match self.tx.send(*byte) {
                Ok(_) => count += 1,
                Err(e) => {
                    error = Some(e);
                    break;
                }
            }
        }

        if count == 0 && error.is_some() {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, error.unwrap()))
        } else {
            Ok(count)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

mod tests {
    #![allow(unused_imports)]

    use super::*;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn reader_reads_what_is_written_to_writer() {
        let (mut writer, mut reader) = create();

        writer.write("hi there".as_bytes()).unwrap();

        let mut buf = [0u8; 8];
        reader.read(&mut buf).unwrap();
        assert_eq!(buf, "hi there".as_bytes());
    }

    #[test]
    fn writer_returns_error_when_reader_is_dropped() {
        let (mut writer, reader) = create();
        drop(reader);
        assert!(writer.write("oops".as_bytes()).is_err());
    }

    #[test]
    fn reader_returns_error_when_writer_is_dropped() {
        let (writer, mut reader) = create();
        drop(writer);

        let mut buf = [0u8; 8];
        assert!(reader.read(&mut buf).is_err());
    }

    #[test]
    fn reader_will_block_on_read() {
        let (mut writer, reader) = create();

        let w_thr = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            writer.write("hi there\n".as_bytes()).unwrap();

            thread::sleep(Duration::from_millis(50));
            writer.write("hello again\n".as_bytes()).unwrap();
        });

        let r_thr = thread::spawn(move || {
            let mut reader = BufReader::new(reader);

            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            assert_eq!(buf, "hi there\n".to_string());

            buf.clear();
            reader.read_line(&mut buf).unwrap();
            assert_eq!(buf, "hello again\n".to_string());
        });

        r_thr.join().unwrap();
        w_thr.join().unwrap();
    }
}
