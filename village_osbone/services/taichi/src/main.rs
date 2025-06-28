//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]

mod crt0;

// print
fn print(message: &str) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in message.as_bytes().iter().enumerate() {
        unsafe {
            *vga_buffer.add(i * 2) = byte;
            *vga_buffer.add(i * 2 + 1) = 0x0f;
        }
    }
}

// Main
#[no_mangle]
pub fn main() {
    print("hello village application");
}
