//###########################################################################
// vk_ps2_mouse.rs
// The specific implementation of functions related to ps2 mouse
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::drivers::chipdrv::ia32legacy::vk_ps2_controller::PS2Controller;
use crate::register_plat_driver;
use crate::traits::vk_callback::Callback;
use crate::traits::vk_driver::{Driver, PlatDevWrapper, PlatDriver};
use crate::traits::vk_event_codes::{EventCode, KeyStatus};
use crate::vendor::ia32legacy::core::i686::*;
use crate::village::kernel;
use alloc::boxed::Box;

// Struct PS2MouseConfig
#[derive(Clone)]
pub struct PS2MouseConfig {
    pub irq: isize,
}

// Impl PS2MouseConfig
impl PS2MouseConfig {
    pub const fn new() -> Self {
        Self { irq: 0 }
    }
}

// Struct MousePacket
struct MousePacket {
    bytes: [u8; 4],
    // Byte 0
    left_btn: bool,
    right_btn: bool,
    middle_btn: bool,
    x_sign_bit: bool,
    y_sign_bit: bool,
    x_over_flow: bool,
    y_over_flow: bool,
    // Byte 1
    x: u8,
    // Byte 2
    y: u8,
    // Byte 3
    z: u8,
}

// Impl MousePacket
impl MousePacket {
    // New
    pub const fn new() -> Self {
        Self {
            bytes: [0u8; 4],
            // Byte 0
            y_over_flow: false,
            x_over_flow: false,
            y_sign_bit: false,
            x_sign_bit: false,
            middle_btn: false,
            right_btn: false,
            left_btn: false,
            // Byte 1
            x: 0,
            // Byte 2
            y: 0,
            // Byte 3
            z: 0,
        }
    }

    // Decode
    pub fn decode(&mut self) {
        let indicator = self.bytes[0];
        self.y_over_flow = (indicator & 0x80) != 0;
        self.x_over_flow = (indicator & 0x40) != 0;
        self.y_sign_bit  = (indicator & 0x20) != 0;
        self.x_sign_bit  = (indicator & 0x10) != 0;
        self.middle_btn  = (indicator & 0x04) != 0;
        self.right_btn   = (indicator & 0x02) != 0;
        self.left_btn    = (indicator & 0x01) != 0;
        self.x = self.bytes[1];
        self.y = self.bytes[2];
        self.z = self.bytes[3];
    }
}

// Struct PS2Mouse
pub struct PS2Mouse {
    config: PS2MouseConfig,
    ps2: PS2Controller,
    work_id: i32,

    ack: u8,
    setting: u8,
    mouseid: u8,
    count: u8,
    packet: MousePacket,
    
    is_left_btn_pressed: bool,
    is_right_btn_pressed: bool,
    is_middle_btn_pressed: bool,
}

// Impl PS2Mouse
impl PS2Mouse {
    pub const fn new() -> Self {
        Self {
            config: PS2MouseConfig::new(),
            ps2: PS2Controller,
            work_id: 0,

            ack: 0,
            setting: 0,
            mouseid: 0,
            count: 0,
            packet: MousePacket::new(),
            
            is_left_btn_pressed: false,
            is_right_btn_pressed: false,
            is_middle_btn_pressed: false,
        }
    }

    // Set config
    fn set_config(&mut self, data: *mut ()) {
        if !data.is_null() {
            self.config = unsafe { (*(data as *mut PS2MouseConfig)).clone() }
        }
    }

    // Write data
    fn write_data(&mut self, data: u8) -> u8 {
        self.ps2.write_cmd(PS2_CMD_WR_CTL_SEC_INPUT_BUFF);
        self.ps2.write_dat(data);
        self.ps2.read_dat()
    }

    // Read data
    fn read_data(&mut self) -> u8 {
        self.ps2.read_dat()
    }

