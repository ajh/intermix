use vterm_sys;
use super::*;
use super::state::*;
use super::layout::*;
use super::modal::*;
use std::sync::mpsc::*;
use std::sync::{Arc, RwLock};
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
    pub servers: Servers,
    pub mode: Box<Mode>,
    pub tty_ioctl_config: TtyIoCtlConfig,
    pub layout: Arc<RwLock<Screen>>,
}

impl MainWorker {
    pub fn spawn(draw_worker_tx: Sender<ClientMsg>, tty_ioctl_config: TtyIoCtlConfig) -> (Sender<ClientMsg>, Arc<RwLock<Screen>>, JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();
        let layout = Arc::new(RwLock::new(Screen::new(
                    Size {
                        rows: tty_ioctl_config.rows,
                        cols: tty_ioctl_config.cols,
                    },
                    Node::row(vec![]),
                    )));
        let layout_clone = layout.clone();

        info!("spawning main worker");
        let handle = thread::spawn(move || {
            let mut worker = MainWorker::new(draw_worker_tx, rx, tty_ioctl_config, layout);
            worker.enter_listener_loop();
            info!("exiting main worker");
        });

        (tx, layout_clone, handle)
    }

    fn new(draw_worker_tx: Sender<ClientMsg>, rx: Receiver<ClientMsg>, tty_ioctl_config: TtyIoCtlConfig, layout: Arc<RwLock<Screen>>) -> MainWorker {
        let mut worker = MainWorker {
            draw_worker_tx: draw_worker_tx,
            rx: rx,
            servers: Default::default(),
            mode: Box::new(CommandMode { accumulator: vec![] }),
            tty_ioctl_config: tty_ioctl_config.clone(),
            layout: layout,
        };
        worker.init();
        worker
    }

    /// creates an initial window, status pane etc
    fn init(&mut self) {
        // borrowch workaround
        let rows = self.tty_ioctl_config.rows;
        let cols = self.tty_ioctl_config.cols;

        {
            let mut screen = self.layout.write().unwrap();

            screen.root
                .as_mut()
                .unwrap()
                .children
                .as_mut()
                .unwrap()
                .push(Node::leaf(Widget::new_with_program_id(
                        "status_line".to_string(),
                        vterm_sys::ScreenSize { cols: cols, rows: 1 },
                        )));

            screen.calculate_layout();
        }

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

    /// For now we only expect this once, so create a pane and enter program mode aimed at it
    fn add_program(&mut self, server_id: String, program: Program) {
        self.mode = Box::new(ProgramMode { program_id: program.id.clone() });

        {
            let mut screen = self.layout.write().unwrap();

            screen.root
                .as_mut()
                .unwrap()
                .children
                .as_mut()
                .unwrap()
                .insert(0, Node::leaf(Widget::new_with_program_id(
                        program.id.clone(),
                        vterm_sys::ScreenSize { cols: 80, rows: 24 },
                        )));

            screen.calculate_layout();
        }

        self.servers.add_program(&server_id, program);

        self.damage_status_line();
    }

    fn damage_status_line(&self) {
        trace!("damage_status_line for mode {:?}", self.mode);

        let found_status_line = {
            let screen = self.layout.read().unwrap();
            if let Some(widget) = screen.root.as_ref().unwrap().widgets().find(|w| w.program_id == "status_line".to_string()) {
                true
            } else {
                false
            }
        };

        // Draw it
        if found_status_line {
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
            trace!("no status line widget");
        }
    }
}
