#![feature(libc)]
extern crate libc;

use pty;
use std::ffi::CString;
use std::ptr;
use std::io::Write;
use std::io::Read;

pub struct Program {
    child: Option<pty::Child>,
    command: String,
    name: String,
}

impl Program {
    pub fn new(name: String, command: String) -> Program {
        Program {
            child: None,
            command: command,
            name: name,
        }
    }

    pub fn run(&mut self) -> Result<String, String> {
        match pty::fork() {
            Ok(child) => {
                if child.pid() == 0 {
                    // run the command
                    let cmd  = CString::new(self.command.clone()).unwrap().as_ptr();
                    let args = [cmd, ptr::null()].as_mut_ptr();

                    unsafe { libc::execvp(cmd, args) };
                    Err("never gets here".to_string())
                }
                else {
                    self.child = Some(child);
                    Ok("Looking good".to_string())
                }
            },
            Err(e) => {
                Err(format!("pty::fork error: {}", e).to_string())
            }
        }
    }

    pub fn write(&self, input: &[u8]) {
        self.child.iter().next().unwrap().pty().unwrap().write(input);
    }

    pub fn read(&self, output: &mut [u8]) {
        self.child.iter().next().unwrap().pty().unwrap().read(output);
    }
}
