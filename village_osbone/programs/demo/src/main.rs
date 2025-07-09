//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

use demo::village::kernel;

// Main
#[unsafe(no_mangle)]
pub fn main(argv: &[&str]) {
    kernel().debug().info("hello village demo");
    for arg in argv {
        kernel().debug().info(arg);
    }
}
