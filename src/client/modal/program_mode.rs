use super::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ProgramMode {
    pub program_id: String
}

impl Mode for ProgramMode {
    fn input(&self, bytes: Vec<u8>, windows: &mut ::client::state::Windows, servers: &mut ::client::state::Servers) {
        // for now, send it to the first program
        if let Some(server) = servers.first() {
            if let Some(program) = server.programs.first() {
                trace!("sending input to program {}", program.id);
                server.tx.send(::server::ServerMsg::ProgramInput {
                    program_id: program.id.clone(),
                    bytes: bytes,
                });
            }
        }
    }

    fn display(&self) -> String {
        "program-mode".to_string()
    }
}
