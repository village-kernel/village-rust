//###########################################################################
// stdlib.rs
// The specific implementation of functions related to stdlib
//
// $Copyright: Copyright (C) village
//###########################################################################

// memcpy
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe { *dest.add(i) = *src.add(i) };
    }
    dest
}

// memmove
#[unsafe(no_mangle)]
pub unsafe  extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const _ && unsafe { src.add(n) } > dest as *const _ {
        for i in (0..n).rev() {
            unsafe {
                *dest.add(i) = *src.add(i);
            }
        }
    } else {
        for i in 0..n {
            unsafe {
                *dest.add(i) = *src.add(i);
            }
        }
    }
    dest
}

// memset
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let c = c as u8;
    for i in 0..n {
        unsafe { *dest.add(i) = c; }
    }
    dest
}

// memcmp
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
    }
    0
}
