use std::io::prelude::*;
use std::sync::mpsc::*;
use std::thread;
use vterm_sys::*;
use super::*;

/// Runs bytes from the pty through VTerm, and sends ServerMsgs.
///
/// This has to handle two kinds of Receivers:
/// * VteWorkerMsg
/// * and vterm_sys::ScreenEvent.
pub struct VteWorker {
    tx: Sender<VteWorkerMsg>,
    rx: Option<Receiver<VteWorkerMsg>>,
    server_tx: Sender<::server::ServerMsg>,
    program_id: String,
    vterm: VTerm,
}

impl VteWorker {
    pub fn spawn(server_tx: Sender<::server::ServerMsg>,
                 program_id: &str)
                 -> (Sender<VteWorkerMsg>, thread::JoinHandle<()>) {
        let (tx, rx) = channel::<VteWorkerMsg>();
        let tx_clone = tx.clone();
        let program_id = program_id.to_string();

        info!("spawning vte worker for program {}", program_id);
        let handle = thread::spawn(move || {
            let mut worker = VteWorker::new(server_tx, tx, rx, &program_id);
            worker.enter_listen_loop();
            info!("exiting vte worker for program {}", program_id);
        });

        (tx_clone, handle)
    }

    pub fn new(server_tx: Sender<::server::ServerMsg>,
               tx: Sender<VteWorkerMsg>,
               rx: Receiver<VteWorkerMsg>,
               program_id: &str)
               -> VteWorker {
        VteWorker {
            program_id: program_id.to_string(),
            rx: Some(rx),
            tx: tx,
            server_tx: server_tx,
            // FIXME: get size from self
            vterm: VTerm::new(ScreenSize {
                rows: 24,
                cols: 80,
            }),
        }
    }

    pub fn enter_listen_loop(&mut self) {
        self.vterm.set_utf8(true);
        self.vterm.screen_set_damage_merge(ffi::VTermDamageSize::VTermDamageRow);

        self.vterm.generate_screen_events().unwrap();
        let vterm_event_rx = self.vterm.screen_event_rx.take().unwrap();

        // work around lifetime issue
        let program_event_rx = self.rx.take().unwrap();

        loop {
            select! {
                program_event = program_event_rx.recv() => self.handle_program_event(program_event.unwrap()),
                screen_event = vterm_event_rx.recv() => self.handle_screen_event(screen_event.unwrap())
            }
        }

        // work around lifetime issue
        self.rx = Some(program_event_rx);
    }

    fn handle_screen_event(&mut self, event: ScreenEvent) {
        match event {
            ScreenEvent::Bell => info!("Bell"),
            ScreenEvent::Damage{rect} => {
                info!("Damage: rect={:?}", rect);
                let event = ::server::ServerMsg::ProgramDamage {
                    program_id: self.program_id.clone(),
                    cells: self.vterm.screen_get_cells_in_rect(&rect),
                };
                self.server_tx.send(event).unwrap();
            }
            ScreenEvent::MoveCursor{new, old, is_visible} => {
                info!("MoveCursor: new={:?} old={:?} is_visible={:?}",
                      new,
                      old,
                      is_visible);
                let event = ::server::ServerMsg::ProgramMoveCursor {
                    program_id: self.program_id.clone(),
                    new: new,
                    old: old,
                    is_visible: is_visible,
                };
                self.server_tx.send(event).unwrap();
            }
            ScreenEvent::MoveRect{dest, src} => {
                info!("MoveRect: dest={:?} src={:?}", dest, src);
                let event = ::server::ServerMsg::ProgramDamage {
                    program_id: self.program_id.clone(),
                    cells: self.vterm.screen_get_cells_in_rect(&dest),
                };
                self.server_tx.send(event).unwrap();
            }
            ScreenEvent::Resize{rows, cols} => info!("Resize: rows={:?} cols={:?}", rows, cols),
            ScreenEvent::SbPopLine{cells: _} => info!("SbPopLine"),
            ScreenEvent::SbPushLine{cells} => info!("SbPushLine"),
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

    fn handle_program_event(&mut self, event: VteWorkerMsg) {
        match event {
            VteWorkerMsg::PtyRead{bytes} => {
                self.vterm.write(bytes.as_slice());
                self.vterm.screen_flush_damage();
            }
            VteWorkerMsg::PtyReadZero => error!("got PtyReadZero"),
            VteWorkerMsg::PtyReadError => error!("got PtyReadError"),
            VteWorkerMsg::RequestRedrawRect{rect: _} => info!("got RequestRedrawRect msg"),
        }
    }
}
