use ::client::main_worker::*;
use std::fmt::Debug;
use super::*;

#[derive(Debug)]
pub struct ProgramMode {
    pub program_id: String
}

impl Mode for ProgramMode {
    fn input(&self, worker: &MainWorker, bytes: Vec<u8>) -> Option<UserCmd> {
        Some(UserCmd::ProgramInput { program_id: self.program_id.clone(), bytes: bytes })
    }

    fn display(&self) -> String {
        "program-mode".to_string()
    }
}
