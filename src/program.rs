extern crate log;
extern crate pty;
extern crate termios;
extern crate libvterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;

use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::ptr;
use std::sync::mpsc::{Receiver};
use std::thread;
use libvterm_sys::*;
use std::iter;

pub fn fork(command_and_args: &Vec<String>) -> pty::Child {
    let mut command_and_args = command_and_args.clone();

    if command_and_args.len() == 0 {
        // TODO: use env to get SHELL variable here
        command_and_args.push("bash".to_string());
    }

    match pty::fork() {
        Ok(child) => {
            if child.pid() == 0 {
                let cstrings: Vec<CString> = command_and_args.iter().map(|s| {
                    let bytes = s.clone().into_bytes();
                    CString::new(bytes).unwrap()
                }).collect();

                let mut ptrs: Vec<*const libc::c_char> = (&cstrings).iter()
                    .map(|s| s.as_ptr())
                    .collect();

                ptrs.push(ptr::null());

                let ret = unsafe { libc::execvp(*ptrs.as_ptr(), ptrs.as_mut_ptr()) };
                panic!("error {} in execvp {}", ret, io::Error::last_os_error());
            }
            else {
                info!("got vim child process");
                child
            }
        },
        Err(e) => {
            panic!("pty::fork error: {}", e);
        }
    }
}

pub fn spawn_stdin_to_pty_thr(pty: &pty::Child) -> thread::JoinHandle<()> {
    // thread for sending stdin to pty
    let mut pty = pty.pty().unwrap().clone();
    thread::spawn(move || {
        let mut buf = [0 as u8; 4096];
        info!("starting stdin -> pty thread");
        loop {
            match io::stdin().read(&mut buf) {
                Ok(num_bytes) => {
                    if num_bytes == 0 { break };

                    //if buf.iter().find(|&x| *x == terminfo::CTRL_C).is_some() {
                        //info!("CTRL_C detected");
                        //exit();
                    //}

                    match pty.write_all(&buf[0..num_bytes]) {
                        Ok(_) => {},
                        Err(msg) => {
                            error!("{}", msg);
                            break;
                        },
                    }
                },
                Err(msg) => {
                    error!("{}", msg);
                    break;
                },
            }
        }
        info!("ending stdin -> pty thread");
    })
}

fn read_bytes_from_pty<'a, F: Read>(io: &mut F, buf: &'a mut [u8]) -> Result<&'a [u8], String> {
    // block waiting to read
    match io.read(buf) {
        Ok(num_bytes) => {
            if num_bytes == 0 {
                return Err("zero bytes reading from pty".to_string());
            }
            Ok(&buf[0..num_bytes])
        },
        Err(_) => Err("error reading from pty".to_string())
    }
}

fn color_to_index(state: &State, target: &Color) -> isize {
    for i in 0..256 {
        let color = state.get_palette_color(i);
        if color.red == target.red && color.green == target.green && color.blue == target.blue {
            return i as isize
        }
    }
    -1
}

