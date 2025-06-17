//###########################################################################
// vk_dev_opt.rs
// The specific implementation of functions related to dev opt
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel;
use alloc::string::{String, ToString};

// Struct DevFopt
pub struct DevFopt {
    name: String,
}

// Imp DevFopt
impl DevFopt  {
    // New
    pub const fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
}

// Impl DevFopt
impl DevFopt  {
    // Open
    pub fn open(&mut self, name: &str) -> bool {
        self.name = name.to_string();
        if let Some(driver) = kernel().device().get_driver(name) {
            return driver.open();
        }
        false
    }

    // Write
    pub fn write(&mut self, data: &[u8], size: usize) -> usize {
        if let Some(driver) = kernel().device().get_driver(&self.name) {
            return driver.write(data, size);
        }
        0
    }
    
    // Read
    pub fn read(&mut self, data: &mut [u8], size: usize) -> usize {
        if let Some(driver) = kernel().device().get_driver(&self.name) {
            return driver.read(data, size);
        }
        0
    }

    // Ioctrl
    pub fn ioctrl(&mut self, cmd: u8, data: &[u8]) -> usize {
        if let Some(driver) = kernel().device().get_driver(&self.name) {
            return driver.ioctrl(cmd, data);
        }
        0
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
