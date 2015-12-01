use std::io::prelude::*;
use std::io;
use std::sync::{Arc, Mutex};

/// A object that implements the Read and Write trait using shared memory. It is also cloneable. It
/// can be used as a test double when a Reader or Writer is needed. For example:
///
/// # Examples
///
/// ```
/// let test_io = TestIO::new();
/// ```
/// upass it in to the code
/// under test that needs a reader and the test can control
///
/// It is cloneable over a shared byte vector. Useful for controlling code
/// under test reads from it. Adding bytes to the shared byte vector will make them available to be
/// read.
#[derive(Clone)]
pub struct TestIO {
    bytes: Arc<Mutex<Vec<u8>>>,
    pos: usize,
}

impl TestIO {
    /// Create a new test reader. The instance and a pointer to the shared byte vector are
    /// returned.
    pub fn new() -> TestIO {
        TestIO {
            bytes: Arc::new(Mutex::new(Vec::new())),
            pos: 0,
        }
    }
}

impl Read for TestIO {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes = self.bytes.lock().unwrap();

        if self.pos >= bytes.len() {
            // not sure if this is correct. I don't see an EOF error in std::io::ErrorKind
            return Ok(0)
        }

        let mut end_index = self.pos + buf.len();
        if end_index > bytes.len() { end_index = bytes.len() }

        for (i, byte) in bytes[self.pos..end_index].iter().enumerate() {
            buf[i] = *byte;
        }

        let bytes_read = end_index - self.pos;
        self.pos = end_index;
        Ok(bytes_read)
    }
}

impl Write for TestIO {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes = self.bytes.lock().unwrap();
        bytes.extend(buf.iter());
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
        let mut io = TestIO::new();
        io.write("hi there".as_bytes());

        let mut output: Vec<u8> = vec![];
        io.read_to_end(&mut output);
        assert_eq!(output.as_slice(), "hi there".as_bytes());
    }

    #[test]
    fn it_can_do_partial_reads() {
        let mut io = TestIO::new();
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
}
