//###########################################################################
// vk_system.rs
// The specific implementation of functions related to system
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_callback::Callback;
use crate::traits::vk_kernel::System;
use crate::vendor::ia32legacy::core::i686::*;
use crate::village::kernel;
use core::arch::asm;

// Struct village system
pub struct VillageSystem {
    systicks: u32,
}

// Impl village system
impl VillageSystem {
    pub const fn new() -> Self {
        Self { systicks: 0 }
    }
}

// Impl village system
impl VillageSystem {
    // Setup
    pub fn setup(&mut self) {
        // Set interrupt handler
        kernel().interrupt().set_isr_cb(
            SYSTICK_IRQN,
            Callback::new(Self::systick_handler as u32).with_instance(self),
        );

        // Configure clock
        self.configure_clock();
    }

    // Exit
    pub fn exit(&mut self) {
        kernel().interrupt().del_isr_cb(
            SYSTICK_IRQN,
            Callback::new(Self::systick_handler as u32).with_instance(self),
        );
    }

    // Configure clock
    fn configure_clock(&mut self) {
        // Reset systicks
        self.systicks = 0;

        // Get the PIT value: hardware clock at 1193182 Hz
        let freq = 1000; //1000hz, 1ms
        let divider = 1193182 / freq;
        let low = ((divider >> 0) & 0xFF) as u8;
        let high = ((divider >> 8) & 0xFF) as u8;

        // Send the command
        port_byte_out(TIMER_CMD, 0x36); //Command port
        port_byte_out(TIMER_CH0, low);
        port_byte_out(TIMER_CH0, high);
    }

    // System clock handler
    fn systick_handler(&mut self) {
        self.systicks = self.systicks + 1;
    }
}

// Impl system for village system
impl System for VillageSystem {
    // Get system ticks
    fn get_ticks(&mut self) -> u32 {
        self.systicks
    }

    // Delay ms
    fn delay_ms(&mut self, millis: u32) {
        let delay_start = self.systicks;
        let delay_cycles = millis;
        loop {
            if (self.systicks - delay_start) >= delay_cycles {
                break;
            }
        }
    }

    // Enable irq
    fn enable_irq(&mut self) {
        unsafe {
            asm!("sti");
        }
    }

    // Disable irq
    fn disable_irq(&mut self) {
        unsafe {
            asm!("cli");
        }
    }

    // Sleep
    fn sleep(&mut self) {}

    // Standby
    fn standby(&mut self) {}

    // Shutdown
    fn shutdown(&mut self) {}

    // Reboot
    fn reboot(&mut self) {}
}
