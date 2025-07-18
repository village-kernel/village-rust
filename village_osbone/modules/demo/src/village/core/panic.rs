//###########################################################################
// panic.rs
// The specific implementation of functions related to panic
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::format;
use core::panic::PanicInfo;
use crate::village::kernel;

// Panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // print panic message
    kernel().debug().error(&format!("{}", info.message()));

    // print panic location
    if let Some(location) = info.location() {
        let msg = format!(
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
        kernel().debug().error(&msg);
    } else {
        kernel()
            .debug()
            .error("panic occurred but can't get location information...");
    }

    loop {}
}
