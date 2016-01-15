use vterm_sys;
use super::*;
use super::servers::*;
use super::layout::*;
use super::modal::*;
use std::sync::mpsc::*;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::io::prelude::*;
use uuid::Uuid;

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
    selected_program_id: Option<String>,
}

static STATUS_LINE: &'static str = "status_line";

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
            selected_program_id: None,
        };
        worker.init();
        worker
    }

    /// creates an initial window, status pane etc
    fn init(&mut self) {
        let status_line = Node::leaf(
            STATUS_LINE.to_string(),
            NodeOptions { height: Some(1), ..Default::default() }
        );

        let mut layout = self.layout.write().unwrap();
        layout.root
            .children
            .push(status_line);
        layout.calculate_layout();
        drop(layout);
        self.draw_worker_tx.send(ClientMsg::LayoutDamage);

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
                ClientMsg::ProgramAdd { server_id, program_id } => self.add_program(server_id, program_id),
                ClientMsg::UserInput { bytes } => {
                    self.modal_key_handler.write(&bytes); // todo check result here
                    while let Some(user_action) = self.modal_key_handler.actions_queue.pop() {
                        match user_action {
                            modal::UserAction::ModeChange { name }           => self.mode_change(&name),
                            modal::UserAction::ProgramFocus                  => self.program_focus_cmd(),
                            modal::UserAction::ProgramInput { bytes: fites } => self.program_input_cmd(fites),
                            modal::UserAction::ProgramStart                  => self.program_start_cmd(),
                            modal::UserAction::Quit                          => error!("user action {:?} is not implemented", user_action),
                            modal::UserAction::UnknownInput { bytes: fites } => error!("unknown input for mode {}: {:?}", self.modal_key_handler.mode_name(), fites),
                        }
                    }
                },
                _ => warn!("unhandled msg {:?}", msg),
            }
        }
    }

    fn program_input_cmd(&self, bytes: Vec<u8>) {
        // for now, always send bytes to the first program
        if let Some(program_id) = self.leaf_names().first() {
            if let Some(server) = self.servers.iter().find(|s| s.programs.iter().any(|p| p.id == *program_id)) {
                trace!("sending input to program {} {:?}", &program_id, &bytes);
                server.tx.send(::server::ServerMsg::ProgramInput {
                    program_id: program_id.clone(),
                    bytes: bytes,
                });
            }
        }
    }

    fn program_start_cmd(&self) {
        if let Some(server) = self.servers.first() {
            trace!("starting program");
            // for now, always start bash
            let command_and_args: Vec<String> = vec!["bash".to_string()];
            server.tx.send(::server::ServerMsg::ProgramStart {
                command_and_args: command_and_args,
                program_id: Uuid::new_v4().to_hyphenated_string(),
            }).unwrap();
        }
    }

    /// The main point of the command, which is to direct user keys to the program, has already
    /// been done by the modal state machine. All we have to do is make sure a program is selected.
    fn program_focus_cmd(&mut self) {
        trace!("program_focus_cmd {:?}", self.selected_program_id);
        let leaf_names: Vec<String> = self.leaf_names().into_iter().filter(|n| n != STATUS_LINE).collect();

        let valid_selection = if let Some(program_id) = self.selected_program_id.clone() {
            if leaf_names.iter().any(|n| *n == program_id) { true }
            else { false }
        }
        else { false };

        if valid_selection {
            trace!("1");
            let mut layout = self.layout.write().unwrap();
            for c in layout.root.children.iter_mut() {
                if c.value == self.selected_program_id.clone().unwrap() {
                    c.has_border = true
                }
                else {
                    c.has_border = false
                }
            }
            layout.calculate_layout();
        }
        else {
            trace!("2");
            self.program_select_next();
        }
    }

    fn program_select_next(&mut self) {
        let leaf_names: Vec<String> = self.leaf_names().into_iter().filter(|n| n != STATUS_LINE).collect();

        if let Some(program_id) = self.selected_program_id.clone() {
            trace!("3");
            if let Some(mut i) = leaf_names.iter().position(|n| *n == program_id) {
                trace!("4");
                i += 1;
                if i < leaf_names.len() {
                    trace!("5");
                    self.selected_program_id = Some(leaf_names[i].clone());
                }
                else {
                    trace!("6");
                    self.selected_program_id = Some(leaf_names[0].clone());
                }
            }
            else {
                trace!("7");
                self.selected_program_id = None;
            }
        }

        if self.selected_program_id.is_none() && leaf_names.len() > 0 {
            trace!("8");
            self.selected_program_id = Some(leaf_names[0].clone());
        }

        if let Some(program_id) = self.selected_program_id.clone() {
            trace!("9 {}", program_id);
            let mut layout = self.layout.write().unwrap();
            for c in layout.root.children.iter_mut() {
                if c.value == program_id {
                    trace!("9a {}", c.value);
                    c.has_border = true
                }
                else {
                    trace!("9b {}", c.value);
                    c.has_border = false
                }
            }
            layout.calculate_layout();
        }

        self.draw_worker_tx.send(ClientMsg::LayoutDamage);
    }

    fn add_program(&mut self, server_id: String, program_id: String) {
        self.servers.add_program(&server_id, Program { id: program_id.clone(), is_subscribed: true });

        // for now, always show it in a pane
        let leaf = Node::leaf(
            program_id.clone(),
            NodeOptions { height: Some(24), width: Some(80), ..Default::default() }
        );

        let mut layout = self.layout.write().unwrap();
        layout.root
            .children
            .insert(0, leaf);
        layout.calculate_layout();
        self.draw_worker_tx.send(ClientMsg::LayoutDamage);
    }

    fn damage_status_line(&self) {
        trace!("damage_status_line for mode {:?}", self.modal_key_handler.mode_name());

        let found_status_line = {
            let layout = self.layout.read().unwrap();
            if let Some(node) = layout.root.descendants().find(|n| n.is_leaf() && n.value == STATUS_LINE.to_string()) {
                true
            } else {
                false
            }
        };

        if !found_status_line {
            warn!("no status line node");
            return;
        }

        // Draw it
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

        // Does this make sense? A status line is not a program.
        self.draw_worker_tx.send(ClientMsg::ProgramDamage {
            program_id: STATUS_LINE.to_string(),
            cells: cells,
        });
    }

    fn mode_change(&mut self, name: &str) {
        self.damage_status_line();
    }

    fn leaf_names(&self) -> Vec<String> {
        self.layout
            .read()
            .unwrap()
            .root
            .descendants()
            .filter(|n| n.is_leaf())
            .map(|n| n.value.clone())
            .collect()
    }
}
