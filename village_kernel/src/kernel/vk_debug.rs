//###########################################################################
// vk_debug.rs
// The specific implementation of functions related to debug
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::format;
use crate::village::kernel;
use crate::traits::vk_kernel::{Debug, DebugLevel};
use crate::drivers::platdrv::serial::vk_pic32_uart::Pic32Uart;

// Static const
static BUF_SIZE: usize = 256;

// Struct concrete debug
pub struct ConcreteDebug {
    transceiver: Pic32Uart,
    debug_level: DebugLevel,
    is_ready: bool,
    tx_pos: usize,
    tx_buf: [u8; BUF_SIZE],
}

// Impl concrete debug
impl ConcreteDebug {
    pub const fn new() -> Self {
        Self { 
            transceiver: Pic32Uart::new(0),
            debug_level: DebugLevel::Lv2,
            is_ready: false,
            tx_pos: 0,
            tx_buf: [0; BUF_SIZE],
        }
    }
}

// Impl concrete debug
impl ConcreteDebug {
    // Setup
    pub fn setup(&mut self) {
        // Open transceiver
        self.transceiver.open();

        // Set ready flag
        self.is_ready = true;

        // Output debug info
        kernel().debug().info("Debug setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Close transceiver
        self.transceiver.close();

        // Clear ready flag
        self.is_ready = false;
    }
}

// Impl concrete debug
impl ConcreteDebug {
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
            let tx_pos = self.tx_pos;
            self.tx_pos += 1;
            self.tx_buf[tx_pos] = *byte;
        }

        // Sending msg
        self.sending();
    }

    // Sending
    fn sending(&mut self) {
        if self.is_ready && self.tx_pos > 0 {
            while self.transceiver.write(&self.tx_buf, self.tx_pos) != self.tx_pos {}
            self.tx_pos = 0;
        }
    }
}

// Impl debug for concrete debug
impl Debug for ConcreteDebug {
    // Log
    fn log(&mut self, log: &str) {
        self.write(&format!("Log: {} \r\n", log));
    }

    // Info
    fn info(&mut self, info: &str) {
        self.write(&format!("\x1b[36m[Info] {} \r\n\x1b[39m", info));
    }

    // Error
    fn error(&mut self, error: &str) {
        self.write(&format!("\x1b[31m[Error] {} \r\n\x1b[39m", error));
    }

    // Warn
    fn warn(&mut self, warn: &str) {
        self.write(&format!("\x1b[33m[Warning] {} \r\n\x1b[39m", warn));
    }

    // Output
    fn output(&mut self, level: DebugLevel, msg: &str) {
        if level >= self.debug_level {
            self.write(&format!("\x1b[36m[Warning] {} \r\n\x1b[39m", msg));
        }
    }

    // Set debug level
    fn set_debug_level(&mut self, level: DebugLevel){
        if level >= DebugLevel::Lv0 && level <= DebugLevel::Lv5 {
            self.debug_level = level;
        } else {
            self.error(&format!("The level {:?} out of range", level));
        }
    }
}
