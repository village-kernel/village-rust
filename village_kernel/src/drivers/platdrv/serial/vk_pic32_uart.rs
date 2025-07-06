//###########################################################################
// vk_pic32_uart.rs
// The specific implementation of functions related to pic32 uart
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_plat_driver;
use crate::traits::vk_driver::{Driver, DriverID, DrvInfo, PlatDevice, PlatDriver};
use crate::vendor::ia32legacy::core::i686::*;
use crate::village::kernel;
use alloc::boxed::Box;

// Constants
const COMX: [u16; 7] = [COM1, COM2, COM3, COM4, COM5, COM6, COM7];

// Struct Pic32UartConfig
pub struct Pic32UartConfig {
    pub port: usize,
}

// Impl Pic32UartConfig
impl Pic32UartConfig {
    pub const fn new() -> Self {
        Self { port: 0 }
    }
}

// Struct Pic32Uart
pub struct Pic32Uart {
    info: DrvInfo,
    port: usize,
}

// Impl Pic32Uart
impl Pic32Uart {
    pub const fn new() -> Self {
        Self {
            info: DrvInfo::new(),
            port: 0,
        }
    }
}

// Impl Pic32Uart
impl Pic32Uart {
    // Check if the send register is empty
    fn is_tx_register_empty(&self) -> bool {
        let status = port_byte_in(COMX[self.port as usize] + COM_LINE_STATUS_POS);
        (status & COM_LINE_STATUS_THRE_MSK) != 0
    }

    // Check if the read date register not empty
    fn is_read_data_reg_not_empty(&self) -> bool {
        let status = port_byte_in(COMX[self.port as usize] + COM_LINE_STATUS_POS);
        (status & COM_LINE_STATUS_DR_MSK) != 0
    }
}

// Impl Pic32Uart
impl Driver for Pic32Uart {
    // Info
    fn info(&mut self) -> &mut DrvInfo {
        &mut self.info
    }

    // Open
    fn open(&mut self) -> bool {
        // Get config
        let config = self.info.get_data::<Pic32UartConfig>().unwrap();

        // Set port
        self.port = config.port;

        // Get base
        let base = COMX[self.port as usize];

        // Setup serial
        port_byte_out(base + 1, 0x00); // Disable all interrupts
        port_byte_out(base + 3, 0x80); // Enable DLAB (set baud rate divisor)
        port_byte_out(base + 0, 0x00); // Set divisor to 0 (lo byte) 115200 baud
        port_byte_out(base + 1, 0x00); //                  (hi byte)
        port_byte_out(base + 3, 0x03); // 8 bits, no parity, one stop bit
        port_byte_out(base + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        port_byte_out(base + 4, 0x0B); // IRQs enabled, RTS/DSR set
        port_byte_out(base + 4, 0x1E); // Set in loopback mode, test the serial chip
        port_byte_out(base + 0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if port_byte_in(base + 0) != 0xAE {
            return false;
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        port_byte_out(base + 4, 0x0F);

        true
    }

    // Write data
    fn write(&mut self, data: &[u8], size: usize, _offset: usize) -> usize {
        let mut count = 0;

        for byte in data {
            while !self.is_tx_register_empty() {}

            port_byte_out(COMX[self.port as usize], *byte);

            count += 1;

            if count >= size {
                break;
            }
        }

        count
    }

    // Read data
    fn read(&mut self, data: &mut [u8], size: usize, _offset: usize) -> usize {
        let mut count = 0;

        for byte in data.iter_mut() {
            if !self.is_read_data_reg_not_empty() {
                break;
            }

            *byte = port_byte_in(COMX[self.port as usize]);

            count += 1;

            if count >= size {
                break;
            }
        }

        count
    }

    // Close
    fn close(&mut self) {}
}

// Struct pic32 uart drv
struct Pic32UartDrv {
    data: DrvInfo,
}

// Impl pic32 uart driver
impl Pic32UartDrv {
    pub const fn new() -> Self {
        Self {
            data: DrvInfo::new(),
        }
    }
}

// Impl plat driver for pic32 uart driver
impl PlatDriver for Pic32UartDrv {
    fn info(&mut self) -> &mut DrvInfo {
        &mut self.data
    }

    fn probe(&mut self, device: &mut dyn PlatDevice) -> bool {
        device.plat().attach(Box::new(Pic32Uart::new()));
        true
    }

    fn remove(&mut self, device: &mut dyn PlatDevice) -> bool {
        device.plat().detach();
        true
    }
}

// Register plat driver
register_plat_driver!(Pic32UartDrv::new(), pic32uart, pic32_uart_drv);
