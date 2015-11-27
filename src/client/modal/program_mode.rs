use super::*;

pub struct ProgramMode {
    pub program_id: String
}

impl Mode for ProgramMode {
    fn input(&mut self, bytes: Vec<u8>, client_state: &mut ::client::state::State) {
        // for now, send it to the first program
        if let Some(server) = client_state.servers.first() {
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
