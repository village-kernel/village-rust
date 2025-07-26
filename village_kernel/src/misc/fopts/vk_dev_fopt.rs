//###########################################################################
// vk_dev_opt.rs
// The specific implementation of functions related to dev opt
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_driver::Command;
use alloc::string::{String, ToString};

// Struct DevFopt
pub struct DevFopt {
    name: String,
}

// Imp DevFopt
impl DevFopt {
    // New
    pub const fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
}

// Impl DevFopt
impl DevFopt {
    // Open
    pub fn open(&mut self, name: &str) -> bool {
        self.name = name.to_string();
        if let Some(driver) = kernel().device().get_driver(name) {
            return driver.open();
        }
        false
    }

    // Write
    pub fn write(&mut self, data: &[u8], size: usize, offset: usize) -> usize {
        if let Some(driver) = kernel().device().get_driver(&self.name) {
            return driver.write(data, size, offset);
        }
        0
    }

    // Read
    pub fn read(&mut self, data: &mut [u8], size: usize, offset: usize) -> usize {
        if let Some(driver) = kernel().device().get_driver(&self.name) {
            return driver.read(data, size, offset);
        }
        0
    }

    // Ioctrl
    pub fn ioctrl(&mut self, command: &mut Command) -> bool {
        if let Some(driver) = kernel().device().get_driver(&self.name) {
            return driver.ioctrl(command);
        }
        false
    }

    // Close
    pub fn close(&mut self) {
        if let Some(driver) = kernel().device().get_driver(&self.name) {
            return driver.close();
        }
    }

    // Get_name
    pub fn get_name(&mut self) -> &str {
        &self.name
    }
}
