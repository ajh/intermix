mod vte_worker;
mod pty_reader;

use vterm_sys::*;
use self::pty_reader::*;
use self::vte_worker::*;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::os::unix::prelude::*;
use std::ptr;
use std::sync::mpsc::*;
use std::thread;
use super::*;
use libc;
use pty;

pub enum VteWorkerMsg {
    PtyRead { bytes: Vec<u8> },
    PtyReadError,
    PtyReadZero,
    RequestRedrawRect { rect: Rect },
}

pub struct Program {
    pub child_pid: i32,
    pub id: String,
    pub size: ScreenSize,
    pub pty: File,
}

impl Program {
    pub fn new(id: &str,
               command_and_args: &Vec<String>,
               server_tx: Sender<ServerMsg>,
               size: &ScreenSize)
               -> (Program, Vec<thread::JoinHandle<()>>) {

        let child = fork(id, command_and_args);

        let mut threads = vec![];

        let (vte_tx, handle) = VteWorker::spawn(server_tx.clone(), id);
        threads.push(handle);

        let fd = child.pty().unwrap().as_raw_fd();
        let io = unsafe { File::from_raw_fd(fd) };
        let handle = PtyReader::spawn(io, vte_tx.clone(), id);
        threads.push(handle);

        let program = Program {
            child_pid: child.pid(),
            id: id.to_string(),
            size: size.clone(), // todo: resize pty with this info
            pty: unsafe { File::from_raw_fd(fd) },
        };

        (program, threads)
    }
}

fn fork(id: &str, command_and_args: &Vec<String>) -> pty::Child {
    info!("forking program {}", id);

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
                child
            }
        }
        Err(e) => {
            panic!("pty::fork error: {}", e);
        }
    }
}
