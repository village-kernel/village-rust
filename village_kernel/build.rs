use std::env;
use std::process::Command;
use time::{OffsetDateTime, PrimitiveDateTime};
use time::macros::format_description;

fn main() {
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
        .output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    };

    // Set the env value
    println!("cargo:rustc-env=BUILD_DATE={}", date);
    println!("cargo:rustc-env=BUILD_TIME={}", time);
    println!("cargo:rustc-env=BUILD_VER=v{}", version);
    println!("cargo:rustc-env=GIT_COMMIT={}", commit_hash);
    println!("cargo:rustc-env=BUILD_YEAR={}", year);
}
