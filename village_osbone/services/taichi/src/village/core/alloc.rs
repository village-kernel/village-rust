//###########################################################################
// alloc.rs
// The specific implementation of functions related to alloc
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use core::alloc::{GlobalAlloc, Layout};

// Struct GlobalAllocator
struct GlobalAllocator;

// Set global allocator
#[global_allocator]
static ALLOCATOR: GlobalAllocator = GlobalAllocator;

// Impl global alloc for global allocator
unsafe impl GlobalAlloc for GlobalAllocator {
    // Alloc
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        kernel().memory().alloc(layout.size() as u32) as *mut u8
    }

    // Dealloc
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        kernel().memory().dealloc(ptr as u32, layout.size() as u32);
    }
}
