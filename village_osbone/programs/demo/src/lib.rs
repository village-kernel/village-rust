//###########################################################################
// lib.rs
// The specific implementation of functions related to lib
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

// import village traits
pub use village::traits as traits;

// import village misc
pub use village::misc as misc;
