//###########################################################################
// vk_filesystem.rs
// The specific implementation of functions related to file system
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel::traits::vk_kernel::FileSystem;

/// struct concrete file system
pub struct ConcreteFileSystem;

/// impl concrete file system
impl ConcreteFileSystem {
    /// setup
    pub fn setup(&self) {

    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl file system for concrete file system
impl FileSystem for ConcreteFileSystem {
    /// mount hard drive
    fn mount_hard_drive(&self, disk: &str) -> bool {
        false
    }

    /// unmount hard drive
    fn unmount_hard_drive(&self, disk: &str) -> bool {
        false
    }

    /// register fs
    fn register_fs(&self, name: &str) {

    }

    /// unregister fs
    fn unregister_fs(&self, name: &str) {

    }

    /// get volume
    fn get_volume(&self, name: &str) {
        
    }
}
