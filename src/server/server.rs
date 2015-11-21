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

use std::thread;
use libvterm_sys::*;
use std::sync::{Arc, Mutex};
use std::os::unix::prelude::*;

pub struct Server {
    windows: Vec<Arc<Mutex<::Window>>>,
    programs: Vec<::Program>,
}

impl Server {
    pub fn new() -> Server {
        info!("starting server");
        Server {
            windows: vec![],
            programs: vec![],
        }
    }

    pub fn start_new_window(&mut self) -> Vec<thread::JoinHandle<()>> {
        let (window, threads) = ::Window::new();
        window.lock().unwrap().start();
        self.windows.push(window);

        threads
    }

    pub fn stop(&mut self) {
        for w in &self.windows {
            w.lock().unwrap().stop();
        }
    }

    /// Start program in a new pane
    pub fn start_program_in_new_pane(&mut self,
                                     command_and_args: &Vec<String>,
                                     size: &ScreenSize,
                                     offset: &Pos)
                                     -> Vec<thread::JoinHandle<()>> {
        info!("starting program");
        let window = self.windows.first().unwrap();
        let (program, threads) = ::Program::new(command_and_args,
                                                window.lock().unwrap().tx.clone(),
                                                size);

        // use window to create the pane?
        let pane = ::pane::Pane {
            size: size.clone(),
            offset: offset.clone(),
            program_id: program.id.clone(),
            program_msg_tx: program.msg_listener_tx.clone(),
        };
        window.lock().unwrap().panes.push(pane);

        self.programs.push(program);

        threads
    }

    /// modes will generalize how this works
    pub fn first_program_pty_fd(&self) -> RawFd {
        (&self.programs).first().unwrap().pty
    }
}
