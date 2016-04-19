use capnp::Error;
use capnp::primitive_list;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use ::schema_capnp;
use gj::{EventLoop, Promise, TaskReaper, TaskSet};
use gj::io::tcp;
use super::*;

#[derive(Default, Debug)]
pub struct Lull {
    programs: Vec<Program>
}

impl schema_capnp::lull::Server for Lull {
    fn get_programs(&mut self,
                    params: schema_capnp::lull::GetProgramsParams,
                    mut results: schema_capnp::lull::GetProgramsResults)
                    -> Promise<(), Error>
    {
        let mut programs = results.get().init_programs(self.programs.len() as u32);

        for (i, p) in self.programs.iter().enumerate() {
            programs.borrow().get(i as u32).set_id(&p.id());
            programs.borrow().get(i as u32).set_command(&p.command());
            programs.borrow().get(i as u32).set_pid(p.pid());
        }

        Promise::ok(())
    }
}

impl Lull {
    pub fn new() -> Lull {
        Default::default()
    }

    pub fn programs(&self) -> &Vec<Program> {
        &self.programs
    }

    pub fn programs_mut(&mut self) -> &mut Vec<Program> {
        &mut self.programs
    }

    pub fn start(&mut self) {
        info!("server starting");
    }
}
