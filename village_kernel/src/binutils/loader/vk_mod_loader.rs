//###########################################################################
// vk_mod_loader.rs
// The specific implementation of functions related to mod loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::string::{String, ToString};

// Struct ModLoader
pub struct ModLoader {
    filename: String,
}

// Impl ModLoader
impl ModLoader {
    // New
    pub const fn new() -> Self {
        Self {
            filename: String::new(),
        }
    }

    // Load
    pub fn load(&mut self, filename: &str) -> bool {
        self.filename = filename.to_string();
        false
    }

    // Fill bss zero
    pub fn fill_bss_zero(&mut self) {}

    // Init array
    pub fn init_array(&mut self) {}

    // Fini array
    pub fn fini_array(&mut self) {}

    // Exit
    pub fn exit(&mut self) -> bool {
        false
    }

    // Get filename
    pub fn get_filename(&mut self) -> &str {
        &self.filename
    }
}
