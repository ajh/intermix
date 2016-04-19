use libintermix::lull::*;
use libintermix::schema_capnp::lull;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use gj::{EventLoop, Promise, TaskReaper, TaskSet};
use gj::io::unix;
use std::thread;
use capnp;
use std::error::Error;
use regex;

fn rpc_top_level<F>(lull: Lull, main: F)
    where F: FnOnce(&::gj::WaitScope, lull::Client) -> Result<(), capnp::Error>,
          F: Send + 'static
{
    EventLoop::top_level(move |wait_scope| {
        let (join_handle, stream) = try!(unix::spawn(|stream, wait_scope| {
            let (reader, writer) = stream.split();
            let mut network =
                Box::new(twoparty::VatNetwork::new(reader, writer,
                                                   rpc_twoparty_capnp::Side::Server,
                                                   Default::default()));
            let disconnect_promise = network.on_disconnect();
            let bootstrap =
                lull::ToClient::new(lull).from_server::<::capnp_rpc::Server>();

            let _rpc_system = RpcSystem::new(network, Some(bootstrap.client));
            try!(disconnect_promise.wait(wait_scope));
            Ok(())
        }));

        let (reader, writer) = stream.split();

        let network =
            Box::new(twoparty::VatNetwork::new(reader, writer,
                                               rpc_twoparty_capnp::Side::Client,
                                               Default::default()));

        let mut rpc_system = RpcSystem::new(network, None);
        let client: lull::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        try!(main(wait_scope, client));
        drop(rpc_system);
        join_handle.join().expect("thread exited unsuccessfully");
        Ok(())
    }).expect("top level error");
}

#[test]
fn get_programs_returns_empty_list_when_none_exist() {
    rpc_top_level(Lull::new(), |wait_scope, client| {
        let response = try!(client.get_programs_request().send().promise.wait(wait_scope));
        let programs = try!(try!(response.get()).get_programs());
        assert_eq!(programs.len(), 0);
        Ok(())
    });
}

#[test]
fn get_programs_returns_program() {
    let mut lull = Lull::new();
    lull.programs_mut().push(Program::new("foobar", "foo -bar -baz > /dev/null", Some(42)));

    rpc_top_level(lull, |wait_scope, client| {
        let response = try!(client.get_programs_request().send().promise.wait(wait_scope));
        let programs = try!(try!(response.get()).get_programs());
        assert_eq!(programs.len(), 1);
        assert_eq!(programs.get(0).get_id().unwrap(), "foobar");
        assert_eq!(programs.get(0).get_command().unwrap(), "foo -bar -baz > /dev/null");
        assert_eq!(programs.get(0).get_pid(), 42);

        Ok(())
    });
}

#[test]
fn create_program_creates_one() {
    rpc_top_level(Lull::new(), |wait_scope, client| {
        let mut request = client.create_program_request();
        {
            let mut program = request.get().get_program().unwrap();
            program.set_id("abc123");
            program.set_command("echo hi");
        }
        let response = try!(request.send().promise.wait(wait_scope));
        let program = try!(try!(response.get()).get_program());
        assert_eq!(program.get_id().unwrap(), "abc123");
        assert_eq!(program.get_command().unwrap(), "echo hi");

        //assert_eq!(program.get_pid(), 42);

        Ok(())
    });
}

#[test]
fn create_program_without_id_fails() {
    rpc_top_level(Lull::new(), |wait_scope, client| {
        let mut request = client.create_program_request();
        {
            let mut program = request.get().get_program().unwrap();
            program.set_command("echo hi");
        }
        let response = request.send().promise.wait(wait_scope);
        let error = response.err().unwrap();
        assert!(regex::is_match("id is missing", error.description()).unwrap());
        Ok(())
    });
}

#[test]
fn create_program_without_command_fails() {
    rpc_top_level(Lull::new(), |wait_scope, client| {
        let mut request = client.create_program_request();
        {
            let mut program = request.get().get_program().unwrap();
            program.set_id("abc123");
        }
        let response = request.send().promise.wait(wait_scope);
        let error = response.err().unwrap();
        assert!(regex::is_match("command is missing", error.description()).unwrap());
        Ok(())
    });
}
