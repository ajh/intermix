extern crate log;
extern crate pty;
extern crate termios;
extern crate libvterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;
extern crate uuid;

use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::ptr;
use std::sync::mpsc;
use std::thread;
use libvterm_sys::*;

/// Handles reads on the pty, passes the input to libvter, and generates ProgramEvents
pub struct EventHandler {
    pty: File,
    tx: mpsc::Sender<super::ProgramEvent>,
}

impl EventHandler {
    pub fn new(io: File, tx: mpsc::Sender<super::ProgramEvent>) -> EventHandler {
        EventHandler {
            pty: io,
            tx: tx,
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut buf = [0 as u8; 4096];
            //let reader = unsafe { File::from_raw_fd(self.pty.as_raw_fd()) };
            let mut reader = BufReader::new(&self.pty);

            // create vterm instance.
            let mut vterm = VTerm::new(24, 80);
            vterm.set_utf8(true);
            let vterm_event_rx = vterm.receive_screen_events();
            vterm.get_screen().reset(true); // boilerplate to avoid segfault

            info!("starting pty -> stdout thread");
            loop {
                // block until read
                let bytes = match reader.read(&mut buf) {
                    Ok(num_bytes) => {
                        if num_bytes == 0 {
                            error!("zero bytes reading from pty");
                            break;
                        }
                        &buf[0..num_bytes]
                    },
                    Err(_) => {
                        error!("error reading from pty");
                        break;
                    }
                };

                vterm.write(bytes);
                vterm.get_screen().flush_damage();

                self.handle_screen_events(&mut vterm, &vterm_event_rx);

                // Not sure I need this since the read is blocking anyway
                thread::sleep_ms(10);
            }
            info!("ending pty -> stdout thr");
        })
    }

    fn handle_screen_events(&self, vterm: &mut VTerm, rx: &mpsc::Receiver<ScreenEvent>) {
        while let Ok(event) = rx.try_recv() {
            match event {
                ScreenEvent::Bell => info!("bell"),
                ScreenEvent::Damage{rect} => self.send_program_damage_event(vterm, &rect),
                ScreenEvent::MoveCursor{new, old, is_visible} => info!("move cursor new {:?} old {:?} is_visible {:?}", new, old, is_visible),
                ScreenEvent::MoveRect{dest, src} => info!("move rect dest {:?} src {:?}", dest, src),
                ScreenEvent::Resize{rows, cols} => info!("resize rows {:?} cols {:?}", rows, cols),
                ScreenEvent::SbPopLine{cells: _} => info!("sb push line"),
                ScreenEvent::SbPushLine{cells: _} => info!("sb push line"),
                ScreenEvent::AltScreen{ is_true: _ } => info!("AltScreen"),
                ScreenEvent::CursorBlink{ is_true: _ } => info!("CursorBlink"),
                ScreenEvent::CursorShape{ value: _ } => info!("CursorShape"),
                ScreenEvent::CursorVisible{ is_true: _ } => info!("CursorVisible"),
                ScreenEvent::IconName{ text: _} => info!("IconName"),
                ScreenEvent::Mouse{ value: _ } => info!("Mouse"),
                ScreenEvent::Reverse{ is_true: _ } => info!("Reverse"),
                ScreenEvent::Title{ text: _} => info!("Title"),
            }
        }
    }

    fn send_program_damage_event(&self, vterm: &mut VTerm, rect: &Rect) {
        //trace!("damage {:?}", rect);
        let mut pos: Pos = Default::default();

        let mut cells: Vec<ScreenCell> = Vec::new(); // capacity is known here FYI

        for row in rect.start_row..rect.end_row {
            pos.row = row as i16;
            for col in rect.start_col..rect.end_col {
                pos.col = col as i16;
                cells.push(vterm.get_screen().get_cell(&pos));
            }
        }

        let event = super::ProgramEvent::Damage { program_id: "not implemented".to_string(), cells: cells };
        self.tx.send(event).unwrap();
    }
}