fn draw_cell<F: Write>(state: &State, cell: &ScreenCell, prev_cell: &ScreenCell, io: &mut F, pos: &Pos, cursor_pos: &mut Pos) {
    let mut sgrs: Vec<isize> = vec!();

    if !prev_cell.attrs.bold && cell.attrs.bold                    { sgrs.push(1); }
    if prev_cell.attrs.bold && !cell.attrs.bold                    { sgrs.push(22); }
    if prev_cell.attrs.underline == 0 && cell.attrs.underline != 0 { sgrs.push(4); }
    if prev_cell.attrs.underline != 0 && cell.attrs.underline == 0 { sgrs.push(24); }
    if !prev_cell.attrs.italic && cell.attrs.italic                { sgrs.push(3); }
    if prev_cell.attrs.italic && !cell.attrs.italic                { sgrs.push(23); }
    if !prev_cell.attrs.blink && cell.attrs.blink                  { sgrs.push(5); }
    if prev_cell.attrs.blink && !cell.attrs.blink                  { sgrs.push(25); }
    if !prev_cell.attrs.reverse && cell.attrs.reverse              { sgrs.push(7); }
    if prev_cell.attrs.reverse && !cell.attrs.reverse              { sgrs.push(27); }
    if !prev_cell.attrs.strike && cell.attrs.strike                { sgrs.push(9); }
    if prev_cell.attrs.strike && !cell.attrs.strike                { sgrs.push(29); }
    if prev_cell.attrs.font == 0 && cell.attrs.font != 0           { sgrs.push(10 + cell.attrs.font as isize); }
    if prev_cell.attrs.font != 0 && cell.attrs.font == 0           { sgrs.push(10); }

    if prev_cell.fg.red   != cell.fg.red   ||
       prev_cell.fg.green != cell.fg.green ||
       prev_cell.fg.blue  != cell.fg.blue {
        trace!("changing fg color: prev {} {} {} cell {} {} {}",
               prev_cell.fg.red,
               prev_cell.fg.green,
               prev_cell.fg.blue,
               prev_cell.bg.red,
               prev_cell.bg.green,
               prev_cell.bg.blue);
        let index = color_to_index(state, &cell.fg);
        if index == -1 { sgrs.push(39); }
        else if index < 8 { sgrs.push(30 + index); }
        else if index < 16 { sgrs.push(90 + (index - 8)); }
        else {
            sgrs.push(38);
            sgrs.push(5 | (1<<31));
            sgrs.push(index | (1<<31));
        }
    }

    if prev_cell.bg.red   != cell.bg.red   ||
       prev_cell.bg.green != cell.bg.green ||
       prev_cell.bg.blue  != cell.bg.blue {
        let index = color_to_index(state, &cell.bg);
        if index == -1 { sgrs.push(49); }
        else if index < 8 { sgrs.push(40 + index); }
        else if index < 16 { sgrs.push(100 + (index - 8)); }
        else {
            sgrs.push(48);
            sgrs.push(5 | (1<<31));
            sgrs.push(index | (1<<31));
        }
    }

    if sgrs.len() != 0 {
        let mut sgr = "\x1b[".to_string();
        for (i, val) in sgrs.iter().enumerate() {
            let bare_val = val & !(1<<31);
            if i == 0 {
                sgr.push_str(&format!("{}", bare_val));
            }
            else if val & (1<<31) != 0 {
                sgr.push_str(&format!(":{}", bare_val));
            }
            else {
                sgr.push_str(&format!(";{}", bare_val));
            }
        }
        sgr.push_str("m");
        io.write_all(sgr.as_bytes()).unwrap();
    }

    if pos.row != cursor_pos.row || pos.col != cursor_pos.col {
        trace!("moving cursor to row {:?} col {:?}", pos.row, pos.col);
        let ti = term::terminfo::TermInfo::from_env().unwrap();
        let cmd = ti.strings.get("cup").unwrap();
        let params = [ term::terminfo::parm::Param::Number(pos.row as i16),
                       term::terminfo::parm::Param::Number(pos.col as i16) ];
        let s = term::terminfo::parm::expand(&cmd, &params, &mut term::terminfo::parm::Variables::new()).unwrap();
        io.write_all(&s).unwrap();
    }

    io.write_all(&cell.chars_as_utf8_bytes()).ok().expect("failed to write");

    cursor_pos.row = pos.row;
    cursor_pos.col = pos.col + 1;
}

