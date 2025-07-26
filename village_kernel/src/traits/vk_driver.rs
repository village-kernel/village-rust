//###########################################################################
// vK_driver.rs
// The interfaces of functions related to driver
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Driver id
#[derive(PartialEq, Clone)]
pub enum DriverID {
    Block = 0,
    Char,
    Display,
    Input,
    Network,
    Misc,
    PlatDev,
    PlatDrv,
}

// Impl driver id
impl DriverID {
    // Iterator
    pub fn iter() -> impl Iterator<Item = DriverID> {
        [
            DriverID::Block,
            DriverID::Char,
            DriverID::Display,
            DriverID::Input,
            DriverID::Network,
            DriverID::Misc,
        ]
        .into_iter()
    }

    // Rev iterator
    pub fn rev_iter() -> impl Iterator<Item = DriverID> {
        [
            DriverID::Misc,
            DriverID::Network,
            DriverID::Input,
            DriverID::Display,
            DriverID::Char,
            DriverID::Block,
        ]
        .into_iter()
    }

    // as string
    pub fn as_str(&self) -> &'static str {
        match self {
            DriverID::Block => "block driver",
            DriverID::Char => "char driver",
            DriverID::Display => "display driver",
            DriverID::Input => "input driver",
            DriverID::Network => "network driver",
            DriverID::Misc => "misc driver",
            _ => "",
        }
    }
}

// Common command
pub enum Command {
    FB(FBCommand),
    NULL(),
}

// FBCommand
pub enum FBCommand {
    Width     { width: u32 },
    Height    { height: u32 },
    DrawPoint { x: u32, y: u32, color: u32 },
    ReadPoint { x: u32, y: u32, color: u32 },
    FillColor { sx: u32, sy: u32, ex: u32, ey: u32, color: u32 },
    FillPixel { sx: u32, sy: u32, ex: u32, ey: u32, pixel: Vec<u16> },
    Clear     { },
}

// Trait Driver
pub trait Driver {
    fn open(&mut self, data: *mut ()) -> bool;
    fn write(&mut self, _data: &[u8], _size: usize, _offset: usize) -> usize { 0 }
    fn read(&mut self, _data: &mut [u8], _size: usize, _offset: usize) -> usize { 0 }
    fn ioctrl(&mut self, _command: &mut Command) -> bool { false }
    fn close(&mut self);
}

// Trait FBDriver
pub trait FBDriver {
    fn init(&mut self) -> bool;
    fn width(&mut self) -> u32;
    fn height(&mut self) -> u32;
    fn draw_point(&mut self, x: u32, y: u32, color: u32);
    fn read_point(&mut self, x: u32, y: u32) -> u32;
    fn fill_color(&mut self, sx: u32, sy: u32, ex: u32, ey: u32, color: u32);
    fn fill_pixel(&mut self, sx: u32, sy: u32, ex: u32, ey: u32, pixel: &[u16]);
    fn clear(&mut self);
    fn exit(&mut self);
}

// Struct Platdata
pub struct PlatData {
    drvid: DriverID,
    drvname: &'static str,
    drvdata: *mut (),
    is_attach: bool,
}

// Impl PlatData
impl PlatData {
    // New
    pub const fn new() -> Self {
        Self {
            drvid: DriverID::Misc,
            drvname: "none",
            drvdata: core::ptr::null_mut(),
            is_attach: false,
        }
    }

    // Attach
    pub fn attach(&mut self, driver: Box<dyn Driver>) {
        if !self.is_attach {
            kernel().device().register_driver(
                DriverWrapper::new(
                    driver,
                    self.drvid.clone(),
                    self.drvname,
                    self.drvdata
                )
            );
            self.is_attach = true;
        }
    }

    // Detach
    pub fn detach(&mut self) {
        if self.is_attach {
            kernel().device().unregister_driver(&self.drvname);
            self.is_attach = false;
        }
    }

