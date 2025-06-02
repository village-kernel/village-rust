use crate::kernel::traits::vk_kernel::System;

pub struct ConcreteSystem;

impl System for ConcreteSystem {
    fn systick_counter(&self) {

    }

    fn get_sysclk_counts(&self) -> u32 {
        0
    }

    fn delay_ms(&self, millis: u32) {
        
    }

    fn enable_irq(&self) {

    }

    fn disable_irq(&self) {

    }

    fn sleep(&self) {

    }

    fn standby(&self) {

    }

    fn shutdown(&self) {

    }

    fn reboot(&self) {
        
    }
}
