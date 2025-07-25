//###########################################################################
// vk_ps2_controller.rs
// The specific implementation of functions related to ps2 controller
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::vendor::ia32legacy::core::i686::*;

// Struct PS2Controller
pub struct PS2Controller;

// Impl PS2Controller
impl PS2Controller {
    // Write cmd
    pub fn write_cmd(&mut self, cmd: u8) {
        while (port_byte_in(PS2_READ_STATUS) & PS2_STATUS_INPUT_BUFFER_MSK) != 0 {}
        port_byte_out(PS2_WRITE_COMMAND, cmd);
    }

    // Write data
    pub fn write_dat(&mut self, dat: u8) {
        while (port_byte_in(PS2_READ_STATUS) & PS2_STATUS_INPUT_BUFFER_MSK) != 0 {}
        port_byte_out(PS2_WRITE_DATA, dat);
    }

    // Read data
    pub fn read_dat(&mut self) -> u8 {
        while (port_byte_in(PS2_READ_STATUS) & PS2_STATUS_OUTPUT_BUFFER_MSK) == 0 {}
        port_byte_in(PS2_WRITE_DATA)
    }
}
