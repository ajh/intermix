use capnp::Error;
use capnp::primitive_list;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use ::schema_capnp;
use gj::{EventLoop, Promise, TaskReaper, TaskSet};
use gj::io::tcp;

pub struct Program {
    id: String,
    command: String,
    pid: i32,
}

pub struct LullImpl {
    program: Program
}

impl schema_capnp::lull::Server for LullImpl {
    fn get_programs(&mut self,
                    params: schema_capnp::lull::GetProgramsParams,
                    mut results: schema_capnp::lull::GetProgramsResults)
                    -> Promise<(), Error>
    {
        let mut programs = results.get().init_programs(1);
        programs.borrow().get(0).set_id(&self.program.id);
        programs.borrow().get(0).set_command(&self.program.command);
        programs.borrow().get(0).set_pid(self.program.pid);
        Promise::ok(())
    }
}

impl LullImpl {
    pub fn new() -> LullImpl {
        LullImpl {
            program: Program {
                id: "foobar".to_string(),
                command: "foo -bar -baz > /dev/null".to_string(),
                pid: 42,
            }
        }
    }

    pub fn start(&mut self) {
        info!("server starting");
    }
}
