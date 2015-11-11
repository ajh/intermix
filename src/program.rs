extern crate log;
extern crate pty;
extern crate termios;
extern crate libvterm_sys;
extern crate term;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;
extern crate uuid;

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
        threads.push(spawn_stdin_to_pty_thr(&child));
        threads.push(spawn_pty_reader(&child, program_event_tx.clone()));

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

fn spawn_pty_reader(pty: &pty::Child, tx: mpsc::Sender<ProgramEvent>) -> thread::JoinHandle<()> {
    // thread for sending stdin to pty
    let pty = pty.pty().unwrap().clone();

    thread::spawn(move || {
        let mut buf = [0 as u8; 4096];
        let reader = unsafe { File::from_raw_fd(pty.as_raw_fd()) };
        let mut reader = BufReader::new(reader);

        // create vterm instance.
        let mut vterm = VTerm::new(24, 80);
        vterm.set_utf8(true);
        let vterm_event_rx = vterm.receive_screen_events();
        vterm.get_screen().reset(true); // boilerplate to avoid segfault

        info!("starting pty -> stdout thread");
        loop {
            // block until read
            let bytes = match reader.read(&mut buf) {
                Ok(num_bytes) => {
                    if num_bytes == 0 {
                        error!("zero bytes reading from pty");
                        break;
                    }
                    &buf[0..num_bytes]
                },
                Err(_) => {
                    error!("error reading from pty");
                    break;
                }
            };

            vterm.write(bytes);
            vterm.get_screen().flush_damage();

            handle_screen_events(&mut vterm, &vterm_event_rx, &tx);

            // Not sure I need this since the read is blocking anyway
            thread::sleep_ms(10);
        }
        info!("ending pty -> stdout thr");
    })
}

fn handle_screen_events(vterm: &mut VTerm, rx: &mpsc::Receiver<ScreenEvent>, tx: &mpsc::Sender<ProgramEvent>) {
    while let Ok(event) = rx.try_recv() {
        match event {
            ScreenEvent::Bell => info!("bell"),
            ScreenEvent::Damage{rect} => send_program_damage_event(vterm, &rect, tx),
            ScreenEvent::MoveCursor{new, old, is_visible} => info!("move cursor new {:?} old {:?} is_visible {:?}", new, old, is_visible),
            ScreenEvent::MoveRect{dest, src} => info!("move rect dest {:?} src {:?}", dest, src),
            ScreenEvent::Resize{rows, cols} => info!("resize rows {:?} cols {:?}", rows, cols),
            ScreenEvent::SbPopLine{cells: _} => info!("sb push line"),
            ScreenEvent::SbPushLine{cells: _} => info!("sb push line"),
            ScreenEvent::AltScreen{ is_true: _ } => info!("AltScreen"),
            ScreenEvent::CursorBlink{ is_true: _ } => info!("CursorBlink"),
            ScreenEvent::CursorShape{ value: _ } => info!("CursorShape"),
            ScreenEvent::CursorVisible{ is_true: _ } => info!("CursorVisible"),
            ScreenEvent::IconName{ text: _} => info!("IconName"),
            ScreenEvent::Mouse{ value: _ } => info!("Mouse"),
            ScreenEvent::Reverse{ is_true: _ } => info!("Reverse"),
            ScreenEvent::Title{ text: _} => info!("Title"),
        }
    }
}

fn send_program_damage_event(vterm: &mut VTerm, rect: &Rect, tx: &mpsc::Sender<ProgramEvent>) {
    //trace!("damage {:?}", rect);
    let mut pos: Pos = Default::default();

    let mut cells: Vec<ScreenCell> = Vec::new(); // capacity is known here FYI

    for row in rect.start_row..rect.end_row {
        pos.row = row as i16;
        for col in rect.start_col..rect.end_col {
            pos.col = col as i16;
            cells.push(vterm.get_screen().get_cell(&pos));
        }
    }

    let event = ProgramEvent::Damage { program_id: "not implemented".to_string(), cells: cells };
    tx.send(event).unwrap();
}
