use capnp::Error;
use capnp::primitive_list;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use ::schema_capnp;
use gj::{EventLoop, Promise, TaskReaper, TaskSet};
use gj::io::tcp;
use super::Program;

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
            if let Some(pid) = p.pid() {
                programs.borrow().get(i as u32).set_pid(pid);
            }
        }

        Promise::ok(())
    }

    fn create_program(&mut self,
                      params: schema_capnp::lull::CreateProgramParams,
                      mut results: schema_capnp::lull::CreateProgramResults)
                      -> Promise<(), Error>
    {
        let program = {
            // Have to reckon with None params
            let p = params.get().unwrap().get_program().unwrap();
            Program::new(p.get_id().unwrap(), p.get_command().unwrap(), None)
        };

        if program.id().len() == 0 {
            return Promise::err(Error::failed("id is missing".to_string()));
        }
        if program.command().len() == 0 {
            return Promise::err(Error::failed("command is missing".to_string()));
        }

        // start program
        self.programs.push(program);

        let program = self.programs.last().unwrap();

        {
            let mut p = results.get().init_program();
            p.set_id(program.id());
            p.set_command(program.command());
            if let Some(pid) = program.pid() {
                p.set_pid(pid);
            }
        }

        Promise::ok(())
    }

    fn program_input(&mut self,
                     params: schema_capnp::lull::ProgramInputParams,
                     mut results: schema_capnp::lull::ProgramInputResults)
                      -> Promise<(), Error>
    {
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
