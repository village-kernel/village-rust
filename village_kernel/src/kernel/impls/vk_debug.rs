//###########################################################################
// vk_debug.rs
// The specific implementation of functions related to debug
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel::traits::vk_kernel::kernel;
use crate::kernel::traits::vk_kernel::Debug;

// 打印
fn print(message: &str) {
    // 示例：向0xB8000写入文本（VGA文本缓冲区）
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in message.as_bytes().iter().enumerate() {
        unsafe {
            *vga_buffer.add(i * 2) = byte;
            *vga_buffer.add(i * 2 + 1) = 0x0f; // 白字黑底
        }
    }
}

/// struct concrete debug
pub struct ConcreteDebug;

/// impl concrete debug
impl ConcreteDebug {
    /// setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Debug setup done!");
    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl debug for concrete debug
impl Debug for ConcreteDebug {
    /// log
    fn log(&self, log: &str) {
        print(log);
    }

    /// info
    fn info(&self, info: &str) {
        print(info);
    }

    /// error
    fn error(&self, error: &str) {
        print(error);
    }

    /// warn
    fn warn(&self, warn: &str) {
        print(warn);
    }

    /// output
    fn output(&self, level: i32, msg: &str) {
        if level >= 0 && level <= 5 {
            print(msg);
        }
    }
}
