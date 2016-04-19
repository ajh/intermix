use libintermix::lull::*;
use libintermix::schema_capnp::lull;
use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use gj::{EventLoop, Promise, TaskReaper, TaskSet};
use gj::io::unix;
use std::thread;
use capnp::Error;

fn rpc_top_level<F>(main: F)
    where F: FnOnce(&::gj::WaitScope, lull::Client) -> Result<(), Error>,
          F: Send + 'static
{
    EventLoop::top_level(|wait_scope| {
        let (join_handle, stream) = try!(unix::spawn(|stream, wait_scope| {
            let lull = Lull::new();

            let (reader, writer) = stream.split();
            //let reader = ReadWrapper::new(reader,
            //                             ::std::fs::File::create("/Users/dwrensha/Desktop/client.dat").unwrap());
            //let writer = WriteWrapper::new(writer,
            //                               ::std::fs::File::create("/Users/dwrensha/Desktop/server.dat").unwrap());
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
fn get_programs_returns_program() {
    rpc_top_level(|wait_scope, client| {
        let response = try!(client.get_programs_request().send().promise.wait(wait_scope));
        let programs = try!(try!(response.get()).get_programs());
        assert_eq!(programs.len(), 1);
        assert_eq!(programs.get(0).get_id().unwrap(), "foobar");
        assert_eq!(programs.get(0).get_command().unwrap(), "foo -bar -baz > /dev/null");
        assert_eq!(programs.get(0).get_pid(), 42);

        Ok(())
    });
}
