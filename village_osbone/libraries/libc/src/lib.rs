//###########################################################################
// lib.rs
// The specific implementation of functions related to lib
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

// Import string
pub mod string;

// Import PanicInfo
use core::panic::PanicInfo;

// abort
unsafe extern "C" {
    unsafe fn abort() -> !;
}

// Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { abort() }
}
