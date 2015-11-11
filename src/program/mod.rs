extern crate log;
extern crate pty;
extern crate termios;
extern crate libvterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;
extern crate uuid;

mod event_handler;

pub use self::event_handler::*;

use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::ptr;
use std::sync::mpsc;
use std::thread;
use libvterm_sys::*;

pub enum ProgramEvent {
    Damage { program_id: String, cells: Vec<ScreenCell> },
    AddProgram { program_id: String, rx: mpsc::Receiver<ProgramEvent> }
}

pub struct ProgramAttachments {
    pub thread_handles: Vec<thread::JoinHandle<()>>,
    pub event_rx: mpsc::Receiver<ProgramEvent>,
}

pub struct Program {
    child_pid: i32,
    id: String,
    tx: mpsc::Sender<ProgramEvent>,
}

impl Program {
    pub fn new(command_and_args: &Vec<String>) -> (Program, ProgramAttachments) {
        info!("forking");
        let child = fork(command_and_args);

        let (program_event_tx, program_event_rx) = mpsc::channel::<ProgramEvent>();

        info!("spawning threads");
        let mut threads = vec!();
        // this should be somewhere else
        threads.push(spawn_stdin_to_pty_thr(&child));

        {
            let pty = child.pty().unwrap().clone();
            let io = unsafe { File::from_raw_fd(pty.as_raw_fd()) };
            let event_handler = EventHandler::new(io, program_event_tx.clone());
            event_handler.spawn();
        }

        info!("program started");

        let attachments = ProgramAttachments {
            thread_handles: threads,
            event_rx: program_event_rx,
        };

        let program = Program {
            child_pid: child.pid(),
            id: uuid::Uuid::new_v4().to_simple_string(),
            tx: program_event_tx,
        };

        (program, attachments)
    }
}

fn fork(command_and_args: &Vec<String>) -> pty::Child {
    match pty::fork() {
        Ok(child) => {
            if child.pid() == 0 {
                let cstrings: Vec<CString> = command_and_args.iter().map(|s| {
                    let bytes = s.clone().into_bytes();
                    CString::new(bytes).unwrap()
                }).collect();

                let mut ptrs: Vec<*const libc::c_char> = (&cstrings).iter()
                    .map(|s| s.as_ptr())
                    .collect();

                ptrs.push(ptr::null());

                let ret = unsafe { libc::execvp(*ptrs.as_ptr(), ptrs.as_mut_ptr()) };
                panic!("error {} in execvp {}", ret, io::Error::last_os_error());
            }
            else {
                info!("got vim child process");
                child
            }
        },
        Err(e) => {
            panic!("pty::fork error: {}", e);
        }
    }
}

/// This thread should really read from a receiver, not from stdin
fn spawn_stdin_to_pty_thr(pty: &pty::Child) -> thread::JoinHandle<()> {
    // thread for sending stdin to pty
    let mut pty = pty.pty().unwrap().clone();
    thread::spawn(move || {
        let mut buf = [0 as u8; 4096];
        info!("starting stdin -> pty thread");
        loop {
            match io::stdin().read(&mut buf) {
                Ok(num_bytes) => {
                    if num_bytes == 0 { break };

                    //if buf.iter().find(|&x| *x == terminfo::CTRL_C).is_some() {
                        //info!("CTRL_C detected");
                        //exit();
                    //}

                    match pty.write_all(&buf[0..num_bytes]) {
                        Ok(_) => {},
                        Err(msg) => {
                            error!("{}", msg);
                            break;
                        },
                    }
                },
                Err(msg) => {
                    error!("{}", msg);
                    break;
                },
            }
        }
        info!("ending stdin -> pty thread");
    })
}
