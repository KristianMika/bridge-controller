extern crate bindgen;

use std::env::{self};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Iheaders")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join("bindings.rs");
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");

    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/mpc.proto"], &["proto/"])?;

    Ok(())
}
