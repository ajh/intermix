extern crate log;
extern crate pty;
extern crate termios;
extern crate libvterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;
extern crate uuid;

use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use libvterm_sys::*;
use super::*;
use ::window::WindowMsg;

/// Handles reads on the pty, passes the input to libvter, and generates ProgramEvents
pub struct MsgListener {
    /// Sends events to this handler
    pub tx: mpsc::Sender<ProgramMsg>,

    window_tx: mpsc::Sender<WindowMsg>,
    rx: mpsc::Receiver<ProgramMsg>,
    program_id: String,
}

impl MsgListener {
    pub fn new(id: &str, window_tx: mpsc::Sender<WindowMsg>) -> MsgListener {
        let (tx, rx) = mpsc::channel::<ProgramMsg>();

        MsgListener {
            program_id: id.to_string(),
            rx: rx,
            tx: tx,
            window_tx: window_tx,
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        info!("spawning event handler");
        thread::spawn(move || {
            // create vterm instance.
            let mut vterm = VTerm::new(ScreenSize {
                rows: 24,
                cols: 80,
            });
            vterm.set_utf8(true);
            let vterm_event_rx = vterm.receive_screen_events();

            info!("starting event handler");
            loop {
                let program_event_rx = &self.rx;

                select! {
                    program_event = program_event_rx.recv() => self.handle_program_event(program_event.unwrap(), &mut vterm),
                    screen_event = vterm_event_rx.recv() => self.handle_screen_event(screen_event.unwrap(), &mut vterm)
                }
            }
            //info!("ending event handler");
        })
    }

    fn handle_screen_event(&self, event: ScreenEvent, vterm: &mut VTerm) {
        match event {
            ScreenEvent::Bell => info!("bell"),
            ScreenEvent::Damage{rect} => {
                // TODO: change this to build_damage_event then explicitly sent it
                self.send_program_damage_event(vterm, &rect);
            }
            ScreenEvent::MoveCursor{new, old, is_visible} => {
                let event = WindowMsg::MoveCursor {
                    program_id: self.program_id.clone(),
                    new: new,
                    old: old,
                    is_visible: is_visible,
                };
                self.window_tx.send(event).unwrap();
            },
            ScreenEvent::MoveRect{dest, src} =>
                info!("MoveRect: dest={:?} src={:?}", dest, src),
            ScreenEvent::Resize{rows, cols} => info!("Resize: rows={:?} cols={:?}", rows, cols),
            ScreenEvent::SbPopLine{cells: _} => info!("SbPopLine"),
            ScreenEvent::SbPushLine{cells} => {
                info!("SbPushLine");
                let event = WindowMsg::SbPushLine {
                    program_id: self.program_id.clone(),
                    cells: cells,
                };
                self.window_tx.send(event).unwrap();
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

    fn handle_program_event(&self, event: ProgramMsg, vterm: &mut VTerm) {
        match event {
            ProgramMsg::PtyRead{bytes} => {
                vterm.write(bytes.as_slice());
                vterm.screen.flush_damage();
            },
            ProgramMsg::PtyReadZero => error!("got read zero bytes event"),
            ProgramMsg::PtyReadError => error!("got read error event"),
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

        let event = WindowMsg::Damage {
            program_id: self.program_id.clone(),
            cells: cells,
        };
        self.window_tx.send(event).unwrap();
    }
}
