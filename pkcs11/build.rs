extern crate bindgen;

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

static PROTO_INPUT_DIRECTORY: &str = "proto";
static PROTO_INPUT_FILE: &str = "mpc.proto";
static PKCS_11_SPEC_VERSION: &str = "v3.0";
static PKCS_11_HEADERS_DIRECTORY: &str = "PKCS11-SPECS";

fn main() -> Result<(), Box<dyn Error>> {
    generate_bindings();

    compile_protofiles(PROTO_INPUT_DIRECTORY, PROTO_INPUT_FILE)
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

fn generate_bindings() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    let header_location = PathBuf::from(PKCS_11_HEADERS_DIRECTORY)
        .join(PKCS_11_SPEC_VERSION)
        .join("headers");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", header_location.to_str().unwrap()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set by cargo"));
    let out_file = out_dir.join("bindings.rs");
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
