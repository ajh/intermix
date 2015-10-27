extern crate libc;
extern crate tsm_sys;

use pty;
use std::ffi::CString;
use std::io::{self, Read, Write};
use std::ptr;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::os::unix::io::AsRawFd;

use screen::*;

pub struct Program {
    pub command:     String,
    pub name:        String,
    pub rows_count:  usize,
    pub cols_count:  usize,
    pub screen:      Arc<Mutex<Screen>>,
    child:       Option<pty::Child>,
}

//
// let program = Program::new('some prog', 'vim Cargo.toml', 50, 24)
//
//
//
// // returns a Writer and Reader for bytes
// // Also starts a thread to fill the reader with bytes from the pty
// let (input, output) = program.run() // returns a Writer and Reader for bytes
//
//
//
// Now consider vte
//
//  pub ptr: *mut TsmVte,
//  pub screen: ::Screen,
//  tx: Box<Sender<char>>,
//  pub rx: Receiver<char>,

impl Program {
    pub fn new(name: String, command: String, rows_count: usize, cols_count: usize) -> Program {
        Program {
            child:      None,
            cols_count: cols_count,
            command:    command,
            name:       name,
            rows_count: rows_count,
            screen: Arc::new(Mutex::new(Screen::new(rows_count, cols_count))),
        }
    }

    fn clone_pty_fd(&mut self) -> Result<pty::ChildPTY, ()> {
        Ok(self.child.as_mut().unwrap().pty().unwrap().clone())
    }

    pub fn run(&mut self) -> Result<(Sender<u8>, Receiver<()>), ()> {
        info!("starting program name={:?} command={:?} rows_count={} cols_count={}", self.name, self.command, self.rows_count, self.cols_count);

        self.fork_pty().unwrap();

        let (output_tx, output_rx) = channel::<()>();
        let fd = self.clone_pty_fd().unwrap();
        self.spawn_pty_to_screen_thr(fd, output_tx);

        let (input_tx, input_rx) = channel::<u8>();
        let fd = self.clone_pty_fd().unwrap();
        self.spawn_channel_to_pty_thr(fd, input_rx);

        Ok((input_tx, output_rx))
    }

    /// Not sure how to kill the forked process. Also maybe this should be in drop?
    pub fn stop(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn fork_pty(&mut self) -> Result<(), String> {
        match pty::fork() {
            Ok(child) => {
                if child.pid() == 0 {
                    // run the command
                    let cmd  = CString::new("vim").unwrap().as_ptr();
                    let arg1  = CString::new("Cargo.toml").unwrap().as_ptr();
                    let args = [cmd, arg1, ptr::null()].as_mut_ptr();

                    unsafe { libc::execvp(cmd, args) };
                    unreachable!();
                }
                else {
                    // have to wait for the exec to happen in the fork. Not sure how to improve
                    // this.
                    thread::sleep_ms(1000);

                    // this isn't working for some reason
                    let res = ::terminfo::set_win_size(
                        child.pty().unwrap().as_raw_fd(),
                        self.rows_count as u32,
                        self.cols_count as u32
                    );
                    if res.is_err() {
                        error!("{}", res.unwrap_err());
                    }

                    self.child = Some(child);
                    Ok(())
                }
            },
            Err(e) => {
                Err(format!("pty::fork error: {}", e).to_string())
            }
        }
    }

    /// Spawn a thread that receives from the input channel and writes to the pty
    fn spawn_channel_to_pty_thr(&mut self, mut pty: pty::ChildPTY, rx: Receiver<u8>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(byte) => pty.write(&[byte]).unwrap(),
                    Err(_) => break
                };
            }
            info!("leaving channel -> pty thread");
        })
    }

    /// Spawn a thread that reads from pty, sends it to the vte, and updates a Screen object that
    /// can be shared across threads. This is all done together in one thread because I'm not
    /// considering anything in libtsm threadsafe.
    ///
    /// the plan:
    /// block on reading the pty
    /// when something is read, pass it to vte via "input" call
    /// then use the vte's screen to ... do something
    /// another thread will use that to read the screen state and draw it
    fn spawn_pty_to_screen_thr(&self, mut pty: pty::ChildPTY, output_tx: Sender<()>) -> thread::JoinHandle<()> {
        let screen_arc = self.screen.clone();
        let (rows_count, cols_count) = (self.rows_count, self.cols_count);

        thread::spawn(move || {
            let mut vte = tsm_sys::Vte::new(rows_count, cols_count).unwrap();
            let mut buf = [0 as u8, 1024];
            loop {
                let mut bytes: &[u8];

                // block waiting to read
                match pty.read(&mut buf) {
                    Ok(num_bytes) => {
                        if num_bytes == 0 { break }
                        bytes = &buf[0..num_bytes];
                    },
                    Err(_) => {
                        error!("error reading from pty, trying");
                        continue;
                    }
                }

                // pass bytes to the vte
                vte.input(bytes);

                // update the screen
                let mut screen = screen_arc.lock().unwrap();
                for cell in vte.screen.borrow_mut().cells() {
                    screen.update_cell(cell.posx as usize, cell.posy as usize, cell.ch, cell.age as u32);
                };

                // signal that we've updated the screen
                // we should really send an Enum or something, not u8.
                output_tx.send(());
            }
            info!("leaving pty -> channel thread");
        })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.stop();
    }
}
