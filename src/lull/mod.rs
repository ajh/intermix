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
        //let op = pry!(pry!(params.get()).get_op());
        //results.get().set_func(
            //calculator::function::ToClient::new(OperatorImpl {op : op}).from_server::<::capnp_rpc::Server>());
        results.get().set_program(&self.program.command);
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
