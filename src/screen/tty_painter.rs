extern crate term;

use std::io::Write;

use super::*;
use term::terminfo::*;

pub fn draw_screen<T: Write+Send>(screen: &mut Screen, writer: &mut T) {
    let mut tty = TerminfoTerminal::new(writer).unwrap();

    let (mut last_x, mut last_y) = (0, 0);
    let params = [ parm::Param::Number(0 as i16),
                   parm::Param::Number(0 as i16) ];
    tty.apply_cap("cup", &params);
    drop(params);

    for row in &mut screen.cells {
        for cell in row {
            //trace!("{:?}", cell);

            // check age and maybe don't draw this cell
            if !cell.dirty {
                //trace!("not dirty");
                continue;
            }
            let is_unprintable = (cell.ch as u32) < 32;
            if is_unprintable {
                //trace!("unprintable");
                continue;
            }

            let already_in_position = (last_x + 1 == cell.x) && (last_y == cell.y);
            if !already_in_position {
                //trace!("out of position");
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

            cell.dirty = false;
            last_x = cell.x;
            last_y = cell.y;
        }
    }

    //panic!("blah");
}

#[cfg(test)]
mod tests {
    extern crate tsm_sys;
    extern crate num;

    use std::io::{self, Error, ErrorKind, Write};
    use std::str::*;
    use std::char::{self, from_u32};

    use super::*;
    use super::super::*;

    fn new_screen(rows_count: usize, contents: &str) -> Screen {
        if contents.len() % rows_count != 0 {
            panic!("contents don't match given rows_count");
        }
        let cols_count = num::integer::div_floor(contents.len(), rows_count);
        let mut screen = Screen::new(rows_count, cols_count);

        for (i, ch) in contents.chars().enumerate() {
            let x = i % cols_count;
            let y = i / cols_count;
            screen.cells[y][x].ch = ch;
            screen.cells[y][x].dirty = true;
        }

        screen
    }

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
        draw_screen(&mut screen, &mut io);

        let actual = build_screen_with_vte(&io, 2, 2);
        assert_eq!(screen, actual);
    }

    #[test]
    fn it_draws_diagonal_chars() {
        let mut screen = new_screen(3, &format!("{}{}{}",
            "y  ",
            " o ",
            "  !"));

        let mut io = CaptureIO::new();
        draw_screen(&mut screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }

    #[test]
    fn it_draws_other_diagonal_chars() {
        let mut screen = new_screen(3, &format!("{}{}{}",
            "  y",
            " o ",
            "!  "));

        let mut io = CaptureIO::new();
        draw_screen(&mut screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }

    #[test]
    fn it_draws_consecutive_chars() {
        let mut screen = new_screen(3, &format!("{}{}{}",
            "   ",
            "yo!",
            "   "));

        let mut io = CaptureIO::new();
        draw_screen(&mut screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }

    #[test]
    fn it_draws_chars_with_gaps() {
        let mut screen = new_screen(3, &format!("{}{}{}",
            "a b",
            "c d",
            "e f"));

        let mut io = CaptureIO::new();
        draw_screen(&mut screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }

    #[test]
    fn it_draws_vertical_chars() {
        let mut screen = new_screen(3, &format!("{}{}{}",
            " a ",
            " b ",
            " c "));

        let mut io = CaptureIO::new();
        draw_screen(&mut screen, &mut io);

        let actual = build_screen_with_vte(&io, 3, 3);
        assert_eq!(screen, actual);
    }
}
