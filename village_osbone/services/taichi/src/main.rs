//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]
#![feature(linkage)]

// import alloc
extern crate alloc;

// import village
pub mod village;
pub use village::traits as traits;
pub use village::misc as misc;

// import taichi
mod taichi;
use crate::taichi::Taichi;

// Main
#[unsafe(no_mangle)]
pub fn main() {
    let mut taichi = Taichi;
    taichi.setup();
    taichi.execute();
    taichi.exit();
}
