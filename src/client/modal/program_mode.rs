use ::client::main_worker::*;
use std::fmt::Debug;
use super::*;
use super::inband_key_detector::*;
use std::io::prelude::*;

#[derive(Debug)]
pub struct ProgramMode {
    pub program_id: String,
    detector: InbandKeyDetector,
}

impl ProgramMode {
    pub fn new(program_id: String) -> ProgramMode {
        let key = 98; // 'b'
        ProgramMode {
            program_id: program_id,
            detector: InbandKeyDetector::new(key),
        }
    }
}

impl Mode for ProgramMode {
    fn input(&mut self, mut bytes: Vec<u8>) -> Option<UserCmd> {
        self.detector.write(bytes.as_slice());
        trace!("{:?}", self.detector);
        if self.detector.key_found() {
            Some(UserCmd::ModeChange { new_mode: "command".to_string() })
        }
        else {
            let mut bytes = vec![];
            self.detector.read_to_end(&mut bytes);
            Some(UserCmd::ProgramInput { program_id: self.program_id.clone(), bytes: bytes })
        }
    }

    fn display(&self) -> String {
        "program-mode".to_string()
    }
}
