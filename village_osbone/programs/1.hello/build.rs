use std::env;
use std::path::PathBuf;

// Map file
fn map_file() {
    // Get target dir
    let out_dir = env::var("OUT_DIR").unwrap();
    let binding = PathBuf::from(&out_dir);
    let target_dir = binding// out/
        .parent().unwrap()           // <crate>-<hash>/
        .parent().unwrap()           // build/
        .parent().unwrap();               // profile/

    // Get packge name
    let package_name = env::var("CARGO_PKG_NAME").unwrap();

    // Set map path
    let map_path = target_dir.join(format!("{}.map", package_name));

    // Ensure path exist
    if let Some(parent) = map_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    // Set the rustc link arg
    println!("cargo:rustc-link-arg=-Map={}", map_path.display());
}

// main
fn main() {
    map_file();
}
