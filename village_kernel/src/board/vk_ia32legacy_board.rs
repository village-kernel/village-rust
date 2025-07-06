//###########################################################################
// vk_ia32legacy_board.rs
// Definitions of the functions that manage board config
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::drivers::platdrv::block::vk_ata_lba_disk::AtaLbaDiskConfig;
use crate::drivers::platdrv::serial::vk_pic32_uart::Pic32UartConfig;
use crate::register_plat_device;
use crate::traits::vk_driver::{DriverID, DrvInfo, PlatData, PlatDevice};
use crate::village::kernel;
use alloc::boxed::Box;

// Struct ata lba disk dev
struct AtaLbaDiskDev {
    info: DrvInfo,
    plat: PlatData,
    config: AtaLbaDiskConfig,
}

// Impl ata lba disk dev
impl AtaLbaDiskDev {
    pub const fn new() -> Self {
        Self {
            info: DrvInfo::new(),
            plat: PlatData::new(),
            config: AtaLbaDiskConfig::new(),
        }
    }
}

// Impl plat device for ata lba disk dev
impl PlatDevice for AtaLbaDiskDev {
    fn info(&mut self) -> &mut DrvInfo {
        &mut self.info
    }

    fn plat(&mut self) -> &mut PlatData {
        &mut self.plat
    }

    fn config(&mut self) {
        self.config = AtaLbaDiskConfig { drv: 1 };
        self.plat.set_data(&self.config);
        self.plat.set_id(DriverID::Block);
        self.plat.set_name("disk0");
    }
}

// Register plat device
register_plat_device!(AtaLbaDiskDev::new(), ataLbaDisk, ata_lba_disk_dev);

// Struct pic32 uart dev
struct Pic32UartDev {
    info: DrvInfo,
    plat: PlatData,
    config: Pic32UartConfig,
}

// Impl pic32 uart dev
impl Pic32UartDev {
    pub const fn new() -> Self {
        Self {
            info: DrvInfo::new(),
            plat: PlatData::new(),
            config: Pic32UartConfig::new(),
        }
    }
}

// Impl plat device for pic32 uart dev
impl PlatDevice for Pic32UartDev {
    fn info(&mut self) -> &mut DrvInfo {
        &mut self.info
    }

    fn plat(&mut self) -> &mut PlatData {
        &mut self.plat
    }

    fn config(&mut self) {
        self.config = Pic32UartConfig { port: 0 };
        self.plat.set_data(&self.config);
        self.plat.set_id(DriverID::Char);
        self.plat.set_name("serial0");
    }
}

// Register plat device
register_plat_device!(Pic32UartDev::new(), pic32uart, pic32_uart_dev);
