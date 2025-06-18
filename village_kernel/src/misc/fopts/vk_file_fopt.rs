//###########################################################################
// vk_file_opt.rs
// The specific implementation of functions related to file opt
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::string::{String, ToString};

// Struct FileFopt
pub struct FileFopt {
    name: String,
}

// Imp FileFopt
impl FileFopt  {
    // New
    pub const fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
}

// Impl FileFopt
impl FileFopt  {
    // Open
    pub fn open(&mut self, name: &str) -> bool {
        self.name = name.to_string();
        false
    }

    // Write
    pub fn write(&mut self, data: &[u8], size: usize) -> usize {
        let _ = data;
        let _ = size;
        0
    }
    
    // Read
    pub fn read(&mut self, data: &mut [u8], size: usize) -> usize {
        let _ = data;
        let _ = size;
        0
    }

    // Close
    pub fn close(&mut self) {

    }

    // Get_name
    pub fn get_name(&mut self) -> &str {
        &self.name
    }
}
