//###########################################################################
// vk_filesystem.rs
// The specific implementation of functions related to file system
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::FileSystem;

// struct concrete file system
pub struct ConcreteFileSystem;

// impl concrete file system
impl ConcreteFileSystem {  
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete file system
impl ConcreteFileSystem {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("File system setup completed!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl file system for concrete file system
impl FileSystem for ConcreteFileSystem {
    // mount hard drive
    fn mount_hard_drive(&mut self, disk: &str) -> bool {
        let _ = disk;
        false
    }

    // unmount hard drive
    fn unmount_hard_drive(&mut self, disk: &str) -> bool {
        let _ = disk;
        false
    }

    // register fs
    fn register_fs(&mut self, name: &str) {
        let _ = name;
    }

    // unregister fs
    fn unregister_fs(&mut self, name: &str) {
        let _ = name;
    }

    // get volume
    fn get_volume(&mut self, name: &str) {
        let _ = name;
    }
}
