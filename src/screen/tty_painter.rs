extern crate term;

use std::io::Write;

use super::*;

pub fn draw_screen<T: Write>(screen: &Screen, writer: &mut T) {
    let (last_x, last_y) = (0, 0);
    //let tty = term::terminfo::TerminfoTerminal::new(writer);

    for row in &screen.cells {
        for cell in row {
            if cell.ch != '\x00' {
                //if last_x + 1 != cell.x {
                    //let terminfo.strings.get('cup').unwrap();
                        //if let Some(cmd) = self.ti.strings.get(cmd) {
                            //if let Ok(s) = expand(&cmd, params, &mut Variables::new()) {
                                //try!(self.out.write_all(&s));
                                //return Ok(true)
                            //}
                        //}
                    // move cursor
                //}

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
    struct byteVecIO {
        pub bytes: Vec<u8>
    }

    impl byteVecIO {
        fn new() -> byteVecIO {
            byteVecIO { bytes: vec!() }
        }

        fn bytes(&self) -> &Vec<u8> {
            &self.bytes
        }
    }

    impl Write for byteVecIO {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            for byte in buf {
                self.bytes.push(*byte);
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn build_screen_with_vte(bytes: &byteVecIO, rows_count: usize, cols_count: usize) -> Screen {
        let mut vte = tsm_sys::Vte::new(rows_count, cols_count).unwrap();
        vte.input(&bytes.bytes());

        let mut screen = Screen::new(rows_count, cols_count);
        for cell in vte.screen.borrow_mut().cells() {
            screen.cells[cell.posy][cell.posx].ch = cell.ch
        };

        screen
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
        let mut io = byteVecIO::new();
        let mut screen = Screen::new(3, 3);
        draw_screen(&screen, &mut io);
        assert_eq!(io.bytes().len(), 0);
    }

    #[test]
    fn it_correctly_draws_empty_screen() {
        let mut screen = Screen::new(2, 2);

        let mut io = byteVecIO::new();
        draw_screen(&screen, &mut io);

        let actual = build_screen_with_vte(&io, 2, 2);
        assert_eq!(screen, actual);
    }

    #[test]
    fn it_correctly_draws_position_of_chars() {
        let mut screen = Screen::new(3, 3);
        screen.cells[0][0].ch = 'y' as char;
        screen.cells[1][1].ch = 'o' as char;
        screen.cells[2][2].ch = '!' as char;

        let mut io = byteVecIO::new();
        draw_screen(&screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }
}
