extern crate docopt;
extern crate libc;
extern crate libvterm_sys;
extern crate log;
extern crate log4rs;
extern crate pty;
extern crate rustc_serialize;
extern crate term;
extern crate termios;
extern crate uuid;

mod program;

use libvterm_sys::*;
use self::program::*;
use std::os::unix::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::*;

pub enum ServerMsg {
    Quit,

    ProgramInput { program_id: String, bytes: Vec<u8> },
    ProgramStart { command_and_args: Vec<String> },
    ProgramKill { program_id: String, signal: u8 },
    ProgramRedrawRect { program_id: String, rect: Rect },
    ProgramDamage { program_id: String, cells: Vec<ScreenCell> },
    ProgramMoveCursor { program_id: String, new: Pos, old: Pos, is_visible: bool },

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

        info!("starting server");
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
            match self.rx.recv() {
                Ok(msg) => self.handle(msg),
                Err(_) => break,
            }
        }
    }

    fn handle(&self, msg: ServerMsg) {
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
