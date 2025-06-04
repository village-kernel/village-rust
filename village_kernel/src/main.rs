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

// init kernel
#[no_mangle]
pub fn __init_kernel() {
    init_kernel(&vk_village::KERNEL_INSTANCE);
}

// main
#[no_mangle]
pub fn main() {
    kernel().setup();
    kernel().start();
    kernel().exit();
}

// panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
