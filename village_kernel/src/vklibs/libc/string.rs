//###########################################################################
// string.rs
// The specific implementation of functions related to string
//
// $Copyright: Copyright (C) village
//###########################################################################

// memcpy
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe { *dst.add(i) = *src.add(i) };
    }
    dst
}

// memmove
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if dst < src as *mut u8 {
        for i in 0..n {
            unsafe { *dst.add(i) = *src.add(i) };
        }
    } else {
        for i in (0..n).rev() {
            unsafe { *dst.add(i) = *src.add(i) };
        }
    }
    dst
}

// memset
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(b: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe { *b.add(i) = c as u8 };
    }
    b
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

// strcpy
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcpy(dst: *mut u8, src: *const u8) -> *mut u8 {
    let mut i = 0;
    loop {
        let c = unsafe { *src.add(i) };
        unsafe { *dst.add(i) = c };
        if c == 0 { break; }
        i += 1;
    }
    dst
}

// strncpy
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncpy(dst: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    for i in 0..len {
        let c = unsafe { *src.add(i) };
        unsafe { *dst.add(i) = c };
        if c == 0 { 
            for j in (i+1)..len {
                unsafe { *dst.add(j) = 0 };
            }
            break; 
        }
    }
    dst
}

// strcat
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcat(s1: *mut u8, s2: *const u8) -> *mut u8 {
    let mut i = 0;
    while unsafe { *s1.add(i) } != 0 { i += 1; }

    let mut j = 0;
    loop {
        let c = unsafe { *s2.add(j) };
        unsafe { *s1.add(i) = c };
        if c == 0 { break; }
        i += 1;
        j += 1;
    }
    s1
}

// strncat
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncat(s1: *mut u8, s2: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while unsafe { *s1.add(i) } != 0 { i += 1; }
    
    let mut j = 0;
    while j < n {
        let c = unsafe { *s2.add(j) };
        unsafe { *s1.add(i) = c };
        if c == 0 { break; }
        i += 1;
        j += 1;
    }
    
    unsafe { *s1.add(i) = 0 };
    s1
}

// strcmp
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
        if a == 0 { break; }
        i += 1;
    }
    0
}

// strncmp
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
        if a == 0 { break; }
    }
    0
}

// strlen
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    while unsafe { *s.add(len) } != 0 { len += 1; }
    len
}

// strnlen
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strnlen(s: *const u8, maxlen: usize) -> usize {
    let mut len = 0;
    while len < maxlen && unsafe { *s.add(len) } != 0 { len += 1; }
    len
}

// strchr
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strchr(s: *const u8, c: i32) -> *mut u8 {
    let c = c as u8;
    let mut i = 0;
    loop {
        let current = unsafe { *s.add(i) };
        if current == c { return unsafe { s.add(i) } as *mut u8; }
        if current == 0 { return core::ptr::null_mut(); }
        i += 1;
    }
}

// strrchr
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strrchr(s: *const u8, c: i32) -> *mut u8 {
    let c = c as u8;
    let mut last = core::ptr::null_mut();
    let mut i = 0;
    loop {
        let current = unsafe { *s.add(i) };
        if current == c { last = unsafe { s.add(i) } as *mut u8; }
        if current == 0 { break; }
        i += 1;
    }
    last
}    
