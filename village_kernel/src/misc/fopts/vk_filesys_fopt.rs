//###########################################################################
// vk_file_opt.rs
// The specific implementation of functions related to file opt
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::string::String;

// Struct FilesysFopt
pub struct FilesysFopt {
    name: String,
}

// Imp FilesysFopt
impl FilesysFopt  {
    // New
    pub const fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
}

// Impl FilesysFopt
impl FilesysFopt  {
    // Move
    pub fn mov(&mut self, source: &str, target: &str) {
        let _ = source;
        let _ = target;
    }

    pub fn copy(&mut self, source: &str, target: &str) {
        let _ = source;
        let _ = target;
    }

    pub fn remove(&mut self, source: &str) {
        let _ = source;
    }

    // Get_name
    pub fn get_name(&mut self) -> &str {
        &self.name
    }
}
