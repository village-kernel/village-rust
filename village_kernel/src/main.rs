//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

use village_kernel::village::kernel;

// Main
#[unsafe(no_mangle)]
pub fn main() {
    kernel().setup();
    kernel().start();
    kernel().exit();
}
