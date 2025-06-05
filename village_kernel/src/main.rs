//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use village_kernel::kernel::vk_village::Village;
use village_kernel::traits::vk_kernel::{init_kernel, kernel};

// Static village kernel instance
pub static VILLAGE: Village = Village::new();

// Init kernel
#[unsafe(no_mangle)]
pub fn __init_kernel() {
    init_kernel(&VILLAGE);
}

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
