//###########################################################################
// vk_debug.rs
// The specific implementation of functions related to debug
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::fopts::vk_dev_fopt::DevFopt;
use crate::traits::vk_kernel::{Debug, DebugLevel};
use crate::debug_info;
use crate::debug_error;
use core::panic::PanicInfo;

// Static const
static BUF_SIZE: usize = 256;

// Struct village debug
pub struct VillageDebug {
    transceiver: DevFopt,
    debug_level: DebugLevel,
    is_ready: bool,
    tx_pos: usize,
    tx_buf: [u8; BUF_SIZE],
}

// Impl village debug
impl VillageDebug {
    pub const fn new() -> Self {
        Self {
            transceiver: DevFopt::new(),
            debug_level: DebugLevel::Lv2,
            is_ready: false,
            tx_pos: 0,
            tx_buf: [0; BUF_SIZE],
        }
    }
}

// Impl village debug
impl VillageDebug {
    // Setup
    pub fn setup(&mut self) {
        // Open transceiver
        self.transceiver.open("serial0");

        // Set ready flag
        self.is_ready = true;

        // Output debug info
        debug_info!("Debug setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Close transceiver
        self.transceiver.close();

        // Clear ready flag
        self.is_ready = false;
    }
}

// Impl village debug
impl VillageDebug {
    // Write
    fn write(&mut self, data: &str) {
        // Calculate the string length
        let len = data.as_bytes().len();

        // When the device is not ready and the buffer is full,
        // the previous part of the data is discarded.
        if !self.is_ready && ((BUF_SIZE - self.tx_pos) < len) {
            // Calculate how much data needs to be discarded
            let delta = len - (BUF_SIZE - self.tx_pos);

            // Discard specified amount of data
            for i in 0..(BUF_SIZE - delta) {
                self.tx_buf[i] = self.tx_buf[i + delta];
            }

            // Update txBufPos
            self.tx_pos -= delta;
        }

        // Copy msg data into txBuffer
        for byte in data.as_bytes() {
            // The txBuffer is full, block here until the data is sent
            if self.tx_pos >= BUF_SIZE {
                self.sending();
            }

            // Copy data
            self.tx_buf[self.tx_pos] = *byte;
            self.tx_pos += 1;
        }

        // Sending msg
        self.sending();
    }

    // Sending
    fn sending(&mut self) {
        if self.is_ready && self.tx_pos > 0 {
            while self.transceiver.write(&self.tx_buf, self.tx_pos, 0) != self.tx_pos {}
            self.tx_pos = 0;
        }
    }
}

// Impl debug for village debug
impl Debug for VillageDebug {
    // Log
    fn log(&mut self, log: &str) {
        self.write("[log] ");
        self.write(log);
        self.write("\r\n");
    }

    // Info
    fn info(&mut self, info: &str) {
        self.write("\x1b[36m[info] ");
        self.write(info);
        self.write("\r\n\x1b[39m");
    }

    // Error
    fn error(&mut self, error: &str) {
        self.write("\x1b[31m[error] ");
        self.write(error);
        self.write("\r\n\x1b[39m");
    }

    // Warn
    fn warning(&mut self, warning: &str) {
        self.write("\x1b[33m[warning] ");
        self.write(warning);
        self.write("\r\n\x1b[39m");
    }

    // Println
    fn println(&mut self, msg: &str) {
        self.write(msg);
        self.write("\r\n");
    }

    // Output
    fn output(&mut self, level: DebugLevel, msg: &str) {
        if level >= self.debug_level {
           self.write("\x1b[34m[message] ");
            self.write(msg);
            self.write("\r\n\x1b[39m");
        }
    }

    // Set debug level
    fn set_debug_level(&mut self, level: DebugLevel) {
        if level >= DebugLevel::Lv0 && level <= DebugLevel::Lv5 {
            self.debug_level = level;
        } else {
            debug_error!("The level {} out of range", level.as_str());
        }
    }
}

// Panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // print panic message
    debug_error!("{}", info.message());

    // print panic location
    if let Some(location) = info.location() {
        debug_error!(
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    } else {
        debug_error!("panic occurred but can't get location information...");
    }

    loop {}
}
