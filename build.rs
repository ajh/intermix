extern crate capnpc;

fn main() {
    ::capnpc::compile("src/capnp", &["src/capnp/schema.capnp"]).unwrap();
}
