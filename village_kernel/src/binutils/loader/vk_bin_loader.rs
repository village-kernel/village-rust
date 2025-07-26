//###########################################################################
// vk_bin_loader.rs
// The specific implementation of functions related to bin loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::fopts::vk_file_fopt::FileFopt;
use crate::traits::vk_builder::ProgLoader;
use crate::traits::vk_filesys::FileMode;
use crate::debug_error;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// Struct BinLoader
pub struct BinLoader {
    filename: String,
}

// Impl BinLoader
impl BinLoader {
    // New
    pub const fn new() -> Self {
        Self {
            filename: String::new(),
        }
    }

    // Load bin
    fn load_bin(&mut self, data: &mut Vec<u8>) -> bool {
        let mut file = FileFopt::new();
        let mut result = false;

        // Read bin file
        if file.open(&self.filename, FileMode::READ) {
            let size = file.size();
            data.resize(size, 0);
            result = file.read(data, size, 0) == size;
            file.close();
        }

        // Return false when read bin file failed
        if !result {
            debug_error!("{} no such file!", self.filename);
            return false;
        }

        true
    }
}

// Impl ProgLoader for BinLoader
impl ProgLoader for BinLoader {
    // Init
    fn init(&mut self, filename: &str, data: &mut Vec<u8>) -> bool {
        // Save filename in local
        self.filename = filename.to_string();

        // Load and mapping
        if !self.load_bin(data) {
            return false;
        }

        true
    }

    // Exit
    fn exit(&mut self) -> bool {
        true
    }
}
