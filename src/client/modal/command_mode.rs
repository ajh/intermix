use ::client::main_worker::*;
use std::fmt::Debug;
use super::*;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct CommandMode {
    pub accumulator: Vec<u8>,
}

impl CommandMode {
    pub fn new() -> CommandMode {
        Default::default()
    }
}

impl Mode for CommandMode {
    fn input(&mut self, bytes: Vec<u8>) -> Option<UserCmd> {
        if bytes == b"s" {
            Some(UserCmd::ProgramStart)
        } else {
            None
        }
    }

    fn display(&self) -> String {
        "command-mode".to_string()
    }
}
