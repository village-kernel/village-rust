//###########################################################################
// vk_device.rs
// The specific implementation of functions related to device
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Device;

// struct concrete device
pub struct ConcreteDevice;

// impl concrete device
impl ConcreteDevice {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete device
impl ConcreteDevice {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Device setup completed!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl deivce for concrete device
impl Device for ConcreteDevice {
    // register block device
    fn register_block_device(&mut self) {

    }

    // unregister block device
    fn unregister_block_device(&mut self) {

    }
}
