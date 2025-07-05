//###########################################################################
// vk_api.rs
// The specific implementation of functions related to village kernel api
//
// $Copyright: Copyright (C) village
//###########################################################################

// System
pub trait System {}

// Memory
pub trait Memory {}

// Debug level
pub enum DebugLevel {
    Lv0 = 0,
    Lv1,
    Lv2,
    Lv3,
    Lv4,
    Lv5
}

// Debug
pub trait Debug {
    fn log(&mut self, log: &str);
    fn info(&mut self, log: &str);
    fn error(&mut self, error: &str);
    fn warn(&mut self, warn: &str);
    fn output(&mut self, level: DebugLevel, msg: &str);
    fn set_debug_level(&mut self, level: DebugLevel);
}

// Trait kernel
pub trait Kernel {
    fn system(&mut self) -> &mut dyn System;
    fn memory(&mut self) -> &mut dyn Memory;
    fn debug(&mut self) -> &mut dyn Debug;
}
