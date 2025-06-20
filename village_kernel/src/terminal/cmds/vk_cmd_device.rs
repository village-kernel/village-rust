//###########################################################################
// vk_cmd_device.rs
// The specific implementation of functions related to cmd device
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use crate::register_cmd;
use crate::traits::vk_driver::DriverID;
use crate::village::kernel;
use crate::traits::vk_command::{Cmd, CmdBase};

// Struct cmd device
struct CmdDevice {
    base: CmdBase
}

// Impl cmd device
impl CmdDevice {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd device
impl Cmd for CmdDevice {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            for driver_id in DriverID::iter() {
                for device in kernel().device().get_drivers().iter_mut() {
                    let info = device.info();
                    if driver_id == info.get_id() {
                        console.println(&format!("name: {}, type: {}", info.get_name(), info.get_id().as_str()));
                    }
                }
            }
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd device: display device list");
        }
    }
}

// Register cmd
register_cmd!(CmdDevice::new(), device);
