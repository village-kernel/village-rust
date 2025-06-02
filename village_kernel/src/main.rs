//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use village_kernel::traits::vk_kernel::{init_kernel, kernel};
use village_kernel::kernel::vk_village;

/// init
#[unsafe(no_mangle)]
pub fn init() {
    init_kernel(&vk_village::KERNEL_INSTANCE);
}

/// main
#[unsafe(no_mangle)]
pub fn main() -> ! {
    let kernel = kernel();
    kernel.setup();
    kernel.start();
    kernel.exit();
    loop {}
}

/// panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
