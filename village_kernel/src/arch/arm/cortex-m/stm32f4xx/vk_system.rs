//###########################################################################
// vk_system.rs
// The specific implementation of functions related to system
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::System;

/// struct concrete system
pub struct ConcreteSystem;

/// impl concrete system
impl ConcreteSystem { 
    pub const fn new() -> Self {
        Self { }
    }
}

/// impl concrete system
impl ConcreteSystem {
    /// setup
    pub fn setup(&self) {

    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl system for concrete system
impl System for ConcreteSystem {
    /// systick counter
    fn systick_counter(&self) {

    }

    /// get sysclk counts
    fn get_sysclk_counts(&self) -> u32 {
        0
    }

    /// delay ms
    fn delay_ms(&self, millis: u32) {
        
    }

    /// enable irq
    fn enable_irq(&self) {

    }

    /// disable irq
    fn disable_irq(&self) {

    }

    /// sleep
    fn sleep(&self) {

    }

    /// standby
    fn standby(&self) {

    }

    /// shutdown
    fn shutdown(&self) {

    }

    /// reboot
    fn reboot(&self) {
        
    }
}
