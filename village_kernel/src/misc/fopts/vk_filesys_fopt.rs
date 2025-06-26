//###########################################################################
// vk_file_opt.rs
// The specific implementation of functions related to file opt
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::string::String;
use crate::village::kernel;

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
    // Moving
    pub fn moving(&mut self, source: &str, target: &str) -> bool {
        let _ = source;
        let _ = target;
        false
    }

    // Copy
    pub fn copy(&mut self, source: &str, target: &str) -> bool {
        let _ = source;
        let _ = target;
        false
    }

    // Remove
    pub fn remove(&mut self, source: &str) -> bool {
        if let Some(volume) = kernel().filesys().get_volume(source) {
            return volume.remove(source);
        }
        false
    }

    // Get_name
    pub fn get_name(&mut self) -> &str {
        &self.name
    }
}
