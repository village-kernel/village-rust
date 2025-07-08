//###########################################################################
// vk_cmd_device.rs
// The specific implementation of functions related to cmd device
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::traits::vk_driver::DriverID;
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

// Struct cmd device
struct CmdDevice;

// Impl cmd for cmd device
impl Cmd for CmdDevice {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, _argv: Vec<&str>) {
        for driver_id in DriverID::iter() {
            for device in kernel().device().get_drivers().iter_mut() {
                if driver_id == device.get_id() {
                    console.println(&format!(
                        "name: {}, type: {}",
                        device.get_name(),
                        device.get_id().as_str()
                    ));
                }
            }
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd device: display device list");
    }
}

// Register cmd
register_cmd!(CmdDevice, device);
