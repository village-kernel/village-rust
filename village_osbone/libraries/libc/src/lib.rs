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

// panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { abort() }
}

// Export marco
#[macro_export]
macro_rules! export {
    ($fn:item) => {
        #[unsafe(link_section = ".exported_symbols")]
        #[unsafe(no_mangle)]
        $fn
    };
}

// Export impl marco
#[macro_export]
macro_rules! export_impl {
    ($ty:ty, $method:ident, $name:ident, $($args:ty),*) => {
        #[unsafe(link_section = ".exported_symbols")]
        #[unsafe(no_mangle)]
        pub extern "C" fn $name(ptr: *mut $ty, $($args),*) {
            unsafe {
                if let Some(obj) = ptr.as_mut() {
                    obj.$method($($args),*);
                }
            }
        }
    };
}
