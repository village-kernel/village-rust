//###########################################################################
// vk_bochs_vbe.rs
// The specific implementation of functions related to bochs vbe
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::drivers::chipdrv::ia32legacy::vk_pci_controller::PCIController;
use crate::traits::vk_driver::{Command, FBCommand, Driver, FBDriver, PlatDevWrapper, PlatDriver};
use crate::vendor::ia32legacy::core::i686::*;
use alloc::boxed::Box;
use crate::register_plat_driver;

/* Constant members begin */ 
pub const VBE_DISPI_IOPORT_INDEX: u16       = 0x01CE;
pub const VBE_DISPI_IOPORT_DATA: u16        = 0x01CF;

pub const VBE_DISPI_INDEX_ID: u16           = 0;
pub const VBE_DISPI_INDEX_XRES: u16         = 1;
pub const VBE_DISPI_INDEX_YRES: u16         = 2;
pub const VBE_DISPI_INDEX_BPP: u16          = 3;
pub const VBE_DISPI_INDEX_ENABLE: u16       = 4;
pub const VBE_DISPI_INDEX_BANK: u16         = 5;
pub const VBE_DISPI_INDEX_VIRT_WIDTH: u16   = 6;
pub const VBE_DISPI_INDEX_VIRT_HEIGHT: u16  = 7;
pub const VBE_DISPI_INDEX_X_OFFSET: u16     = 8;
pub const VBE_DISPI_INDEX_Y_OFFSET: u16     = 9;

pub const VBE_DISPI_DISABLED: u16           = 0x00;
pub const VBE_DISPI_ENABLED: u16            = 0x01;
pub const VBE_DISPI_LFB_ENABLED: u16        = 0x40;
pub const VBE_DISPI_NOCLEARMEM: u16         = 0x80;

pub const VBE_DISPI_BPP_4: u16              = 0x04;
pub const VBE_DISPI_BPP_8: u16              = 0x08;
pub const VBE_DISPI_BPP_15: u16             = 0x0F;
pub const VBE_DISPI_BPP_16: u16             = 0x10;
pub const VBE_DISPI_BPP_24: u16             = 0x18;
pub const VBE_DISPI_BPP_32: u16             = 0x20;

pub const VBE_DISPI_ID0: u16                = 0xB0C0;
pub const VBE_DISPI_ID1: u16                = 0xB0C1;
pub const VBE_DISPI_ID2: u16                = 0xB0C2;
pub const VBE_DISPI_ID3: u16                = 0xB0C3;
pub const VBE_DISPI_ID4: u16                = 0xB0C4;
pub const VBE_DISPI_ID5: u16                = 0xB0C5;
/* Constant members end */ 

// Struct BochsVBEConfig
#[derive(Clone)]
pub struct BochsVBEConfig {
    pub vmap: *mut u16,
    pub width: u16,
    pub height: u16,
    pub bit_depth: u16,
}

// Impl BochsVBEConfig
impl BochsVBEConfig {
    pub const fn new() -> Self {
        Self {
            vmap: core::ptr::null_mut(),
            width: 0,
            height: 0,
            bit_depth: 0,
        }
    }
}

// Struct BochsVBE
pub struct BochsVBE {
    config: BochsVBEConfig,
    pci: PCIController,
}

// Impl BochsVBE
impl BochsVBE {
    // New
    pub const fn new() -> Self {
        Self {
            config: BochsVBEConfig::new(),
            pci: PCIController,
        }
    }

    // Set config
    fn set_config(&mut self, data: *mut ()) {
        if !data.is_null() {
            self.config = unsafe { (*(data as *mut BochsVBEConfig)).clone() }
        }
    }
}

// Impl BochsVBE
impl BochsVBE {
    // Write data
    #[inline]
    fn write_data(&mut self, reg: usize, val: u16) {
        unsafe { *self.config.vmap.wrapping_add(reg) = val; }
    }

    // Read data
    #[inline]
    fn read_data(&mut self, reg: usize) -> u16 {
        unsafe { *self.config.vmap.wrapping_add(reg) }
    }

    // Write reg
    #[inline]
    fn write_reg(&mut self, reg: u16, dat: u16) {
        port_word_out(VBE_DISPI_IOPORT_INDEX, reg);
        port_word_out(VBE_DISPI_IOPORT_DATA, dat);
    }

    // Read reg
    #[inline]
    fn read_reg(&mut self, reg: u16) -> u16 {
        port_word_out(VBE_DISPI_IOPORT_INDEX, reg);
        port_word_in(VBE_DISPI_IOPORT_DATA)
    }
}

// Impl BochsVBE
impl BochsVBE {
    // Is bochs vbe available
    pub fn is_bochs_vbe_available(&mut self) -> bool {
        self.read_reg(VBE_DISPI_INDEX_ID) >= VBE_DISPI_ID4
    }

