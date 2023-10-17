use std::{error::Error, path::Path};

static PROTO_INPUT_DIRECTORY: &str = "proto";
static PROTO_INPUT_FILE: &str = "mpc.proto";

fn main() {
    compile_protofiles(PROTO_INPUT_DIRECTORY, PROTO_INPUT_FILE)
        .expect("Coudln't compile protofiles");
    tauri_build::build()
}

fn compile_protofiles(
    proto_input_directory: &str,
    proto_input_file: &str,
) -> Result<(), Box<dyn Error>> {
    let proto_input_filepath = Path::new(proto_input_directory).join(proto_input_file);

    tonic_build::configure()
        .build_server(false)
        .compile(&[proto_input_filepath], &[proto_input_directory])?;
    Ok(())
}
