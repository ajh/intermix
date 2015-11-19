extern crate libc;
extern crate pty;
extern crate termios;
extern crate log4rs;
extern crate ioctl_rs as ioctl;
extern crate libvterm_sys;

use std::io;
use std::thread;
use std::sync::mpsc::*;
use std::sync::{Weak, Mutex};
use super::*;

pub struct EventHandler {
    window: Weak<Mutex<Window>>,
    // deal with Program Events for now, until we have window events implemented
    pub receivers: Vec<Box<Receiver<WindowMsg>>>,
}

impl EventHandler {
    pub fn new(window: Weak<Mutex<Window>>) -> EventHandler {
        EventHandler {
            window: window,
            receivers: vec![],
        }
    }

    // just loop over the one receiver, deal with multiple receivers and changes to what receivers
    // we have later
    pub fn spawn(mut self) -> thread::JoinHandle<()> {
        info!("spawning event handler");
        thread::spawn(move || {
            let select = Select::new();
            let mut handles: Vec<Box<Handle<_>>> = vec![];

            let mut painter: ::tty_painter::TtyPainter = Default::default();
            painter.reset(&mut io::stdout());

            // add initial receivers
            for rx in &self.receivers {
                // lose ownership info
                let rx = unsafe { &*((&**rx) as *const _) };
                handles.push(Box::new(select.handle(rx)));
                unsafe {
                    handles.last_mut().unwrap().add();
                }
            }

            while handles.len() > 0 {
                let id = select.wait();
                let handle = match handles.iter_mut().find(|h| h.id() == id) {
                    Some(mut h) => unsafe { &mut *((&mut **h) as *mut Handle<_>) },
                    None => panic!("error: handle for id {} not found", id),
                };

                match handle.recv() {
                    Ok(event) => match event {
                        WindowMsg::Damage{program_id, cells} => {
                            let offset = {
                                let window_arc = self.window.upgrade().unwrap();
                                let window = window_arc.lock().unwrap();
                                let pane = window.panes.iter().find(|p| p.program_id == program_id);
                                match pane {
                                    Some(p) => p.offset.clone(),
                                    None => libvterm_sys::Pos { row: 10, col: 5 },
                                }
                            };

                            painter.draw_cells(&cells, &mut io::stdout(), &offset);
                        }
                        WindowMsg::MoveCursor{program_id: _, new, old: _, is_visible} => {
                            // TODO: apply offset here
                            painter.move_cursor(new, is_visible, &mut io::stdout());
                        },
                        WindowMsg::SbPushLine{program_id, cells} => {
                            let (size, pos) = {
                                let window_arc = self.window.upgrade().unwrap();
                                let window = window_arc.lock().unwrap();
                                let pane = window.panes.iter().find(|p| p.program_id == program_id).unwrap();
                                (pane.size.clone(), pane.offset.clone())
                            };
                            painter.insert_line(&size, &pos, &mut io::stdout());
                            painter.draw_cells(&cells, &mut io::stdout(), &pos);
                        }
                        WindowMsg::AddProgram{program_id: _, rx} => {
                            info!("add program");
                            self.receivers.push(Box::new(rx));
                            let rx = unsafe { &*(&**self.receivers.last().unwrap() as *const _) };
                            handles.push(Box::new(select.handle(rx)));
                            unsafe {
                                handles.last_mut().unwrap().add();
                            }
                        }
                    },
                    Err(_) => {
                        unsafe { handle.remove() };
                        match handles.iter().position(|h| h.id() == handle.id()) {
                            Some(i) => handles.remove(i),
                            None => panic!("can't remove handle, not in vec"),
                        };
                    }
                };
            }
        })
    }
}
