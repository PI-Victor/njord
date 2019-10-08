extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos/internal",
        input: &[
            "protobuf/internal/node.proto",
            "protobuf/internal/registry.proto",
        ],
        includes: &["protobuf/"],
        customize: Customize {
            ..Default::default()
        },
    })
    .unwrap();

    // protoc_rust::run(protoc_rust::Args {
    //     out_dir: "assets/clients",
    //     input: &[],
    //     includes: &["protobuf"],
    //     customize: Customize {
    //         ..Default::default()
    //     },
    // })
    // .unwrap();
}
