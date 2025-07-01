//###########################################################################
// vK_driver.rs
// The interfaces of functions related to driver
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::{boxed::Box, string::{String, ToString}};
use crate::village::kernel;

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
        [DriverID::Block, DriverID::Char, DriverID::Display, 
         DriverID::Input, DriverID::Network, DriverID::Misc].into_iter()
    }

    // Rev iterator
    pub fn rev_iter() -> impl Iterator<Item = DriverID> {
        [DriverID::Misc, DriverID::Network, DriverID::Input,
         DriverID::Display, DriverID::Char, DriverID::Block].into_iter()
    }
}

// Impl driver id
impl DriverID {
    pub fn as_str(&self) -> &'static str {
        match self {
            DriverID::Block   => "block driver",
            DriverID::Char    => "char driver",
            DriverID::Display => "display driver",
            DriverID::Input   => "input driver",
            DriverID::Network => "network driver",
            DriverID::Misc    => "misc driver",
            _ => "",
        }
    }
}

// Struct driver info
pub struct DrvInfo {
    id: DriverID,
    name: String,
    data: *mut(),
}

// Impl driver data
impl DrvInfo {
    // New
    pub const fn new() -> Self {
        Self {
            id: DriverID::Misc,
            name: String::new(),
            data: core::ptr::null_mut(),
        }
    }

    // Set id
    pub fn set_id(&mut self, id: DriverID) {
        self.id = id;
    }

    // Get id
    pub fn get_id(&self) -> DriverID {
        self.id.clone()
    }

    // Set name
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // Get name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Set data
    pub fn set_data(&mut self, data: *mut()) {
        self.data = data;
    }

    // Get data
    pub fn get_data<T>(&mut self) -> Option<&mut T> {
        if self.data.is_null() {
            return None;
        }
        unsafe { Some(&mut *(self.data as *mut T)) }
    }
}

// Driver
pub trait Driver {
    fn info(&mut self) -> &mut DrvInfo;
    fn open(&mut self) -> bool;
    fn write(&mut self, _data: &[u8], _size: usize, _offset: usize) -> usize { 0 }
    fn read(&mut self, _data: &mut [u8], _size: usize, _offset: usize) -> usize { 0 }
    fn ioctrl(&mut self, _cmd: u8, _data: &[u8]) -> usize { 0 }
    fn close(&mut self);
}

// Plat data
pub struct PlatData {
    pub drvid: DriverID,
    pub drvname: String,
    pub drvdata: *mut(),
    is_attach: bool,
}

impl PlatData {
    // New
    pub const fn new() -> Self {
        Self {
            drvid: DriverID::Misc,
            drvname: String::new(),
            drvdata: core::ptr::null_mut(),
            is_attach: false,
        }
    }

    // Attach
    pub fn attach(&mut self, mut driver: Box<dyn Driver>) {
        driver.info().set_id(self.drvid.clone());
        driver.info().set_name(&self.drvname);
        driver.info().set_data(self.drvdata);
        kernel().device().register_driver(driver);
        self.is_attach = true;
    }

    // Detach
    pub fn detach(&mut self) {
        kernel().device().unregister_driver(&self.drvname);
        self.is_attach = false;
    }

    // Is attach
    pub fn is_attach(&self) -> bool {
        self.is_attach
    }

    // Set id
    pub fn set_id(&mut self, id: DriverID) {
        self.drvid = id;
    }

    // Set name
    pub fn set_name(&mut self, name: &str) {
        self.drvname = name.to_string();
    }

    // Set data
    pub fn set_data<T>(&mut self, data: &T) {
        self.drvdata = data as *const T as *mut ();
    }
}

pub trait PlatDriver {
    fn info(&mut self) -> &mut DrvInfo;
    
    fn probe(&mut self, device: &mut dyn PlatDevice) -> bool;
    fn remove(&mut self, device: &mut dyn PlatDevice) -> bool;
}

pub trait PlatDevice {
    fn info(&mut self) -> &mut DrvInfo;
    fn plat(&mut self) -> &mut PlatData;

    fn config(&mut self);
    fn release(&mut self) {}
}

// Register driver macro
#[macro_export]
macro_rules! register_driver {
    ($drv:expr, $id:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[link_section = ".init_array"]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[link_section = ".fini_array"]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let mut driver = Box::new($drv);
                driver.info().set_name(stringify!($name));
                driver.info().set_id($id);
                kernel().device().register_driver(driver);
            }

            fn [<$name _exit>]() {
                kernel().device().unregister_driver(stringify!($name));
            }
        }
    };
}

// Register plat driver macro
#[macro_export]
macro_rules! register_plat_driver {
    ($drv:expr, $name:ident, $fn_name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $fn_name:upper>]: fn() = [<$fn_name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $fn_name:upper>]: fn() = [<$fn_name _exit>];

            fn [<$fn_name _init>]() {
                let mut driver = Box::new($drv);
                driver.info().set_name(stringify!($name));
                driver.info().set_id(DriverID::PlatDrv);
                kernel().device().register_plat_driver(driver);
            }

            fn [<$fn_name _exit>]() {
                kernel().device().unregister_plat_driver(stringify!($name));
            }
        }
    };
}

// Register plat device macro
#[macro_export]
macro_rules! register_plat_device {
    ($drv:expr, $name:ident, $fn_name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $fn_name:upper>]: fn() = [<$fn_name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $fn_name:upper>]: fn() = [<$fn_name _exit>];

            fn [<$fn_name _init>]() {
                let mut device = Box::new($drv);
                device.info().set_name(stringify!($name));
                device.info().set_id(DriverID::PlatDev);
                kernel().device().register_plat_device(device);
            }

            fn [<$fn_name _exit>]() {
                kernel().device().unregister_plat_device(stringify!($name));
            }
        }
    };
}
