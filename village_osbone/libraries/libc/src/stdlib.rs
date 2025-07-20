//###########################################################################
// stdlib.rs
// The specific implementation of functions related to stdlib
//
// $Copyright: Copyright (C) village
//###########################################################################

// extern kalloc and kfree
unsafe extern "C" {
    unsafe fn kalloc(size: u32) -> u32;
    unsafe fn kfree(ptr: u32, size: u32);
}

// malloc
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    unsafe { kalloc(size as u32) as *mut u8 }
}

// free
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    unsafe { kfree(ptr as u32, 0); }
}
