use std::{env, fs, process::Command};

// Example custom build script.
fn main() {
    // https://github.com/rust-lang/cargo/issues/6412

    // let _child = Command::new("cargo")
    //     .arg("test")
    //     .spawn()
    //     .expect("Failed to generate bindings");
    // cargo test --package server --bin server -- tests --nocapture
    // --target-dir

    // Tell Cargo that if the given file changes, to rerun this build script.
    // println!("cargo:cargo test");
    // println!("Moving /bindings to /ui-astro");
    // // Use the `cc` crate to build a C file and statically link it.
    // let curr = env::current_dir();
    // println!("current dir: {curr:?}");
    // // println!("cargo:rerun-if-changes=src/bindings");
    // fs::rename("./bindings", "../ui-astro/src/server_types").unwrap();
    // println!("Complete!");
}
