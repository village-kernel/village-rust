//###########################################################################
// main.rs
// The specific implementation of functions related to main
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]
#![feature(linkage)]

// import alloc module
extern crate alloc;

// import village module
pub mod village;
pub use village::traits as traits;
pub use village::misc as misc;

// import module
pub mod module;