fn draw_rect<F: Write>(vterm: &mut VTerm, rect: &Rect, io: &mut F) {
    trace!("damage {:?}", rect);
    let (fg, bg) = vterm.get_state().get_default_colors();
    let mut prev_cell: ScreenCell = Default::default();
    prev_cell.fg = fg;
    prev_cell.bg = bg;
    let mut pos: Pos = Default::default();

    // turn off cursor
    let ti = term::terminfo::TermInfo::from_env().unwrap();
    let cmd = ti.strings.get("civis").unwrap();
    let s = term::terminfo::parm::expand(&cmd, &[], &mut term::terminfo::parm::Variables::new()).unwrap();
    io.write_all(&s).unwrap();

    // move cursor to first position
    let ti = term::terminfo::TermInfo::from_env().unwrap();
    let cmd = ti.strings.get("cup").unwrap();
    let params = [ term::terminfo::parm::Param::Number(rect.start_row as i16),
                   term::terminfo::parm::Param::Number(rect.start_col as i16) ];
    let s = term::terminfo::parm::expand(&cmd, &params, &mut term::terminfo::parm::Variables::new()).unwrap();
    io.write_all(&s).unwrap();
    let mut cursor_pos = Pos { row: rect.start_row, col: rect.start_col };

    for row in rect.start_row..rect.end_row {
        pos.row = row;
        for col in rect.start_col..rect.end_col {
            pos.col = col;
            let cell = vterm.get_screen().get_cell(&pos);
            draw_cell(&vterm.get_state(), &cell, &prev_cell, io, &pos, &mut cursor_pos);
            prev_cell = cell;
        }
    }

    io.flush().unwrap();

    let ti = term::terminfo::TermInfo::from_env().unwrap();
    let cmd = ti.strings.get("cvvis").unwrap();
    let s = term::terminfo::parm::expand(&cmd, &[], &mut term::terminfo::parm::Variables::new()).unwrap();
    io.write_all(&s).unwrap();
}

fn draw_with_vterm<F: Write>(bytes: &[u8], vterm: &mut VTerm, io: &mut F, rx: &Receiver<ScreenEvent>) {
    vterm.write(bytes);
    vterm.get_screen().flush_damage();

    // Handle screen events
    while let Ok(event) = rx.try_recv() {
        match event {
            ScreenEvent::Damage{rect} => draw_rect(vterm, &rect, io),
            ScreenEvent::SbPushLine{cells: _} => info!("sb push line"),
            ScreenEvent::SbPopLine{cells: _} => info!("sb push line"),
            ScreenEvent::MoveRect{dest, src} => info!("move rect dest {:?} src {:?}", dest, src),
            ScreenEvent::MoveCursor{new, old, is_visible} => info!("move cursor new {:?} old {:?} is_visible {:?}", new, old, is_visible),
            ScreenEvent::Bell => info!("bell"),
            ScreenEvent::Resize{rows, cols} => info!("resize rows {:?} cols {:?}", rows, cols),
        }
    }
}

fn draw_direct<F: Write>(bytes: &[u8], io: &mut F) {
    io.write_all(bytes).unwrap();
    io.flush().unwrap();
}

pub fn spawn_pty_to_stdout_thr(pty: &pty::Child) -> thread::JoinHandle<()> {
    // thread for sending stdin to pty
    let pty = pty.pty().unwrap().clone();

    thread::spawn(move || {
        let mut buf = [0 as u8; 4096];
        let reader = unsafe { File::from_raw_fd(pty.as_raw_fd()) };
        let mut reader = BufReader::new(reader);

        let mut vterm = VTerm::new(24, 80);
        vterm.set_utf8(true);
        let rx = vterm.receive_screen_events();
        vterm.get_screen().reset(true);

        let writer = io::stdout();
        let mut writer = BufWriter::new(writer);

        info!("starting pty -> stdout thread");
        loop {
            let result = read_bytes_from_pty(&mut reader, &mut buf);
            if result.is_err() {
                error!("{}", result.err().unwrap());
                break;
            }
            let bytes = result.unwrap();

            if true {
                draw_with_vterm(bytes, &mut vterm, &mut writer, &rx);
            }
            else {
                draw_direct(bytes, &mut writer);
            }

            thread::sleep_ms(10);
        }
        info!("ending pty -> stdout thr");
    })
}
