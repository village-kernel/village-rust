//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

pub mod crt0;
pub mod village;
pub mod vk_kernel;
use core::panic::PanicInfo;
use crate::village::kernel;
pub use c::string::{memcmp, memset};

// Main
#[unsafe(no_mangle)]
pub fn main(argv: &[&str]) {
    kernel().debug().info("hello village application");
    for arg in argv {
        kernel().debug().info(arg);
    }
}

// panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
