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
use ::client::window::WindowMsg;

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
            vterm.screen.set_damage_merge(ffi::VTermDamageSize::VTermDamageRow);

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
            ScreenEvent::Bell => info!("Bell"),
            ScreenEvent::Damage{rect} => {
                info!("Damage: rect={:?}", rect);
                let event = WindowMsg::Damage {
                    program_id: self.program_id.clone(),
                    cells: self.get_cells_in_rect(vterm, &rect),
                };
                self.window_tx.send(event).unwrap();
            }
            ScreenEvent::MoveCursor{new, old, is_visible} => {
                info!("MoveCursor: new={:?} old={:?} is_visible={:?}", new, old, is_visible);
                let event = WindowMsg::MoveCursor {
                    program_id: self.program_id.clone(),
                    new: new,
                    old: old,
                    is_visible: is_visible,
                };
                self.window_tx.send(event).unwrap();
            },
            ScreenEvent::MoveRect{dest, src} => {
                info!("MoveRect: dest={:?} src={:?}", dest, src);
                let event = WindowMsg::Damage {
                    program_id: self.program_id.clone(),
                    cells: self.get_cells_in_rect(vterm, &dest),
                };
                self.window_tx.send(event).unwrap();
            }
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
            ScreenEvent::AltScreen{ is_true } => info!("AltScreen: is_true={:?}", is_true),
            ScreenEvent::CursorBlink{ is_true } => info!("CursorBlink: is_true={:?}", is_true),
            ScreenEvent::CursorShape{ value } => info!("CursorShape: value={:?}", value),
            ScreenEvent::CursorVisible{ is_true } => info!("CursorVisible: is_true={:?}", is_true),
            ScreenEvent::IconName{ text } => info!("IconName: text={:?}", text),
            ScreenEvent::Mouse{ value } => info!("Mouse: value={:?}", value),
            ScreenEvent::Reverse{ is_true } => info!("Reverse: is_true={:?}", is_true),
            ScreenEvent::Title{ text } => info!("Title: text={:?}", text),
        }
    }

    fn handle_program_event(&self, event: ProgramMsg, vterm: &mut VTerm) {
        match event {
            ProgramMsg::PtyRead{bytes} => {
                vterm.write(bytes.as_slice());
                vterm.screen.flush_damage();
            },
            ProgramMsg::PtyReadZero => error!("got PtyReadZero"),
            ProgramMsg::PtyReadError => error!("got PtyReadError"),
            ProgramMsg::RequestRedrawRect{rect: _} => info!("got RequestRedrawRect msg"),
        }
    }

    /// TODO: move this to libvterm, since it seems useful
    fn get_cells_in_rect(&self, vterm: &VTerm, rect: &Rect) -> Vec<ScreenCell> {
        let mut pos: Pos = Default::default();
        let mut cells: Vec<ScreenCell> = Vec::new(); // capacity is known here FYI

        // could fancy functional iterator stuff be used here?
        for row in rect.start_row..rect.end_row {
            pos.row = row as i16;
            for col in rect.start_col..rect.end_col {
                pos.col = col as i16;
                cells.push(vterm.screen.get_cell(&pos));
            }
        }

        cells
    }
}
