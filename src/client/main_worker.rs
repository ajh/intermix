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
    pub windows: Windows,
    pub servers: Servers,
    pub mode: Box<Mode>,
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
            mode: Box::new(CommandMode { accumulator: vec![] }),
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

        self.add_pane("win_0".to_string(), Pane {
            id: "status_line".to_string(),
            size: vterm_sys::ScreenSize { cols: 80, rows: 1 },
            offset: vterm_sys::Pos { col: 0, row: 25 },
            program_id: "status_line".to_string(),
        });

        self.damage_status_line();
    }

    /// Start receiving messages from Receiver. Exits on a Quit message.
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
                    if let Some(cmd) = self.mode.input(self, bytes) {
                        match cmd {
                            UserCmd::ProgramInput { program_id, bytes: fites } => self.program_input_cmd(program_id, fites),
                            UserCmd::ProgramStart => self.program_start_cmd(),
                        }
                    }
                },
                _ => warn!("unhandled msg {:?}", msg),
            }
        }
    }

    fn program_input_cmd(&self, program_id: String, yikes: Vec<u8>) {
        if let Some(server) = self.servers.iter().find(|s| s.programs.iter().any(|p| p.id == program_id)) {
            trace!("sending input to program {}", &program_id);
            server.tx.send(::server::ServerMsg::ProgramInput {
                program_id: program_id,
                bytes: yikes,
            });
        }
    }

    fn program_start_cmd(&self) {
        if let Some(server) = self.servers.first() {
            trace!("starting program");
            let command_and_args: Vec<String> = vec!["bash".to_string()];
            server.tx.send(::server::ServerMsg::ProgramStart {
                command_and_args: command_and_args,
                program_id: "bash-123".to_string(),
            }).unwrap();
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
        self.damage_status_line();
        self.add_pane("win_0".to_string(), Pane {
            id: "pane_0".to_string(),
            size: vterm_sys::ScreenSize { rows: 24, cols: 80 },
            offset: vterm_sys::Pos { row: 0, col: 10 },
            program_id: program.id.clone(),
        });
        self.servers.add_program(&server_id, program);
    }

    fn damage_status_line(&self) {
        trace!("damage_status_line for mode {:?}", self.mode);

        // Draw it
        let mut panes = self.windows.iter().flat_map(|w| w.panes.iter());
        if let Some(pane) = panes.find(|p| p.id == "status_line" ) {
            let mut cells = vec![];
            for (i, char) in self.mode.display().chars().enumerate() {
                cells.push(vterm_sys::ScreenCell {
                    pos: vterm_sys::Pos { row: 0, col: i as i16 },
                    chars: vec!(char),
                    width: 1,
                    attrs: Default::default(),
                    fg: vterm_sys::Color { red: 240, green: 240, blue: 240 },
                    bg: Default::default(),
                });
            }

            self.draw_worker_tx.send(ClientMsg::ProgramDamage {
                program_id: "status_line".to_string(),
                cells: cells,
            });
        } else {
            trace!("no status line pane");
        }
    }
}
