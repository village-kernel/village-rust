//###########################################################################
// vk_hex_loader.rs
// The specific implementation of functions related to hex loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec::Vec;

// Struct HexLoader
pub struct HexLoader {

}

// Impl HexLoader
impl HexLoader {
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
