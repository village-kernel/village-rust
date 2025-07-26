//###########################################################################
// panic.rs
// The specific implementation of functions related to panic
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::panic::PanicInfo;
use crate::debug_error;

// Panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // print panic message
    debug_error!("{}", info.message());

    // print panic location
    if let Some(location) = info.location() {
        debug_error!(
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    } else {
        debug_error!("panic occurred but can't get location information...");
    }

    loop {}
}
