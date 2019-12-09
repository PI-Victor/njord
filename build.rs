extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &["protobuf/registry.proto"],
        includes: &["protobuf/"],
        customize: Customize {
            ..Default::default()
        },
    })
    .unwrap();
}