    // Is attach
    pub fn is_attach(&self) -> bool {
        self.is_attach
    }

    // Set drv id
    pub fn set_drvid(&mut self, id: DriverID) {
        self.drvid = id;
    }

    // Set drv name
    pub fn set_drvname(&mut self, name: &'static str) {
        self.drvname = name;
    }

    // Set drv data
    pub fn set_drvdata<T>(&mut self, data: &T) {
        self.drvdata = data as *const T as *mut ();
    }
}

// PlatDriver
pub trait PlatDriver {
    fn probe(&mut self, device: &mut PlatDevWrapper) -> bool;
    fn remove(&mut self, device: &mut PlatDevWrapper) -> bool;
}

// PlatDevice
pub trait PlatDevice {
    fn plat(&mut self) -> &mut PlatData;

    fn config(&mut self);
    fn release(&mut self) {}
}

// Struct driver wrapper
pub struct DriverWrapper {
    id: DriverID,
    name: &'static str,
    data: *mut (),
    inner: Box<dyn Driver>,
}

// Impl driver wrapper
impl DriverWrapper {
    // New with
    #[inline]
    pub fn new(inner: Box<dyn Driver>, id: DriverID, name: &'static str, data: *mut ()) -> Self {
        Self {
            id,
            name,
            data,
            inner,
        }
    }

    // Get id
    #[inline]
    pub fn id(&self) -> DriverID {
        self.id.clone()
    }

    // Get name
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    // Open
    #[inline]
    pub fn open(&mut self) -> bool {
        self.inner.open(self.data)
    }

    // Write
    #[inline]
    pub fn write(&mut self, data: &[u8], size: usize, offset: usize) -> usize {
        self.inner.write(data, size, offset)
    }

    // Read
    #[inline]
    pub fn read(&mut self, data: &mut [u8], size: usize, offset: usize) -> usize {
        self.inner.read(data, size, offset)
    }

    // IOctrl
    #[inline]
    pub fn ioctrl(&mut self, command: &mut Command) -> bool {
        self.inner.ioctrl(command)
    }

    // Close
    #[inline]
    pub fn close(&mut self) {
        self.inner.close();
    }
}

// Struct PlatDrvWrapper
pub struct PlatDrvWrapper {
    id: DriverID,
    name: &'static str,
    inner: Box<dyn PlatDriver>,
}

// Impl PlatDrvWrapper
impl PlatDrvWrapper {
    // New with
    #[inline]
    pub fn new(inner: Box<dyn PlatDriver>, name: &'static str) -> Self {
        Self {
            id: DriverID::PlatDrv,
            name,
            inner,
        }
    }

    // Get id
    #[inline]
    pub fn id(&self) -> DriverID {
        self.id.clone()
    }

    // Get name
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    // Probe
    #[inline]
    pub fn probe(&mut self, device: &mut PlatDevWrapper) -> bool {
        self.inner.probe(device)
    }

    // Remove
    #[inline]
    pub fn remove(&mut self, device: &mut PlatDevWrapper) -> bool {
        self.inner.remove(device)
    }
}

// Struct PlatDevWrapper
pub struct PlatDevWrapper {
    id: DriverID,
    name: &'static str,
    inner: Box<dyn PlatDevice>,
}

// Impl PlatDevWrapper
impl PlatDevWrapper {
    // New with
    #[inline]
    pub fn new(inner: Box<dyn PlatDevice>, name: &'static str) -> Self {
        Self {
            id: DriverID::PlatDev,
            name,
            inner,
        }
    }

    // Get id
    #[inline]
    pub fn id(&self) -> DriverID {
        self.id.clone()
    }

    // Get name
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    // Plat
    #[inline]
    pub fn plat(&mut self) -> &mut PlatData {
        self.inner.plat()
    }

    // Config
    #[inline]
    pub fn config(&mut self) {
        self.inner.config();
    }

    // Release
    #[inline]
    pub fn release(&mut self) {
        self.inner.release();
    }
}
