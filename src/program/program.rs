extern crate log;
extern crate pty;
extern crate termios;
extern crate libvterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;
extern crate uuid;

use ::window::WindowMsg;
use libvterm_sys::*;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::os::unix::prelude::*;
use std::ptr;
use std::sync::mpsc;
use std::thread;
use super::*;

pub struct Program {
    pub child_pid: i32,
    pub id: String,
    pub tx: mpsc::Sender<WindowMsg>,
    pub msg_listener_tx: mpsc::Sender<ProgramMsg>,
    pub size: ScreenSize,
    pub pty: RawFd,
}

impl Program {
    pub fn new(command_and_args: &Vec<String>,
               tx: mpsc::Sender<WindowMsg>,
               size: &ScreenSize)
               -> (Program, Vec<thread::JoinHandle<()>>) {
        info!("forking");
        let child = fork(command_and_args);

        let (listener_tx, listener_rx) = mpsc::channel::<WindowMsg>();

        info!("program started");

        let fd = child.pty().unwrap().as_raw_fd();

        let mut threads = vec![];

        let program_id = uuid::Uuid::new_v4().to_simple_string();
        let msg_listener = super::msg_listener::MsgListener::new(&program_id, listener_tx.clone());
        let msg_listener_tx = msg_listener.tx.clone();
        threads.push(msg_listener.spawn());

        let io = unsafe { File::from_raw_fd(fd) };
        let pty_reader = super::pty_reader::PtyReader::new(io, msg_listener_tx.clone());
        threads.push(pty_reader.spawn());

        // let the window know we exist
        tx.send(WindowMsg::AddProgram {
            program_id: program_id.clone(),
            rx: listener_rx,
        }).unwrap();

        let program = Program {
            child_pid: child.pid(),
            id: program_id,
            tx: listener_tx.clone(),
            size: size.clone(), // todo: resize pty with this info
            pty: fd,
            msg_listener_tx: msg_listener_tx,
        };

        (program, threads)
    }
}

fn fork(command_and_args: &Vec<String>) -> pty::Child {
    match pty::fork() {
        Ok(child) => {
            if child.pid() == 0 {
                let cstrings: Vec<CString> = command_and_args.iter()
                                                             .map(|s| {
                                                                 let bytes = s.clone().into_bytes();
                                                                 CString::new(bytes).unwrap()
                                                             })
                                                             .collect();

                let mut ptrs: Vec<*const libc::c_char> = (&cstrings)
                                                             .iter()
                                                             .map(|s| s.as_ptr())
                                                             .collect();

                ptrs.push(ptr::null());

                let ret = unsafe { libc::execvp(*ptrs.as_ptr(), ptrs.as_mut_ptr()) };
                panic!("error {} in execvp {}", ret, io::Error::last_os_error());
            } else {
                info!("got vim child process");
                child
            }
        }
        Err(e) => {
            panic!("pty::fork error: {}", e);
        }
    }
}
