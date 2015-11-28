use vterm_sys;
use super::*;
use super::state::*;
use super::modal::*;
use std::sync::mpsc::*;
use std::thread::{self, JoinHandle};

/// This worker handles:
/// * user input
/// * server state change messages
///
/// It also owns the client state. When client state is changed it sends msgs to sync the draw
/// worker's internal representation.
///
/// It doesn't receive any server damage messages.
pub struct MainWorker {
    rx: Receiver<ClientMsg>,
    draw_worker_tx: Sender<ClientMsg>,
    windows: Windows,
    servers: Servers,
    mode: Box<Mode>,
}

impl MainWorker {
    pub fn spawn(draw_worker_tx: Sender<ClientMsg>) -> (Sender<ClientMsg>, JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();

        info!("spawning main worker");
        let handle = thread::spawn(move || {
            let mut worker = MainWorker::new(draw_worker_tx, rx);
            worker.enter_listener_loop();
            info!("exiting main worker");
        });

        (tx, handle)
    }

    fn new(draw_worker_tx: Sender<ClientMsg>, rx: Receiver<ClientMsg>) -> MainWorker {
        let mut worker = MainWorker {
            draw_worker_tx: draw_worker_tx,
            rx: rx,
            windows: Default::default(),
            servers: Default::default(),
            mode: Box::new(ProgramMode { program_id: "fixme".to_string() }),
        };
        worker.init();
        worker
    }

    /// creates an initial window, status pane etc
    fn init(&mut self) {
        self.add_window(Window {
            id: "win_0".to_string(),
            size: vterm_sys::ScreenSize { cols: 80, rows: 25 },
            .. Default::default()
         });
    }

    fn enter_listener_loop(&mut self) {
        loop {
            let msg = match self.rx.recv() {
                Ok(msg) => msg,
                Err(_) => break,
            };

            match msg {
                ClientMsg::Quit => break,
                ClientMsg::ServerAdd { server } => self.servers.add_server(server),
                ClientMsg::ProgramAdd { server_id, program } => self.add_program(server_id, program),
                ClientMsg::UserInput { bytes } => {
                    // Would also like to have mode be able to change self.mode. Cell?
                    //
                    // Maybe not have mode make changes, but indicate which command if any to run?
                    // Then it could just accumulate bytes and match against commands. Commands
                    // could be run on self as methods.
                    self.mode.input(bytes, &mut self.windows, &mut self.servers)
                }
                _ => warn!("unhandled msg {:?}", msg),
            }
        }
    }

    fn add_window(&mut self, window: Window) {
        self.windows.add_window(window.clone());
        let msg = ClientMsg::WindowAdd { window: window };
        self.draw_worker_tx.send(msg).unwrap();
    }

    fn add_pane(&mut self, window_id: String, pane: Pane) {
        self.windows.add_pane(&window_id, pane.clone());
        let msg = ClientMsg::PaneAdd { window_id: window_id, pane: pane };
        self.draw_worker_tx.send(msg).unwrap();
    }

    /// For now we only expect this once, so create a pane and enter program mode aimed at it
    fn add_program(&mut self, server_id: String, program: Program) {
        self.mode = Box::new(ProgramMode { program_id: program.id.clone() });
        self.add_pane("win_0".to_string(), Pane {
            id: "pane_0".to_string(),
            size: vterm_sys::ScreenSize { rows: 24, cols: 80 },
            offset: vterm_sys::Pos { row: 0, col: 10 },
            program_id: program.id.clone(),
        });
        self.servers.add_program(&server_id, program);
    }
}
