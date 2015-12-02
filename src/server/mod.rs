mod program;

use vterm_sys::{self, ScreenCell, Rect, Pos};
use self::program::*;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::prelude::*;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub enum ServerMsg {
    Quit,

    ProgramDamage { program_id: String, cells: Vec<ScreenCell> },
    ProgramInput { program_id: String, bytes: Vec<u8> },
    ProgramKill { program_id: String, signal: u8 },
    ProgramMoveCursor { program_id: String, new: Pos, old: Pos, is_visible: bool },
    ProgramRedrawRect { program_id: String, rect: Rect },
    ProgramStart { program_id: String, command_and_args: Vec<String> },

    ClientAdd { client: Client },
    ClientUpdate { client: Client },
    ClientRemote { client_id: String },
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

                ServerMsg::ProgramDamage { program_id, cells } => {
                    self.send_msg_to_clients(
                        ::client::ClientMsg::ProgramDamage { program_id: program_id, cells: cells },
                        true
                    );
                },
                ServerMsg::ProgramInput { program_id, bytes } => self.program_input(program_id, bytes),
                ServerMsg::ProgramKill { program_id, signal } => {},
                ServerMsg::ProgramMoveCursor { program_id, new, old, is_visible } => {},
                ServerMsg::ProgramRedrawRect { program_id, rect } => {},

                // need client id here
                ServerMsg::ProgramStart { program_id, command_and_args } => self.start_program(program_id, command_and_args),

                ServerMsg::ClientAdd { client } => {
                    self.clients.push(client);
                },
                ServerMsg::ClientUpdate { client } => {},
                ServerMsg::ClientRemote { client_id } => {},
            }
        }
    }

    fn program_input(&mut self, program_id: String, bytes: Vec<u8>) {
        trace!("input for program {:?}", program_id);
        if let Some(mut program) = self.programs.iter_mut().find( |p| p.id == program_id ) {
            program.pty.write_all(unsafe { bytes.as_slice() });
        } else {
            trace!("couldnt send input to unknown program {:?}", program_id);
        }
    }

    fn send_msg_to_clients(&self, msg: ::client::ClientMsg, hard: bool) {
        trace!("sending msg {:?} to {} clients", msg, self.clients.len());
        for client in &self.clients {
            let result = client.tx.send(msg.clone());
            if hard { result.expect("didnt send"); }
        }
    }

    fn start_program(&mut self, id: String, command_and_args: Vec<String>) {
        // FIXME: get size from client
        let size = vterm_sys::ScreenSize { rows: 24, cols: 80 };
        let (program, threads) = Program::new(&id, &command_and_args, self.tx.clone(), &size);
        self.programs.push(program);

        if let Some(client) = self.clients.first() {
            client.tx.send(::client::ClientMsg::ProgramAdd {
                server_id: "some server".to_string(),
                program: ::client::state::Program {
                    id: id,
                    is_subscribed: true,
                }
            });
        }
    }

    //pub fn start_new_window(&mut self) -> Vec<thread::JoinHandle<()>> {
        //let (window, threads) = ::Window::new();
        //window.lock().unwrap().start();
        //self.windows.push(window);

        //threads
    //}

    //pub fn stop(&mut self) {
        //for w in &self.windows {
            //w.lock().unwrap().stop();
        //}
    //}

    ///// Start program in a new pane
    //pub fn start_program_in_new_pane(&mut self,
                                     //command_and_args: &Vec<String>,
                                     //size: &ScreenSize,
                                     //offset: &Pos)
                                     //-> Vec<thread::JoinHandle<()>> {
        //info!("starting program");
        //let window = self.windows.first().unwrap();
        //let (program, threads) = ::Program::new(command_and_args,
                                                //window.lock().unwrap().tx.clone(),
                                                //size);

        //// use window to create the pane?
        //let pane = Pane {
            //size: size.clone(),
            //offset: offset.clone(),
            //program_id: program.id.clone(),
            //program_msg_tx: program.msg_listener_tx.clone(),
        //};
        //window.lock().unwrap().panes.push(pane);

        //self.programs.push(program);

        //threads
    //}

    ///// modes will generalize how this works
    //pub fn first_program_pty_fd(&self) -> RawFd {
        //(&self.programs).first().unwrap().pty
    //}
}
