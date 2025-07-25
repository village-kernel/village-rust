//###########################################################################
// vk_pci_controller.rs
// The specific implementation of functions related to PCI controller
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::vendor::ia32legacy::core::i686::*;

// Struct PCIController
pub struct PCIController;

// Impl PCIController
impl PCIController {
    // Read data
    fn read_data(&mut self, bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
        // Crate configuration address
        let address = ((bus as u32) << 16) | 
                      ((dev as u32) << 16)    |
                      ((dev as u32) << 16)    |
                      ((func as u32) << 16)   |
                      ((offset as u32) << 16) | 
                      (0x80000000);
        // Write out the address
        port_long_out(0xCF8, address);
        // Read in the data
        port_long_in(0xCFC)
    }

    // Get vendor id
    fn get_vendor_id(&mut self, bus: u8, dev: u8) -> u16 {
        let value = self.read_data(bus, dev, 0, 0);
        (value & 0xFFFF) as u16 
    }

    // Get device id
    fn get_device_id(&mut self, bus: u8, dev: u8) -> u16 {
        let value = self.read_data(bus, dev, 0, 2);
        ((value >> 16) & 0xFFFF) as u16 
    }

    // Get base addr
    fn get_base_addr(&mut self, bus: u8, dev: u8, bar: u8) -> u32 {
        let offset = (0x4 << 2 << bar) as u8;
        self.read_data(bus, dev, 0, offset) & 0xFFFFFFF0
    }

    // Read BAR
    pub fn read_bar(&mut self, vendor_id: u16, device_id: u16, bar: u8) -> u32 {
        for bus in 0..=255 {
            for dev in 0..32 {
                if vendor_id == self.get_vendor_id(bus, dev) {
                    if device_id == self.get_device_id(bus, dev) {
                        return self.get_base_addr(bus, dev, bar);
                    }
                }
            }
        }
        0
    }
}
