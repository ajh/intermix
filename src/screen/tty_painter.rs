extern crate term;

use std::io::Write;

use super::*;
use term::terminfo::*;

pub fn draw_screen<T: Write+Send>(screen: &Screen, writer: &mut T) {
    let mut tty = TerminfoTerminal::new(writer).unwrap();

    let (mut last_x, mut last_y) = (0, 0);
    let params = [ parm::Param::Number(0 as i16),
                   parm::Param::Number(0 as i16) ];
    tty.apply_cap("cup", &params);
    drop(params);

    for row in &screen.cells {
        for cell in row {
            // TODO: check age and maybe don't draw this cell

            // move cursor maybe
            if (last_x + 1 != cell.x) || (last_y != cell.y) {
                let params = [ parm::Param::Number(cell.x as i16),
                               parm::Param::Number(cell.y as i16) ];
                tty.apply_cap("cup", &params);
            }

            // write character
            let mut buf = [0 as u8; 4];
            match cell.ch.encode_utf8(&mut buf) {
                Some(num_bytes) => {
                    tty.write(&buf[0..num_bytes]);
                },
                None => {}
            }

            last_x = cell.x;
            last_y = cell.y;
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
    struct CaptureIO {
        pub cursor: usize,
        pub bytes:  Vec<u8>
    }

    impl CaptureIO {
        fn new() -> CaptureIO {
            CaptureIO { cursor: 0, bytes: vec!() }
        }

        fn slice(&self) -> &[u8] {
            &self.bytes
        }
    }

    impl Write for CaptureIO {
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

    fn build_screen_with_vte(bytes: &CaptureIO, rows_count: usize, cols_count: usize) -> Screen {
        let mut vte = tsm_sys::Vte::new(rows_count, cols_count).unwrap();
        vte.input(&bytes.slice());

        let mut screen = Screen::new(rows_count, cols_count);
        for cell in vte.screen.borrow_mut().cells() {
            screen.cells[cell.posy][cell.posx].ch = cell.ch
        };

        screen
    }

    #[test]
    fn it_correctly_draws_empty_screen() {
        let mut screen = Screen::new(2, 2);

        let mut io = CaptureIO::new();
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

        let mut io = CaptureIO::new();
        draw_screen(&screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }

    #[test]
    fn it_draws_consecutive_chars() {
        let mut screen = Screen::new(3, 3);
        screen.cells[1][0].ch = 'y' as char;
        screen.cells[1][1].ch = 'o' as char;
        screen.cells[1][2].ch = '!' as char;

        let mut io = CaptureIO::new();
        draw_screen(&screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }

    #[test]
    fn it_draws_chars_with_gaps() {
        let mut screen = Screen::new(3, 3);
        screen.cells[0][0].ch = 'a' as char;
        screen.cells[0][2].ch = 'b' as char;
        screen.cells[1][0].ch = 'c' as char;
        screen.cells[1][2].ch = 'd' as char;

        let mut io = CaptureIO::new();
        draw_screen(&screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }

    #[test]
    fn it_draws_vertical_chars() {
        let mut screen = Screen::new(3, 3);
        screen.cells[0][1].ch = 'a' as char;
        screen.cells[1][1].ch = 'b' as char;
        screen.cells[2][1].ch = 'c' as char;

        let mut io = CaptureIO::new();
        draw_screen(&screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }
}
