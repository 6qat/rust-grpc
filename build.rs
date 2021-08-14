#[allow(unused_imports)]
use std::env;
use std::path::{PathBuf, Path};

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_health_v1_descriptor_set_path: PathBuf =
        PathBuf::from("out/").join("grpc_health_v1.bin");

    let out_dir = Path::new("out/");
    tonic_build::configure()
        .out_dir(out_dir)
        .file_descriptor_set_path(grpc_health_v1_descriptor_set_path)
        .build_server(true)
        .build_client(true)
        .format(true)
        .compile(&["proto/dtc.proto", "proto/helloworld.proto"], &["proto/"])?;

    Ok(())
}
