use vterm_sys;
use super::*;
use super::state::*;
use super::layout::*;
use super::modal::*;
use std::sync::mpsc::*;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::io::prelude::*;

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
    pub modal_key_handler: modal::ModalKeyHandler,
    pub tty_ioctl_config: TtyIoCtlConfig,
    pub layout: Arc<RwLock<Layout>>,
}

impl MainWorker {
    pub fn spawn(draw_worker_tx: Sender<ClientMsg>, tty_ioctl_config: TtyIoCtlConfig) -> (Sender<ClientMsg>, Arc<RwLock<Layout>>, JoinHandle<()>) {
        let (tx, rx) = channel::<ClientMsg>();
        let layout = Arc::new(RwLock::new(Layout::new(
                    Size {
                        rows: tty_ioctl_config.rows,
                        cols: tty_ioctl_config.cols,
                    },
                    Node::row(
                        NodeOptions {
                            vertical_align: VerticalAlign::Bottom,
                            height: Some(tty_ioctl_config.rows),
                            ..Default::default()},
                        vec![]
                    ))));
        let layout_clone = layout.clone();

        info!("spawning main worker");
        let handle = thread::spawn(move || {
            let mut worker = MainWorker::new(draw_worker_tx, rx, tty_ioctl_config, layout);
            worker.enter_listener_loop();
            info!("exiting main worker");
        });

        (tx, layout_clone, handle)
    }

    fn new(draw_worker_tx: Sender<ClientMsg>, rx: Receiver<ClientMsg>, tty_ioctl_config: TtyIoCtlConfig, layout: Arc<RwLock<Layout>>) -> MainWorker {
        let mut worker = MainWorker {
            draw_worker_tx: draw_worker_tx,
            rx: rx,
            servers: Default::default(),
            modal_key_handler: modal::ModalKeyHandler::new_with_graph(),
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
            let mut layout = self.layout.write().unwrap();

            let status_line = Node::leaf_v2(
                "status_line".to_string(),
                NodeOptions { height: Some(1), width: Some(cols), ..Default::default() }
            );

            layout.root
                .children
                .push(status_line);


            layout.calculate_layout();
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
                    self.modal_key_handler.write(&bytes);
                    while let Some(user_action) = self.modal_key_handler.actions_queue.pop() {
                        match user_action {
                            modal::UserAction::ModeChange { name } => self.change_mode(&name),
                            modal::UserAction::ProgramStart => self.program_start_cmd(),
                            modal::UserAction::ProgramInput { bytes: fites } => self.program_input_cmd("bash-123".to_string(), fites),
                            modal::UserAction::Quit => return,
                            modal::UserAction::UnknownInput { bytes: fites } => error!("unknown input for mode {}: {:?}", self.modal_key_handler.mode_name(), fites),
                        }
                    }
                },
                _ => warn!("unhandled msg {:?}", msg),
            }
        }
    }

    fn program_input_cmd(&self, program_id: String, bytes: Vec<u8>) {
        if let Some(server) = self.servers.iter().find(|s| s.programs.iter().any(|p| p.id == program_id)) {
            trace!("sending input to program {} {:?}", &program_id, &bytes);
            server.tx.send(::server::ServerMsg::ProgramInput {
                program_id: program_id,
                bytes: bytes,
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
        {
            let mut layout = self.layout.write().unwrap();

            let leaf = Node::leaf_v2(
                program.id.clone(),
                NodeOptions { height: Some(24), width: Some(80), ..Default::default() }
            );

            layout.root
                .children
                .insert(0, leaf);

            layout.calculate_layout();
        }

        self.servers.add_program(&server_id, program);

        self.damage_status_line();
    }

    fn damage_status_line(&self) {
        trace!("damage_status_line for mode {:?}", self.modal_key_handler.mode_name());

        let found_status_line = {
            let layout = self.layout.read().unwrap();
            if let Some(node) = layout.root.descendants().find(|n| n.is_leaf() && n.value == "status_line".to_string()) {
                true
            } else {
                false
            }
        };

        // Draw it
        if found_status_line {
            let mut cells = vec![];
            for (i, char) in self.modal_key_handler.mode_name().chars().enumerate() {
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
            warn!("no status line node");
        }
    }

    fn change_mode(&mut self, name: &str) {
        self.damage_status_line();
    }
}
