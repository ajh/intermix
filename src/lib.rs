#![feature(mpsc_select)]
#![feature(convert)]

extern crate docopt;
extern crate libc;
extern crate vterm_sys;
#[macro_use]
extern crate log;
extern crate pty;
extern crate rustc_serialize;
extern crate term;
extern crate termios;
extern crate uuid;

use vterm_sys::*;
use std::io;
use std::os::unix::io::RawFd;
use std::sync::mpsc::*;
use std::thread;
use term::terminfo::*;

pub mod client;
pub mod server;
