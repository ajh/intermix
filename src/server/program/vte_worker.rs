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

        // FIXME: get size from self
        let mut vterm = VTerm::new(&Size::new(80, 24));
        let fg = vterm.state_get_rgb_color_from_palette(7);
        let bg = vterm.state_get_rgb_color_from_palette(0);
        vterm.state_set_default_colors(&fg, &bg);
        vterm.set_utf8(true);
        vterm.screen_reset(true);

        VteWorker {
            program_id: program_id.to_string(),
            rx: Some(rx),
            server_tx: server_tx,
            tx: tx,
            vterm: vterm,
        }
    }

    pub fn enter_listen_loop(&mut self) {
        self.vterm.set_utf8(true);
        self.vterm.screen_set_damage_merge(DamageSize::Row);

        self.vterm.screen_receive_events(&ScreenCallbacksConfig::all());
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
            ScreenEvent::Damage(e) => {
                info!("Damage: {:?}", e);
                let event = ::server::ServerMsg::ProgramDamage {
                    program_id: self.program_id.clone(),
                    cells: self.vterm.screen_get_cells_in_rect(&e.rect),
                    rect: e.rect,
                };
                self.server_tx.send(event).unwrap();
            }
            ScreenEvent::MoveCursor(e) => {
                info!("MoveCursor: {:?}", e);
                let event = ::server::ServerMsg::ProgramMoveCursor {
                    program_id: self.program_id.clone(),
                    new: e.new,
                    old: e.old,
                    is_visible: e.is_visible,
                };
                self.server_tx.send(event).unwrap();
            }
            ScreenEvent::MoveRect(e) => {
                info!("MoveRect: {:?}", e);
                let event = ::server::ServerMsg::ProgramDamage {
                    program_id: self.program_id.clone(),
                    cells: self.vterm.screen_get_cells_in_rect(&e.src),
                    rect: e.src,
                };
                self.server_tx.send(event).unwrap();

                let event = ::server::ServerMsg::ProgramDamage {
                    program_id: self.program_id.clone(),
                    cells: self.vterm.screen_get_cells_in_rect(&e.dest),
                    rect: e.dest,
                };
                self.server_tx.send(event).unwrap();
            }
            ScreenEvent::Resize(e) => info!("Resize: {:?}", e),
            ScreenEvent::SbPopLine(e) => info!("SbPopLine"),
            ScreenEvent::SbPushLine(e) => info!("SbPushLine"),
            ScreenEvent::AltScreen(e) => info!("AltScreen: {:?}", e),
            ScreenEvent::CursorBlink(e) => info!("CursorBlink: {:?}", e),
            ScreenEvent::CursorShape(e) => info!("CursorShape: {:?}", e),
            ScreenEvent::CursorVisible(e) => info!("CursorVisible: {:?}", e),
            ScreenEvent::IconName(e) => info!("IconName: {:?}", e),
            ScreenEvent::Mouse(e) => info!("Mouse: {:?}", e),
            ScreenEvent::Reverse(e) => info!("Reverse: {:?}", e),
            ScreenEvent::Title(e) => info!("Title: {:?}", e),
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
