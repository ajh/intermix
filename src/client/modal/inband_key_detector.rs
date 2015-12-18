use std::io::prelude::*;
use std::io;

#[derive(Clone)]
pub struct InbandKeyDetector {
    bytes: Vec<u8>,
    key: char,
}

impl InbandKeyDetector {
    pub fn new(key: char) -> InbandKeyDetector {
        InbandKeyDetector {
            bytes: vec![],
            key: key,
        }
    }

    pub fn key_found(&self) -> bool {
        false
    }
}

impl Read for InbandKeyDetector {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut end_index = buf.len();
        if end_index > self.bytes.len() { end_index = self.bytes.len() }


        let tail = self.bytes.split_off(end_index);

        for (i, byte) in self.bytes.iter().enumerate() {
            buf[i] = *byte;
        }

        self.bytes = tail;
        Ok(end_index)
    }
}

impl Write for InbandKeyDetector {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.bytes.extend(buf.iter());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

mod tests {
    use super::*;
    use std::io::prelude::*;

    #[test]
    fn it_can_read_and_write() {
        let mut io = InbandKeyDetector::new('b');
        io.write("hi there".as_bytes());

        let mut output: Vec<u8> = vec![];
        io.read_to_end(&mut output);
        assert_eq!(output.as_slice(), "hi there".as_bytes());
    }

    #[test]
    fn it_can_do_partial_reads() {
        let mut io = InbandKeyDetector::new('b');
        io.write("hi there".as_bytes());

        let mut buf = [0u8; 2];
        io.read(&mut buf);
        assert_eq!(&buf, b"hi");

        io.read(&mut buf);
        assert_eq!(&buf, b" t");

        io.read(&mut buf);
        assert_eq!(&buf, b"he");

        io.read(&mut buf);
        assert_eq!(&buf, b"re");
    }

    #[test]
    fn key_found_returns_false_when_key_not_found() {
        let mut io = InbandKeyDetector::new('b');
        io.write("hi there".as_bytes());
        assert!(!io.key_found());
    }

    #[test]
    fn key_found_returns_true_when_key_found() {
        let mut io = InbandKeyDetector::new('b');
        io.write("hi b there".as_bytes());
        assert!(io.key_found());
    }

    #[test]
    fn key_found_returns_false_when_key_escaped() {
        let mut io = InbandKeyDetector::new('b');
        io.write("hi bb there".as_bytes());
        assert!(!io.key_found());
    }

    #[test]
    fn key_found_returns_false_when_key_escaped_across_writes() {
        let mut io = InbandKeyDetector::new('b');
        io.write("hi b".as_bytes());
        io.write("b there".as_bytes());
        assert!(!io.key_found());
    }

    #[test]
    fn read_only_returns_bytes_before_key_found() {
    }

    #[test]
    fn bytes_after_key_found_can_be_read() {
    }
}
