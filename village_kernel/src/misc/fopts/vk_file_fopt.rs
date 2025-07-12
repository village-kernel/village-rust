//###########################################################################
// vk_file_opt.rs
// The specific implementation of functions related to file opt
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_filesys::{FileMode, FileType};
use crate::village::kernel;
use alloc::string::{String, ToString};

// Struct FileFopt
pub struct FileFopt {
    path: String,
    fd: usize,
}

// Imp FileFopt
impl FileFopt {
    // New
    pub const fn new() -> Self {
        Self {
            path: String::new(),
            fd: 0,
        }
    }
}

// Impl FileFopt
impl FileFopt {
    // Is exist
    pub fn exist(&mut self, path: &str) -> bool {
        if let Some(volume) = kernel().filesys().get_volume(path) {
            return volume.exist(path, FileType::File);
        }
        false
    }

    // Open
    pub fn open(&mut self, path: &str, mode: FileMode) -> bool {
        self.path = path.to_string();
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            self.fd = volume.open(path, mode);
            return self.fd != 0;
        }
        false
    }

    // Write
    pub fn write(&mut self, data: &[u8], size: usize, offset: usize) -> usize {
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            return volume.write(self.fd, data, size, offset);
        }
        0
    }

    // Read
    pub fn read(&mut self, data: &mut [u8], size: usize, offset: usize) -> usize {
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            return volume.read(self.fd, data, size, offset);
        }
        0
    }

    // Size
    pub fn size(&mut self) -> usize {
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            return volume.size(self.fd);
        }
        0
    }

    // Flush
    pub fn flush(&mut self) {
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            volume.flush(self.fd);
        }
    }

    // Close
    pub fn close(&mut self) {
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            volume.close(self.fd);
        }
    }

    // Get_name
    pub fn get_name(&mut self) -> &str {
        &self.path
    }
}
