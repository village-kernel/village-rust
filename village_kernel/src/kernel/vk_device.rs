//###########################################################################
// vk_device.rs
// The specific implementation of functions related to device
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Device;

/// struct concrete device
pub struct ConcreteDevice;

/// impl concrete device
impl ConcreteDevice {
    /// setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Device setup done!");
    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl deivce for concrete device
impl Device for ConcreteDevice {
    /// register block device
    fn register_block_device(&self) {

    }

    /// unregister block device
    fn unregister_block_device(&self) {

    }
}
