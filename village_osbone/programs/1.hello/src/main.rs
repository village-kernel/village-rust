//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

// import alloc
extern crate alloc;

// import village
pub mod village;
pub use village::traits as traits;
pub use village::misc as misc;

// Main
#[unsafe(no_mangle)]
pub fn main(argv: &[&str]) {
    println!("hello village demo");
    for arg in argv {
        println!(arg);
    }
}
