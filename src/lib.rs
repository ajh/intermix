#![feature(mpsc_select)]
#![feature(libc)]

#[macro_use] extern crate log;
extern crate docopt;
extern crate ego_tree;
extern crate itertools;
extern crate libc;
extern crate pty;
extern crate rustc_serialize;
extern crate term;
extern crate termios;
extern crate uuid;
extern crate vterm_sys;

pub mod client;
pub mod server;
