use std::error::Error;
use std::{env, path::PathBuf};


fn main() -> Result<(), Box<dyn Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_prost_build::configure()
        .file_descriptor_set_path(out_path.join("parley_descriptor.bin"))
        .protoc_arg("--proto_path=/Users/riversong/git/river-studio-net/parley/models")
        .compile_protos(&["tables.proto","server_models.proto","services.proto"], 
            &["tables", "server_models", "services"])?;
    
    Ok(())
}
