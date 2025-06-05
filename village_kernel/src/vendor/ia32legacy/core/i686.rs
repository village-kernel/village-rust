//###########################################################################
// i686.c
// Low level file that manages i686
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::arch::asm;

// Read a byte from the specified port
#[inline(always)]
pub fn port_byte_in(port: u16) -> u8 {
    let mut val: u8;
    unsafe { asm!("inb al, dx", out("al") val, in("dx") port); }
    val
}

// Write a byte to the specified port
#[inline(always)]
pub fn port_byte_out(port: u16, val: u8) {
    unsafe { asm!("outb dx, al", in("dx") port, in("al") val); }
}

// Read a word from the specified port
#[inline(always)]
pub fn port_word_in(port: u16) -> u16 {
    let mut val: u16;
    unsafe { asm!("inw ax, dx", out("ax") val, in("dx") port); }
    val
}

// Write a word to the specified port
#[inline(always)]
pub fn port_word_out(port: u16, val: u16) {
    unsafe { asm!("outw dx, ax", in("dx") port, in("ax") val); }
}

// Read a long word from the specified port
#[inline(always)]
pub fn port_long_in(port: u16) -> u32 {
    let mut val: u32;
    unsafe { asm!("inl eax, dx", out("eax") val, in("dx") port); }
    val
}

// Write a long word to the specified port
#[inline(always)]
pub fn port_long_out(port: u16, val: u32) {
    unsafe { asm!("outl dx, eax", in("dx") port, in("eax") val); }
}
