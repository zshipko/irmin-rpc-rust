extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("irmin_api.capnp")
        .run()
        .unwrap();
}
