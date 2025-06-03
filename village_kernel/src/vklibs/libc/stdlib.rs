//###########################################################################
// stdlib.rs
// The specific implementation of functions related to stdlib
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    kernel().memory().heap_alloc(size as u32) as *mut u8
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    kernel().memory().free(ptr as u32, 0);
}
