mod program;

use vterm_sys::{self, ScreenCell, Rect, Pos, Size};
use self::program::*;
use std::io::prelude::*;
use std::os::unix::prelude::*;
use std::sync::mpsc::*;
use std::thread;

pub enum ServerMsg {
    Quit,

    ProgramDamage {
        program_id: String,
        cells: Vec<ScreenCell>,
        rect: Rect,
    },
    ProgramInput {
        program_id: String,
        bytes: Vec<u8>,
    },
    ProgramKill {
        program_id: String,
        signal: u8,
    },
    ProgramMoveCursor {
        program_id: String,
        new: Pos,
        old: Pos,
        is_visible: bool,
    },
    ProgramRedrawRect {
        program_id: String,
        rect: Rect,
    },
    ProgramStart {
        program_id: String,
        command_and_args: Vec<String>,
    },

    ClientAdd {
        client: Client,
    },
    ClientUpdate {
        client: Client,
    },
    ClientRemote {
        client_id: String,
    },
}

/// a server's representation of a client
pub struct Client {
    pub id: String,

    /// replace with with cap'n proto or whatever
    pub tx: Sender<::client::ClientMsg>,
}

pub struct Server {
    tx: Sender<ServerMsg>,
    rx: Receiver<ServerMsg>,
    clients: Vec<Client>,
    programs: Vec<Program>,
}

impl Server {
    pub fn spawn() -> (Sender<ServerMsg>, thread::JoinHandle<()>) {
        let (tx, rx) = channel::<ServerMsg>();
        let tx_clone = tx.clone();

        info!("spawning server");
        let handle = thread::spawn(move || {
            let mut server = Server::new(tx, rx);
            server.enter_listener_loop();
            info!("exiting server");
        });

        (tx_clone, handle)
    }

    fn new(tx: Sender<ServerMsg>, rx: Receiver<ServerMsg>) -> Server {
        Server {
            tx: tx,
            rx: rx,
            clients: vec![],
            programs: vec![],
        }
    }

    fn enter_listener_loop(&mut self) {
        loop {
            let msg = match self.rx.recv() {
                Ok(msg) => msg,
                Err(_) => break,
            };

            match msg {
                ServerMsg::Quit => break,

                ServerMsg::ProgramDamage { program_id, cells, rect } => {
                    // What is Vec going to do with the cell data on the heap? Hopefully it will
                    // leave it alone?
                    self.send_msg_to_clients(::client::ClientMsg::ProgramDamage {
                                                 program_id: program_id,
                                                 cells: cells,
                                                 rect: rect,
                                             },
                                             true);
                }
                ServerMsg::ProgramInput { program_id, bytes } => {
                    self.program_input(program_id, bytes)
                }
                ServerMsg::ProgramKill { program_id, signal } => {}
                ServerMsg::ProgramMoveCursor { program_id, new, old, is_visible } => {}
                ServerMsg::ProgramRedrawRect { program_id: _, rect } => {}

                // need client id here
                ServerMsg::ProgramStart { program_id, command_and_args } => {
                    self.start_program(program_id, command_and_args)
                }

                ServerMsg::ClientAdd { client } => {
                    self.clients.push(client);
                }
                ServerMsg::ClientUpdate { client } => {}
                ServerMsg::ClientRemote { client_id } => {}
            }
        }
    }

    fn program_input(&mut self, program_id: String, bytes: Vec<u8>) {
        trace!("input for program {:?}", program_id);
        if let Some(mut program) = self.programs.iter_mut().find(|p| p.id == program_id) {
            program.pty.write_all(bytes.as_slice()).unwrap();
        } else {
            trace!("couldnt send input to unknown program {:?}", program_id);
        }
    }

    fn send_msg_to_clients(&self, msg: ::client::ClientMsg, hard: bool) {
        // trace!("sending msg {:?} to {} clients", msg, self.clients.len());
        for client in &self.clients {
            let result = client.tx.send(msg.clone());
            if hard {
                result.expect("didnt send");
            }
        }
    }

    fn start_program(&mut self, id: String, command_and_args: Vec<String>) {
        // FIXME: get size from client
        let size = Size::new(80, 24);
        let (program, _) = Program::new(&id, &command_and_args, self.tx.clone(), size);
        self.programs.push(program);

        if let Some(client) = self.clients.first() {
            client.tx
                  .send(::client::ClientMsg::ProgramAdd {
                      server_id: "some server".to_string(),
                      program_id: id,
                  })
                  .unwrap();
        }
    }
}
