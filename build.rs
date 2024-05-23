use std::path::PathBuf;
use std::process::Command;
use std::{env, io};

fn main() -> io::Result<()> {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("v4l2-bindings.rs"))
        .expect("Couldn't write bindings!");

    // Ensure Cargo re-runs this script if the C file changes
    println!("cargo:rerun-if-changed=resources/resolve.c");
    // compile our helper resolver.
    let target_exe = out_path.join("resolve_constants");
    Command::new("gcc")
        .args(["resources/resolve.c", "-o", target_exe.to_str().unwrap()])
        .status()
        .expect("Faild to build resolver.c with gcc");

    // Run the compiled c resolver  binary to create the `v4l2_constants.rs` file
    Command::new(target_exe)
        .status()
        .expect("Failed to run constant resolver");

    // Move the generated file to OUT_DIR
    let generated_file = PathBuf::from("v4l2_constants.rs");
    let dest_path = out_path.join("v4l2_constants.rs");

    std::fs::rename(generated_file, dest_path).expect("Failed to move v4l2_constants.rs file");

    Ok(())
}
