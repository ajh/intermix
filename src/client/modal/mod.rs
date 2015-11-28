mod program_mode;
mod command_mode;

use ::client::main_worker::*;
pub use self::program_mode::*;
pub use self::command_mode::*;
use std::fmt::Debug;

pub enum UserCmd {
    ProgramInput { program_id: String, bytes: Vec<u8> },
    ProgramStart,
}

pub trait Mode : Debug {
    fn input(&self, worker: &MainWorker, bytes: Vec<u8>) -> Option<UserCmd>;

    /// Maybe use the Display trait instead?
    fn display(&self) -> String;
}
