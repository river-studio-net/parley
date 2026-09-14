use std::error::Error;
use std::{env, path::PathBuf};


fn main() -> Result<(), Box<dyn Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_prost_build::configure()
        .file_descriptor_set_path(out_path.join("parley_descriptor.bin"))
        .compile_protos(&["../../../models/parley.proto","../../../models/message_service.proto"], 
            &["models", "message_service"])?;
    
    Ok(())
}
