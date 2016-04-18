#![feature(mpsc_select)]

#[macro_use]
extern crate log;
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
extern crate gj;
extern crate capnp;
extern crate capnp_rpc;

pub mod schema_capnp {
  include!(concat!(env!("OUT_DIR"), "/schema_capnp.rs"));
}

pub mod cell_buffer;
pub mod client;
pub mod server;
pub mod lull;
