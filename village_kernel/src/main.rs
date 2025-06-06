//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use village_kernel::village::kernel;

// Main
#[unsafe(no_mangle)]
pub fn main() {
    kernel().setup();
    kernel().start();
    kernel().exit();
}

// Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
