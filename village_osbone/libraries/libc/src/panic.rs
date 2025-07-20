//###########################################################################
// panic.rs
// The specific implementation of functions related to panic
//
// $Copyright: Copyright (C) village
//###########################################################################

// Import PanicInfo
use core::panic::PanicInfo;

// kpanic
unsafe extern "Rust" {
    unsafe fn kpanic(info: &PanicInfo)  -> !;
}

// panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe { kpanic(info); }
}
