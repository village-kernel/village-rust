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

pub struct ConcreteDebug;

impl Debug for ConcreteDebug {
    fn log(&self, log: &str) {
        print(log);
    }

    fn info(&self, info: &str) {
        print(info);
    }

    fn error(&self, error: &str) {
        print(error);
    }

    fn warn(&self, warn: &str) {
        print(warn);
    }

    fn output(&self, level: i32, msg: &str) {
        if level >= 0 && level <= 5 {
            print(msg);
        }
    }
}
