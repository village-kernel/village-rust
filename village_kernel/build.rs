use std::env;
use std::path::PathBuf;
use std::process::Command;
use time::macros::format_description;
use time::{OffsetDateTime, PrimitiveDateTime};

// Set build info env
fn build_info() {
    // Get the local time
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

    // Change to primitive date time
    let prim_datetime = PrimitiveDateTime::new(now.date(), now.time());

    // Get the build date
    let date_format = format_description!("[month repr:short] [day padding:space] [year]");
    let date = prim_datetime.format(&date_format).unwrap();

    // Get the build time
    let time_format = format_description!("[hour]:[minute]:[second]");
    let time = prim_datetime.format(&time_format).unwrap();

    // Get the year
    let year = now.year();

    // Get the build version
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());

    // Get the git commit sha
    let commit_hash = match Command::new("git")
        .current_dir("..")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    };

    // Set the rustc env
    println!("cargo:rustc-env=BUILD_YEAR={}", year);
    println!("cargo:rustc-env=BUILD_DATE={}", date);
    println!("cargo:rustc-env=BUILD_TIME={}", time);
    println!("cargo:rustc-env=BUILD_VER=v{}", version);
    println!("cargo:rustc-env=GIT_COMMIT={}", commit_hash);
}

// Set map file path
fn map_file() {
    // Get target dir
    let out_dir = env::var("OUT_DIR").unwrap();
    let binding = PathBuf::from(&out_dir);
    let target_dir = binding // out/
        .parent()
        .unwrap() // <crate>-<hash>/
        .parent()
        .unwrap() // build/
        .parent()
        .unwrap(); // profile/

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
    build_info();
    map_file();
}