    // Set video mode
    pub fn set_video_mode(&mut self, width: u16, height: u16, bit_depth: u16, 
        is_use_linear_fb: bool, is_clear_video_memory: bool) 
    {
        // Disable display
        self.write_reg(VBE_DISPI_INDEX_ENABLE, VBE_DISPI_DISABLED);

        // Set width
        self.write_reg(VBE_DISPI_INDEX_XRES, width);

        // Set height
        self.write_reg(VBE_DISPI_INDEX_YRES, height);
        
        // Set bit depth
        self.write_reg(VBE_DISPI_INDEX_BPP,  bit_depth);

        // Enable display
        let mut config = VBE_DISPI_ENABLED;
        if  is_use_linear_fb      { config |= VBE_DISPI_LFB_ENABLED; }
        if !is_clear_video_memory { config |= VBE_DISPI_NOCLEARMEM;  }
        self.write_reg(VBE_DISPI_INDEX_ENABLE, config);
    }

    // Set bank
    pub fn set_bank(&mut self, bank_num: u16) {
        self.write_reg(VBE_DISPI_INDEX_BANK, bank_num);
    }
}

// Impl fb driver for bochs VBE
impl FBDriver for BochsVBE {
    // Init
    fn init(&mut self) -> bool {
        if self.is_bochs_vbe_available() {
            // Set video mode
            self.set_video_mode(
                self.config.width,
                self.config.height,
                self.config.bit_depth,
                true,
                true
            );

            // Get PCI device 0x01234:0x1111 BAR 0
            self.config.vmap = self.pci.read_bar(0x1234, 0x1111, 0) as *mut u16;

            // Reture false when vmap is null
            if self.config.vmap == core::ptr::null_mut() {
                return false;
            }

            return true;
        }
        false
    }
    
    // Get width
    fn width(&mut self) -> u32 {
        self.config.width as u32
    }

    // Get height
    fn height(&mut self) -> u32 {
        self.config.height as u32
    }

    // Draw point
    fn draw_point(&mut self, x: u32, y: u32, color: u32) {
        let reg = x + y * self.config.width as u32;
        self.write_data(reg as usize, color as u16);
    }

    // Read point
    fn read_point(&mut self, x: u32, y: u32) -> u32 {
        let reg = x + y * self.config.width as u32;
        self.read_data(reg as usize) as u32
    }

    // Fill color
    fn fill_color(&mut self, sx: u32, sy: u32, ex: u32, ey: u32, color: u32) {
        for y in sy..ey {
            for x in sx..ex {
                self.draw_point(x, y, color);
            }
        }
    }
    
    // Fill pixel
    fn fill_pixel(&mut self, sx: u32, sy: u32, ex: u32, ey: u32, pixel: &[u16]) {
        let mut index = 0;
        for y in sy..ey {
            for x in sx..ex {
                self.draw_point(x, y, pixel[index] as u32);
                index += 1;
            }
        }
    }

    // Clear
    fn clear(&mut self) {
        self.fill_color(0, 0, self.config.width as u32, self.config.height as u32, 0xFFFF);
    }

    // Exit
    fn exit(&mut self) {}
}

// Impl driver for bochs VBE
impl Driver for BochsVBE {
    // Open
    fn open(&mut self, data: *mut ()) -> bool {
        // Get config
        self.set_config(data);

        self.init()
    }

    // Ioctrl
    fn ioctrl(&mut self, command: &mut Command) -> bool {
        match command {
            Command::FB(fb_cmd) => match fb_cmd{

                FBCommand::Width { width } => {
                    *width = self.width();
                    true
                },

                FBCommand::Height { height } => {
                    *height = self.height();
                    true
                },

                FBCommand::DrawPoint { x, y, color } => {
                    self.draw_point(*x, *y, *color);
                    true
                },

                FBCommand::ReadPoint { x, y, color } => {
                    *color = self.read_point(*x, *y);
                    true
                },

                FBCommand::FillColor { sx, sy, ex, ey, color } => {
                    self.fill_color(*sx, *sy, *ex, *ey, *color);
                    true
                },

                FBCommand::FillPixel { sx, sy, ex, ey, pixel } => {
                    self.fill_pixel(*sx, *sy, *ex, *ey, pixel);
                    true
                },

                FBCommand::Clear {  } => {
                    self.clear();
                    true
                },
            },
            _ => { false }
        }
    }

    // Close
    fn close(&mut self) {
        self.exit();
    }
}

// Struct bochs vbe drv
struct BochsVBEDrv;

// Impl plat driver for bochs vbe driver
impl PlatDriver for BochsVBEDrv {
    // Probe
    fn probe(&mut self, device: &mut PlatDevWrapper) -> bool {
        device.plat().attach(Box::new(BochsVBE::new()));
        true
    }

    // Remove
    fn remove(&mut self, device: &mut PlatDevWrapper) -> bool {
        device.plat().detach();
        true
    }
}

// Register plat driver
register_plat_driver!(BochsVBEDrv, bochsVBE, bochs_vbe_drv);
