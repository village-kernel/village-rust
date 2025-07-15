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

    // Exit
    pub fn exit(&mut self) -> bool {
        false
    }

    // Get filename
    pub fn filename(&mut self) -> &str {
        &self.filename
    }
}