    // Config mouse
    fn config_mouse(&mut self) -> bool {
        // Disable irq
        kernel().system().disable_irq();

        // Read setting
        self.ps2.write_cmd(PS2_CMD_READ_BYTE_0);
        self.setting = self.ps2.read_dat();

        // Set irq12 and ps2 clk enable flag
        self.setting |= PS2_CTL_SECOND_PORT_INT_MSK;
        self.setting &= !PS2_CTL_SECOND_PORT_CLK_MSK;

        // Enable irq12
        self.ps2.write_cmd(PS2_CMD_WRITE_NEXT_BYTE_0);
        self.ps2.write_dat(self.setting);

        // Restore to defaults
        self.ps2.write_cmd(PS2_CMD_ENA_SEC_PS2_PORT);

        // Init/Detection Command Sequences
        self.write_data(PS2_MOUSE_CMD_SET_DEFAULTS);

        self.write_data(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
        self.write_data(200);

        self.write_data(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
        self.write_data(100);

        self.write_data(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
        self.write_data(80);

        self.write_data(PS2_MOUSE_CMD_GET_DEVICE_ID);
        self.mouseid = self.read_data();

        // Enable 4th and 5th mouse buttons
        if self.mouseid == 3 {
            self.write_data(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
            self.write_data(200);

            self.write_data(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
            self.write_data(200);

            self.write_data(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
            self.write_data(80);

            self.write_data(PS2_MOUSE_CMD_GET_DEVICE_ID);
            self.mouseid = self.read_data();
        }

        // Set sample rate
        self.write_data(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
        self.write_data(200);

        // Set resolution
        self.write_data(PS2_MOUSE_CMD_SET_RESOLUTION);
        self.write_data(3);

        // Set scaling 1:1
        self.write_data(PS2_MOUSE_CMD_SCALING_1_1);

        // Enable mouse
        self.ack = self.write_data(PS2_MOUSE_CMD_ENA_DATA_REPORTING);

        // Enable irq
        kernel().system().enable_irq();

        // Return
        self.ack == 0xfa
    }

    // Input handler
    fn input_handler(&mut self) {
        if (port_byte_in(PS2_READ_STATUS) & PS2_STATUS_OUTPUT_BUFFER_MSK) != 0 {
            self.packet.bytes[self.count as usize] = port_byte_in(PS2_READ_DATA);

            self.count += 1;

            if self.count >= self.mouseid {
                self.count = 0;
                self.packet.decode();
                kernel().workqueue().sched(self.work_id);
            }
        }
    }

    // Report handler
    fn report_handler(&mut self) {
        // Report left button
        if self.packet.left_btn && !self.is_left_btn_pressed {
            self.is_left_btn_pressed = true;
            kernel().event().report_key(EventCode::BTN_LEFT as isize, KeyStatus::KEY_PRESSED.into());
        } else if self.packet.left_btn && self.is_left_btn_pressed {
            self.is_left_btn_pressed = false;
            kernel().event().report_key(EventCode::BTN_LEFT as isize, KeyStatus::KEY_RELEASED.into());
        }

        // Report right button
        if self.packet.right_btn && !self.is_right_btn_pressed {
            self.is_right_btn_pressed = true;
            kernel().event().report_key(EventCode::BTN_RIGHT as isize, KeyStatus::KEY_PRESSED.into());
        } else if !self.packet.right_btn && self.is_right_btn_pressed {
            self.is_right_btn_pressed = false;
            kernel().event().report_key(EventCode::BTN_RIGHT as isize, KeyStatus::KEY_RELEASED.into());
        }

        // Report middle button
        if self.packet.middle_btn && !self.is_middle_btn_pressed {
            self.is_middle_btn_pressed = true;
            kernel().event().report_key(EventCode::BTN_MIDDLE as isize, KeyStatus::KEY_PRESSED.into());
        } else if !self.packet.middle_btn && self.is_middle_btn_pressed {
            self.is_middle_btn_pressed = false;
            kernel().event().report_key(EventCode::BTN_MIDDLE as isize, KeyStatus::KEY_RELEASED.into());
        }

        // Report axis x, y, z movement value
        let mut axis_x = self.packet.x as isize; if self.packet.x_sign_bit { axis_x -= 0x100; }
        let mut axis_y = self.packet.y as isize; if self.packet.y_sign_bit { axis_y -= 0x100; }
        let axis_z = self.packet.z as isize;
        kernel().event().report_axis(axis_x, axis_y, axis_z);
    }
}

// Impl Driver for PS2Mouse
impl Driver for PS2Mouse {
    // Open
    fn open(&mut self, data: *mut ()) -> bool {
        // Get config
        self.set_config(data);

        // Create work
        let report_cb = Callback::new(Self::report_handler as u32).with_instance(self);
        self.work_id = kernel().workqueue().create(report_cb, 0);

        // Create input handler
        let input_cb = Callback::new(Self::input_handler as u32).with_instance(self);
        kernel().interrupt().set_isr_cb(self.config.irq, input_cb);

        // Config
        self.config_mouse()
    }

    // Close
    fn close(&mut self) {
        // Delete input handler
        let input_cb = Callback::new(Self::input_handler as u32).with_instance(self);
        kernel().interrupt().del_isr_cb(self.config.irq, input_cb);

        // Delete workqueue job
        kernel().workqueue().delete(self.work_id);
    }
}

// Struct ps2 mouse drv
struct PS2MouseDrv;

// Impl plat driver for ps2 mouse driver
impl PlatDriver for PS2MouseDrv {
    // Probe
    fn probe(&mut self, device: &mut PlatDevWrapper) -> bool {
        device.plat().attach(Box::new(PS2Mouse::new()));
        true
    }

    // Remove
    fn remove(&mut self, device: &mut PlatDevWrapper) -> bool {
        device.plat().detach();
        true
    }
}

// Register plat driver
register_plat_driver!(PS2MouseDrv, ps2mouse, ps2_mouse_drv);
