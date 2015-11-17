extern crate log;
extern crate pty;
extern crate termios;
extern crate libvterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;
extern crate uuid;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;
use libvterm_sys::*;
use super::*;

/// Handles reads on the pty, passes the input to libvter, and generates ProgramEvents
pub struct EventHandler {
    pty: File,
    tx: mpsc::Sender<ProgramEvent>,
    program_id: String,
}

impl EventHandler {
    pub fn new(id: &str, io: File, tx: mpsc::Sender<ProgramEvent>) -> EventHandler {
        EventHandler {
            pty: io,
            tx: tx,
            program_id: id.to_string(),
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        info!("spawning event handler");
        thread::spawn(move || {
            let mut buf = [0 as u8; 4096];
            // let reader = unsafe { File::from_raw_fd(self.pty.as_raw_fd()) };
            let mut reader = BufReader::new(&self.pty);

            // create vterm instance.
            let mut vterm = VTerm::new(ScreenSize {
                rows: 24,
                cols: 80,
            });
            vterm.set_utf8(true);
            let vterm_event_rx = vterm.receive_screen_events();

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
                    }
                    Err(_) => {
                        error!("error reading from pty");
                        break;
                    }
                };

                vterm.write(bytes);
                vterm.screen.flush_damage();

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
                ScreenEvent::Damage{rect} => {
                    // TODO: change this to build_damage_event then explicitly sent it
                    self.send_program_damage_event(vterm, &rect);
                }
                ScreenEvent::MoveCursor{new, old, is_visible} => {
                    let event = ProgramEvent::MoveCursor {
                        program_id: self.program_id.clone(),
                        new: new,
                        old: old,
                        is_visible: is_visible,
                    };
                    self.tx.send(event).unwrap();
                },
                ScreenEvent::MoveRect{dest, src} =>
                    info!("MoveRect: dest={:?} src={:?}", dest, src),
                ScreenEvent::Resize{rows, cols} => info!("Resize: rows={:?} cols={:?}", rows, cols),
                ScreenEvent::SbPopLine{cells: _} => info!("SbPopLine"),
                ScreenEvent::SbPushLine{cells: cells} => {
                    info!("SbPushLine");
                    let event = ProgramEvent::SbPushLine {
                        program_id: self.program_id.clone(),
                        cells: cells,
                    };
                    self.tx.send(event).unwrap();
                },
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

    /// find cells that are damanged and send them as part of the program event
    fn send_program_damage_event(&self, vterm: &mut VTerm, rect: &Rect) {
        // trace!("damage {:?}", rect);
        let mut pos: Pos = Default::default();

        let mut cells: Vec<ScreenCell> = Vec::new(); // capacity is known here FYI

        for row in rect.start_row..rect.end_row {
            pos.row = row as i16;
            for col in rect.start_col..rect.end_col {
                pos.col = col as i16;
                cells.push(vterm.screen.get_cell(&pos));
            }
        }

        let event = ProgramEvent::Damage {
            program_id: self.program_id.clone(),
            cells: cells,
        };
        self.tx.send(event).unwrap();
    }
}
