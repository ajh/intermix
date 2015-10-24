use std::io::Write;

use super::*;

pub fn draw_screen<T: Write>(screen: &Screen, writer: &mut T) {
    for row in &screen.cells {
        for cell in row {
            if cell.ch != '\x00' {
                let mut buf = [0 as u8; 4];
                match cell.ch.encode_utf8(&mut buf) {
                    Some(num_bytes) => {
                        writer.write(&buf[0..num_bytes]);
                    },
                    None => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate tsm_sys;

    use std::io::Write;
    use std::io;
    use std::str::*;
    use std::char::from_u32;
    use std::char;
    use std::io::{Error, ErrorKind};

    use super::*;
    use super::super::*;

    // implements Write trait and writes to a string
    struct StringIO {
        pub s: String
    }

    impl StringIO {
        fn new() -> StringIO {
            StringIO { s: String::new() }
        }

        fn string(&self) -> &str {
            &self.s
        }
    }

    impl Write for StringIO {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let mut string = String::new();

            for byte in buf {
                match char::from_u32(*byte as u32) {
                    Some(ch) => string.push(ch),
                    None => return Err(Error::new(ErrorKind::Other, "oh no!"))
                }
            }

            self.s.push_str(&string);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    // Implements write trait and writes to a vte which it owns. Then uses vte to update a screen
    // that it owns. The idea is that if vte interprets our output in a way that returns an
    // identical screen, then our output is good.
    struct VteIO {
        vte: tsm_sys::Vte,
        screen: Screen
    }

    impl VteIO {
        fn new(rows_count: usize, cols_count: usize) -> VteIO {
            VteIO {
                vte: tsm_sys::Vte::new(rows_count, cols_count).unwrap(),
                screen: Screen::new(rows_count, cols_count),
            }
        }
    }

    impl Write for VteIO {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.vte.input(buf);

            for cell in self.vte.screen.borrow_mut().cells() {
                println!("{} {}", cell.posy, cell.posx);
                self.screen.cells[cell.posy][cell.posx].ch = cell.ch
            };
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn it_draws_nothing_when_empty() {
        let mut output = StringIO::new();
        let mut screen = Screen::new(3, 3);
        draw_screen(&screen, &mut output);
        assert_eq!(output.string(), "");
    }

    #[test]
    fn it_correctly_draws_empty_screen() {
        let mut screen = Screen::new(3, 3);
        let mut vte_io = VteIO::new(3, 3);

        draw_screen(&screen, &mut vte_io);
        assert_eq!(vte_io.screen, screen)
    }

    #[test]
    fn it_correctly_draws_screen_with_some_chars() {
        let mut screen = Screen::new(3, 3);
        screen.cells[0][0].ch = 'l' as char;
        screen.cells[0][1].ch = 'l' as char;
        screen.cells[0][2].ch = 'o' as char;
        let mut vte_io = VteIO::new(3, 3);

        draw_screen(&screen, &mut vte_io);
        assert_eq!(vte_io.screen, screen)
    }
}
