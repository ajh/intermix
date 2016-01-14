mod program_mode;
mod command_mode;
mod inband_key_detector;
mod graph;
mod new;

use ::client::main_worker::*;
pub use self::program_mode::*;
pub use self::command_mode::*;
use std::fmt::Debug;
use std::io::prelude::*;

pub enum UserCmd {
    ProgramInput { program_id: String, bytes: Vec<u8> },
    ProgramStart,
    ModeChange { new_mode: String },
}

pub trait Mode : Debug + Write {
    fn input(&mut self, bytes: Vec<u8>) -> Option<UserCmd>;

    /// Maybe use the Display trait instead?
    fn display(&self) -> String;
}
