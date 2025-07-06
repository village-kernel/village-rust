//###########################################################################
// vk_dir_opt.rs
// The specific implementation of functions related to dir opt
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_filesys::{FileDir, FileMode, FileType};
use crate::village::kernel;
use alloc::string::{String, ToString};

// Struct DirFopt
pub struct DirFopt {
    path: String,
    fd: usize,
}

// Imp DirFopt
impl DirFopt {
    // New
    pub const fn new() -> Self {
        Self {
            path: String::new(),
            fd: 0,
        }
    }
}

// Impl DirFopt
impl DirFopt {
    // Is exist
    pub fn is_exist(&mut self, path: &str) -> bool {
        if let Some(volume) = kernel().filesys().get_volume(path) {
            return volume.is_exist(path, FileType::Directory);
        }
        false
    }

    // Open
    pub fn open(&mut self, path: &str, mode: FileMode) -> bool {
        self.path = path.to_string();
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            self.fd = volume.opendir(path, mode);
            return self.fd != 0;
        }
        false
    }

    // Read
    pub fn read(&mut self, dirs: &mut [FileDir], size: usize) -> usize {
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            return volume.readdir(self.fd, dirs, size, 0);
        }
        0
    }

    // Size
    pub fn size(&mut self) -> usize {
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            return volume.sizedir(self.fd);
        }
        0
    }

    // Close
    pub fn close(&mut self) {
        if let Some(volume) = kernel().filesys().get_volume(&self.path) {
            volume.closedir(self.fd);
        }
    }

    // Get name
    pub fn get_name(&mut self) -> &str {
        &self.path
    }
}
