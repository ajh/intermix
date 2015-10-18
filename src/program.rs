#![feature(libc)]
extern crate libc;

use pty;
use std::ffi::CString;
use std::ptr;
use std::io::Write;
use std::io::Read;

pub struct Program {
    child: pty::Child,
}

impl Program {
    pub fn new(command: String) -> Result<Program, String> {
        println!("0");
        match pty::fork() {
            Ok(child) => {
                println!("1: pid {:?}", child.pid());
                if child.pid() == 0 {
                    println!("2");
                    // run the command
                    let cmd  = CString::new(command).unwrap().as_ptr();
                    let args = [cmd, ptr::null()].as_mut_ptr();

                    println!("about to execvp!");
                    unsafe { libc::execvp(cmd, args) };
                    Err("never gets here".to_string())
                }
                else {
                    println!("3");
                    Ok(Program { child: child })
                }
            },
            Err(e) => {
                println!("4");
                Err(format!("pty::fork error: {}", e).to_string())
            }
        }
    }

    pub fn write(&self, input: &[u8]) {
        self.child.pty().unwrap().write(input);
    }

    pub fn read(&self, output: &mut [u8]) {
        self.child.pty().unwrap().read(output);
    }

    pub fn read_to_string(&self) -> String {
        let mut s = String::new();
        self.child.pty().unwrap().read_to_string(&mut s);
        s
    }
}

//extern crate libc;
//extern crate pty;

//use std::ffi::CString;
//use std::io::Read;
//use std::ptr;

//fn main()
//{
    //match pty::fork() {
        //Ok(child) => {
            //if child.pid() == 0 {
                //// Child process just exec `tty`
                //let cmd  = CString::new("tty").unwrap().as_ptr();
                //let args = [cmd, ptr::null()].as_mut_ptr();

                //unsafe { libc::execvp(cmd, args) };
            //}
            //else {
                //// Read output via PTY master
                //let mut output     = String::new();
                //let mut pty_master = child.pty().unwrap();

                //match pty_master.read_to_string(&mut output) {
                    //Ok(_nread) => println!("child tty is: {}", output.trim()),
                    //Err(e)     => panic!("read error: {}", e)
                //}

                //let _ = child.wait();
            //}
        //},
        //Err(e)    => panic!("pty::fork error: {}", e)
    //}
//}
