//###########################################################################
// vk_system.rs
// The specific implementation of functions related to system
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel::traits::vk_kernel::System;

/// struct concrete system
pub struct ConcreteSystem;

// impl concrete system
impl ConcreteSystem {    
    pub const fn new() -> Self {
        Self { }
    }
}

/// impl concrete system
impl ConcreteSystem {
    /// setup
    pub fn setup(&mut self) {

    }

    /// exit
    pub fn exit(&mut self) {

    }
}

/// impl system for concrete system
impl System for ConcreteSystem {
    /// systick counter
    fn systick_counter(&mut self) {

    }

    /// get sysclk counts
    fn get_sysclk_counts(&mut self) -> u32 {
        0
    }

    /// delay ms
    fn delay_ms(&mut self, millis: u32) {
        
    }

    /// enable irq
    fn enable_irq(&mut self) {

    }

    /// disable irq
    fn disable_irq(&mut self) {

    }

    /// sleep
    fn sleep(&mut self) {

    }

    /// standby
    fn standby(&mut self) {

    }

    /// shutdown
    fn shutdown(&mut self) {

    }

    /// reboot
    fn reboot(&mut self) {
        
    }
}
