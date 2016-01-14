use std::io::prelude::*;
use std::io;

// let welcome_handler = InputHandler::new();
// welcome_handler.default('enter_command_mode');
//
// let command_handler = InputHandler::new();
// command_handler.commands.push('up arrow', 'selection_up');
// command_handler.commands.push('right arrow', 'selection_right');
// command_handler.commands.push('i', 'focus_selection');
// command_handler.commands.push('c', 'new_program');
// command_handler.commands.push('x', 'kill_program');
//
// let program_handler = InputHandler::new();
// let program_handler.default('send_to_server');
// let program_handler.command.push("ctrl-b", 'switch_to_program_command_handler');
// let program_handler.escape_character("ctrl-b");
//
// let program_command_handler = InputHandler::new();
// let program_handler.default('unknown_command_and_switch_to_program_handler');
// let program_handler.command.push("esc", 'enter_command_mode');
//
// # How is this stuff used?
//
// // in stdin thread
// stdio().read() {
//   _ => main_thread_tx.send(UserInput { bytes: bytes })
// }
//
// // in main thread
// match rx.recv() {
//   ...
//   UserInput(bytes) { self.mode.write(bytes) }
//
// // in mode
//
// fn write(...) {
//   while bytes {
//     self.input_handler.write(bytes)
//     for action in self.input_handler.actions {
//       match action {
//         // do stuff like: change input handler, switch modes, send input to server, update
//         // status line, etc
//       }
//     }
//   }
// }

#[derive(Clone, Debug)]
pub struct InputHandler {
    bytes: Vec<u8>,
    key: u8,
    key_found: bool,
    is_possible: bool,
}

impl InputHandler {
    pub fn new(key: u8) -> InputHandler {
        InputHandler {
            bytes: vec![],
            key: key,
            key_found: false,
            is_possible: false,
        }
    }

    pub fn key_found(&self) -> bool {
        self.key_found
    }
}

impl Read for InputHandler {
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

impl Write for InputHandler {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut read = 0;

        for byte in buf.iter() {
            let is_escape = self.is_possible && *byte == self.key;
            if self.is_possible && !is_escape {
                self.key_found = true;
                self.bytes.pop();
                break;
            }
            else if is_escape {
                self.is_possible = false;
            }
            else {
                self.is_possible = *byte == self.key;
            }

            read += 1;
            self.bytes.push(*byte);
        }

        Ok(read)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

mod tests {
    use super::*;
    use std::io::prelude::*;

    const B_KEY: u8 = 98; // ascii code

    #[test]
    fn it_can_read_and_write() {
        let mut io = InputHandler::new(B_KEY);
        io.write("hi there".as_bytes());

        let mut output: Vec<u8> = vec![];
        io.read_to_end(&mut output);
        assert_eq!(output.as_slice(), "hi there".as_bytes());
    }

    #[test]
    fn it_can_do_partial_reads() {
        let mut io = InputHandler::new(B_KEY);
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
        let mut io = InputHandler::new(B_KEY);
        io.write("hi there".as_bytes());
        assert!(!io.key_found());
    }

    #[test]
    fn key_found_returns_true_when_key_pressed() {
        let mut io = InputHandler::new(B_KEY);
        io.write("hi b there".as_bytes());
        assert!(io.key_found());
    }

    #[test]
    fn key_found_returns_true_when_key_pressed_among_reads() {
        let mut io = InputHandler::new(B_KEY);
        let mut out = vec![];

        io.write("h".as_bytes());
        io.read_to_end(&mut out);
        io.write("b".as_bytes());
        io.read_to_end(&mut out);
        io.write("h".as_bytes());

        assert!(io.key_found());
    }

    #[test]
    fn key_found_returns_false_when_key_escaped() {
        let mut io = InputHandler::new(B_KEY);
        io.write("hi bb there".as_bytes());
        assert!(!io.key_found());
    }

    #[test]
    fn key_found_returns_false_when_key_escaped_among_reads() {
        let mut io = InputHandler::new(B_KEY);
        let mut out = vec![];

        io.write("h".as_bytes());
        io.read_to_end(&mut out);
        io.write("b".as_bytes());
        io.read_to_end(&mut out);
        io.write("b".as_bytes());
        io.read_to_end(&mut out);
        io.write("h".as_bytes());

        assert!(!io.key_found());
    }

    #[test]
    fn key_found_returns_false_when_key_escaped_across_writes() {
        let mut io = InputHandler::new(B_KEY);
        io.write("hi b".as_bytes());
        io.write("b there".as_bytes());
        assert!(!io.key_found());
    }

    #[test]
    fn key_found_returns_false_when_key_at_end() {
        let mut io = InputHandler::new(B_KEY);
        io.write("hi b".as_bytes());
        assert!(!io.key_found());
    }

    #[test]
    fn read_only_returns_bytes_before_key_found() {
        let mut io = InputHandler::new(B_KEY);
        io.write("hi b there".as_bytes());
        let mut buf = [0u8; 4];
        io.read(&mut buf);
        assert_eq!(&buf, b"hi \0");
    }

    #[test]
    fn write_stops_when_key_found() {
        let mut io = InputHandler::new(B_KEY);
        let res = io.write("hi b there".as_bytes());
        assert_eq!(res.unwrap(), 4);
    }
}