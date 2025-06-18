//###########################################################################
// vk_bin_loader.rs
// The specific implementation of functions related to bin loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec::Vec;

// Struct BinLoader
pub struct BinLoader {

}

// Impl BinLoader
impl BinLoader {
    // New
    pub const fn new() -> Self {
        Self {

        }
    }

    // Load
    pub fn load(&mut self, filename: &str) -> bool {
        let _ = filename;
        false
    }

    // Execute
    pub fn execute(&mut self, argv: Vec<&str>) -> bool {
        let _ = argv;
        false
    }

    // Exit
    pub fn exit(&mut self) -> bool {
        false
    }
}
