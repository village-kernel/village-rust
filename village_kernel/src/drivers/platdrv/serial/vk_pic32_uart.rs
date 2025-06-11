//###########################################################################
// vk_pic32_uart.rs
// The specific implementation of functions related to pic32 uart
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::vendor::ia32legacy::core::i686::*;

static COMX: [u16; 4] = [COM1, COM2, COM3, COM4];

// Struct Pic32Uart
pub struct Pic32Uart {
    port: usize,
}

// Impl Pic32Uart
impl Pic32Uart {
    // New
    pub const fn new(port: usize) -> Self {
        Self { port }
    }

    // Open
    pub fn open(&self) -> bool {
        let base = COMX[self.port as usize];

        // Setup serial
        port_byte_out(base + 1, 0x00);    // Disable all interrupts
        port_byte_out(base + 3, 0x80);    // Enable DLAB (set baud rate divisor)
        port_byte_out(base + 0, 0x00);    // Set divisor to 0 (lo byte) 115200 baud
        port_byte_out(base + 1, 0x00);    //                  (hi byte)
        port_byte_out(base + 3, 0x03);    // 8 bits, no parity, one stop bit
        port_byte_out(base + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
        port_byte_out(base + 4, 0x0B);    // IRQs enabled, RTS/DSR set
        port_byte_out(base + 4, 0x1E);    // Set in loopback mode, test the serial chip
        port_byte_out(base + 0, 0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)
        
        // Check if serial is faulty (i.e: not same byte as sent)
        if port_byte_in(base + 0) != 0xAE {
            return false;
        }
        
        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        port_byte_out(base + 4, 0x0F);

        true
    }

    // Close
    pub fn close(&self) {

    }

    // Check if the send register is empty
    pub fn is_tx_register_empty(&self) -> bool {
        let status = port_byte_in(COMX[self.port as usize] + COM_LINE_STATUS_POS);
        (status & COM_LINE_STATUS_THRE_MSK) != 0
    }

    // Check if the read date register not empty
    pub fn is_read_data_reg_not_empty(&self) -> bool {
        let status = port_byte_in(COMX[self.port as usize] + COM_LINE_STATUS_POS);
        (status & COM_LINE_STATUS_DR_MSK) != 0
    }

    // Write data
    pub fn write(&self, data: &[u8], size: usize) -> usize {
        let mut count = 0;
        
        for i in 0..size {
            while !self.is_tx_register_empty() {}
            
            port_byte_out(COMX[self.port as usize],data[i]);

            count += 1;
        }
        
        count
    }

    // Read data
    pub fn read(&self, buffer: &mut [u8], size: usize) -> usize {
        let mut count = 0;
        
        for byte in buffer.iter_mut() {
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
}
