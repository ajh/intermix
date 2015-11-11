extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;
extern crate libvterm_sys;

use std::os::unix::io::RawFd;
use std::io::prelude::*;
use std::io;
use term::terminfo::*;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use ::program::ProgramEvent;

pub struct EventHandler {
    // deal with Program Events for now, until we have window events implemented
    pub receivers: Vec<Box<Receiver<ProgramEvent>>>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {receivers: vec!()}
    }

    // just loop over the one receiver, deal with multiple receivers and changes to what receivers
    // we have later
    pub fn spawn(mut self) -> thread::JoinHandle<()> {
        info!("spawning event handler");
        thread::spawn(move || {
            let select = Select::new();
            let mut handles: Vec<Box<Handle<_>>> = vec!();

            let mut painter: ::tty_painter::TtyPainter = Default::default();

            // add initial receivers
            for rx in &self.receivers {
                // lose ownership info
                let rx = unsafe { & *((&**rx) as *const _) };
                handles.push(Box::new(select.handle(rx)));
                unsafe { handles.last_mut().unwrap().add(); }
            }

            while handles.len() > 0 {
                let id = select.wait();
                let handle = match handles.iter_mut().find(|h| { h.id() == id } ) {
                    Some(mut h) => unsafe { &mut *((&mut **h) as *mut Handle<_>) },
                    None => panic!("error: handle for id {} not found", id),
                };

                match handle.recv() {
                    Ok(event) => match event {
                        ProgramEvent::Damage{program_id: _, cells} => {
                            painter.draw_cells(&cells, &mut io::stdout(), &libvterm_sys::Pos { row: 10, col: 5 });
                        },
                        ProgramEvent::AddProgram{program_id: _, rx: rx} => {
                            info!("add program");
                            self.receivers.push(Box::new(rx));
                            let rx = unsafe { & *(&**self.receivers.last().unwrap() as *const _) };
                            handles.push(Box::new(select.handle(rx)));
                            unsafe { handles.last_mut().unwrap().add(); }
                        }
                    },
                    Err(_) => {
                        unsafe { handle.remove() };
                        match handles.iter().position(|h| h.id() == handle.id()) {
                            Some(i) => handles.remove(i),
                            None => panic!("can't remove handle, not in vec"),
                        };
                    },
                };
            }
        })
    }
}