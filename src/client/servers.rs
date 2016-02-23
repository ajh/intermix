use std::sync::mpsc::*;
use std::fmt;
use std::slice;

#[derive(Default, Clone, Debug)]
pub struct Servers {
    servers: Vec<Server>,
}

impl Servers {
    pub fn iter(&self) -> slice::Iter<Server> {
        self.servers.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<Server> {
        self.servers.iter_mut()
    }

    pub fn first(&self) -> Option<&Server> {
        self.servers.first()
    }

    pub fn add_server(&mut self, server: Server) {
        if !self.servers.iter().any(|w| w.id == server.id) {
            self.servers.push(server)
        }
    }

    pub fn remove_server(&mut self, id: &str) {
        if let Some(i) = self.servers.iter().position(|w| w.id == id) {
            self.servers.remove(i);
        }
    }

    pub fn add_program(&mut self, server_id: &str, program: Program) {
        let server = self.servers.iter_mut().find(|w| w.id == server_id);
        if server.is_none() {
            warn!("couldnt add program {:?} to unknown server {:?}",
                  program.id,
                  server_id);
            return;
        }
        let mut server = server.unwrap();

        if !server.programs.iter().any(|p| p.id == program.id) {
            server.programs.push(program)
        }
    }
}

/// A connection to an intermix server
#[derive(Clone)]
pub struct Server {
    pub id: String,
    pub programs: Vec<Program>,

    /// replace with with cap'n proto or whatever
    pub tx: Sender<::server::ServerMsg>,
}

impl fmt::Debug for Server {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Server")
           .field("id", &self.id)
           .field("programs", &self.programs)
           .finish()
    }
}

/// A program running on the server
#[derive(Default, Clone, Debug)]
pub struct Program {
    pub id: String,
    /// Whether the client is interested in msgs about this program. If its not visible, the answer
    /// is probably no.
    pub is_subscribed: bool,
}
