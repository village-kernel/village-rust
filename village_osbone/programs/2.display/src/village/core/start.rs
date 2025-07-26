//###########################################################################
// start.c
// Low level file that manages app entry
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::ffi::c_void;

// extern set kernel
unsafe extern "Rust" { unsafe fn set_kernel(village: *const c_void); }

// extern main
unsafe extern "Rust" { unsafe fn main(argv: &[&str]); }

// dynamic header
unsafe extern "Rust" { unsafe fn _DYNAMIC(_: *const c_void, _: &[&str]); }

// entry section
#[used]
#[unsafe(link_section = ".entry")]
pub static G_PFN_VECTORS: [unsafe extern "Rust" fn(*const c_void, &[&str]); 2] = [
    _DYNAMIC,
    _start,
];

// fill bss zero
#[unsafe(no_mangle)]
pub extern "C" fn __fill_bss_zero() {
    unsafe extern "C" {
        unsafe static mut _sbss: u8;
        unsafe static mut _ebss: u8;
    }

    unsafe {
        // Zero fill the bss segment
        let mut dst = &raw mut _sbss as *mut u8;
        while dst < &raw mut _ebss as *mut u8 {
            *dst = 0;
            dst = dst.add(1);
        }
    }
}

// init array
#[unsafe(no_mangle)]
pub extern "C" fn __init_array() {
    unsafe extern "C" {
        unsafe static __init_array_start: [Option<unsafe extern "C" fn()>; 0];
        unsafe static __init_array_end: [Option<unsafe extern "C" fn()>; 0];
    }

    unsafe {
        let start = &__init_array_start as *const _ as *const unsafe extern "C" fn();
        let end = &__init_array_end as *const _ as *const unsafe extern "C" fn();
        let count = (end as usize - start as usize) / core::mem::size_of::<unsafe extern "C" fn()>();

        for i in 0..count {
            let func = start.add(i);
            (*func)();
        }
    }
}

// fini array
#[unsafe(no_mangle)]
pub extern "C" fn __fini_array() {
    unsafe extern "C" {
        unsafe static __fini_array_start: [Option<unsafe extern "C" fn()>; 0];
        unsafe static __fini_array_end: [Option<unsafe extern "C" fn()>; 0];
    }

    unsafe {
        let start = &__fini_array_start as *const _ as *const unsafe extern "C" fn();
        let end = &__fini_array_end as *const _ as *const unsafe extern "C" fn();
        let count = (end as usize - start as usize) / core::mem::size_of::<unsafe extern "C" fn()>();

        for i in 0..count {
            let func = start.add(i);
            (*func)();
        }
    }
}

// _start
#[unsafe(no_mangle)]
pub unsafe extern "Rust" fn _start(village: *const c_void, argv: &[&str]) {
    __fill_bss_zero();

    unsafe { set_kernel(village) };

    __init_array();

    unsafe { main(argv) };

    __fini_array();
}
