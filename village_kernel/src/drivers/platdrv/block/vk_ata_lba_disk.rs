//###########################################################################
// vk_ata_lba_disk.rs
// The specific implementation of functions related to ata lba disk
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::lock::vk_mutex::Mutex;
use crate::register_plat_driver;
use crate::traits::vk_driver::{Driver, PlatDevWrapper, PlatDriver};
use crate::vendor::ia32legacy::core::i686::*;
use alloc::boxed::Box;

// Struct AtaLbaDiskConfig
#[derive(Clone)]
pub struct AtaLbaDiskConfig {
    pub drv: usize,
}

// Impl AtaLbaDiskConfig
impl AtaLbaDiskConfig {
    pub const fn new() -> Self {
        Self { drv: 0 }
    }
}

// Struct AtaLbaDisk
pub struct AtaLbaDisk {
    mutex: Mutex,
    config: AtaLbaDiskConfig,
}

// Impl AtaLbaDisk
impl AtaLbaDisk {
    pub const fn new() -> Self {
        Self {
            mutex: Mutex::new(),
            config: AtaLbaDiskConfig::new(),
        }
    }

    // Set config
    fn set_config(&mut self, data: *mut ()) {
        if !data.is_null() {
            self.config = unsafe { (*(data as *mut AtaLbaDiskConfig)).clone() }
        }
    }
}

// Impl AtaLbaDisk
impl Driver for AtaLbaDisk {
    // Open
    fn open(&mut self, data: *mut ()) -> bool {
        // Get config
        self.set_config(data);

        // Stop device from sending interrupts
        self.mutex.lock();
        port_byte_out(ATA_PRIMARY_PORT_CTRL, ATA_CTRL_N_IEN);
        port_byte_out(ATA_SECOND_PORT_CTRL, ATA_CTRL_N_IEN);
        self.mutex.unlock();

        true
    }

    // Write data
    fn write(&mut self, data: &[u8], count: usize, blk: usize) -> usize {
        self.mutex.lock();

        let mut blk = blk;

        for cnt in 0..count {
            // LBA 28 mode
            let val = (self.config.drv << ATA_MODE_DRV_POS) | ((blk >> 24) & 0x0f);
            port_byte_out(ATA_MODE, ATA_MODE_LBA | (val as u8));

            // Write one sector
            port_byte_out(ATA_SECTOR_CNT, 1);

            // Set block address
            port_byte_out(ATA_SECTOR_0_7_BITS, (blk >> 0) as u8);
            port_byte_out(ATA_SECTOR_8_15_BITS, (blk >> 8) as u8);
            port_byte_out(ATA_SECTOR_16_23_BITS, (blk >> 16) as u8);

            // Write cmd
            port_byte_out(ATA_CMD, ATA_CMD_WRITE);

            // Wait
            while ATA_STATUS_BSY_MSK == (port_byte_in(ATA_STATUS) & ATA_STATUS_BSY_MSK) {}
            while ATA_STATUS_RDY_MSK != (port_byte_in(ATA_STATUS) & ATA_STATUS_RDY_MSK) {}

            // Write data
            for size in 0..256 {
                let index = (size + cnt * 256) * 2;
                if index + 1 < data.len() {
                    let value = (data[index] as u16) | ((data[index + 1] as u16) << 8);
                    port_word_out(ATA_DATA, value);
                }
            }

            // Flush cache
            port_byte_out(ATA_CMD, ATA_CMD_FLUSH);

            // Wait
            while ATA_STATUS_BSY_MSK == (port_byte_in(ATA_STATUS) & ATA_STATUS_BSY_MSK) {}

            // Add blk
            blk += 1;
        }

        self.mutex.unlock();

        count
    }

    // Read data
    fn read(&mut self, data: &mut [u8], count: usize, blk: usize) -> usize {
        self.mutex.lock();

        let mut blk = blk;

        for cnt in 0..count {
            // LBA 28 mode
            let val = (self.config.drv << ATA_MODE_DRV_POS) | ((blk >> 24) & 0x0f);
            port_byte_out(ATA_MODE, ATA_MODE_LBA | (val as u8));

            // Read one sector
            port_byte_out(ATA_SECTOR_CNT, 1);

            // Set block address
            port_byte_out(ATA_SECTOR_0_7_BITS, (blk >> 0) as u8);
            port_byte_out(ATA_SECTOR_8_15_BITS, (blk >> 8) as u8);
            port_byte_out(ATA_SECTOR_16_23_BITS, (blk >> 16) as u8);

            // Read cmd
            port_byte_out(ATA_CMD, ATA_CMD_READ);

            // Wait
            while ATA_STATUS_BSY_MSK == (port_byte_in(ATA_STATUS) & ATA_STATUS_BSY_MSK) {}
            while ATA_STATUS_RDY_MSK != (port_byte_in(ATA_STATUS) & ATA_STATUS_RDY_MSK) {}

            // Read data
            for size in 0..256 {
                let value = port_word_in(ATA_DATA);
                let index = (size + cnt * 256) * 2;
                if index + 1 < data.len() {
                    data[index] = (value & 0xFF) as u8;
                    data[index + 1] = ((value >> 8) & 0xFF) as u8;
                }
            }

            // Add blk
            blk += 1;
        }

        self.mutex.unlock();

        count
    }

    // Close
    fn close(&mut self) {}
}

// Struct ata lba disk drv
struct AtaLbaDiskDrv;

// Impl plat driver for ata lba disk driver
impl PlatDriver for AtaLbaDiskDrv {
    // Probe
    fn probe(&mut self, device: &mut PlatDevWrapper) -> bool {
        device.plat().attach(Box::new(AtaLbaDisk::new()));
        true
    }

    // Remove
    fn remove(&mut self, device: &mut PlatDevWrapper) -> bool {
        device.plat().detach();
        true
    }
}

// Register plat driver
register_plat_driver!(AtaLbaDiskDrv, ataLbaDisk, ata_lba_disk_drv);
