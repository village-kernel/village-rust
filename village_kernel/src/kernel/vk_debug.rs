//###########################################################################
// vk_debug.rs
// The specific implementation of functions related to debug
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Debug;
use crate::drivers::platdrv::serial::vk_pic32_uart::Pic32Uart;

// struct concrete debug
pub struct ConcreteDebug {
    uart: Pic32Uart,
}

// impl concrete debug
impl ConcreteDebug {
    pub const fn new() -> Self {
        Self { 
            uart: Pic32Uart::new(0),
        }
    }
}

// impl concrete debug
impl ConcreteDebug {
    // setup
    pub fn setup(&mut self) {
        //open uart
        self.uart.open();

        //output debug info
        kernel().debug().info("Debug setup done!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl debug for concrete debug
impl Debug for ConcreteDebug {
    // log
    fn log(&mut self, log: &str) {
        self.uart.write("Log: ".as_bytes());
        self.uart.write(log.as_bytes());
        self.uart.write("\r\n".as_bytes());
    }

    // info
    fn info(&mut self, info: &str) {
        self.uart.write("\x1b[36m[Info] ".as_bytes());
        self.uart.write(info.as_bytes());
        self.uart.write("\r\n\x1b[39m".as_bytes());
    }

    // error
    fn error(&mut self, error: &str) {
        self.uart.write("\x1b[31m[Error] ".as_bytes());
        self.uart.write(error.as_bytes());
        self.uart.write("\r\n\x1b[39m".as_bytes());
    }

    // warn
    fn warn(&mut self, warn: &str) {
        self.uart.write("\x1b[33m[Warning] ".as_bytes());
        self.uart.write(warn.as_bytes());
        self.uart.write("\r\n\x1b[39m".as_bytes());
    }

    // output
    fn output(&mut self, level: i32, msg: &str) {
        if level >= 0 && level <= 5 {
            self.uart.write("\x1b[36m[Debug] ".as_bytes());
            self.uart.write(msg.as_bytes());
            self.uart.write("\r\n\x1b[39m".as_bytes());
        }
    }
}
