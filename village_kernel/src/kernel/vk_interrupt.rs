//###########################################################################
// vk_interrupt.rs
// The specific implementation of functions related to interrupt
//
// $Copyright: Copyright (C) village
//###########################################################################
use spin::Mutex;
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Interrupt;
use crate::traits::vk_callback::*;
use crate::arch::ia32::legacy::vk_exception::ConcreteException;

// Struct concrete interrupt
pub struct ConcreteInterrupt {
    is_ready: Mutex<bool>,
    exception: ConcreteException,
}

// Impl concrete interrupt
impl ConcreteInterrupt {
    pub const fn new() -> Self {
        Self {
            is_ready: Mutex::new(false),
            exception: ConcreteException,
        }
    }
}

// Impl concrete interrupt
impl ConcreteInterrupt {
    // Setup
    pub fn setup(&self) {
        // Setupt exception
        self.exception.setup();

        // Set ready flag
        *self.is_ready.lock() = true;

        // Output debug info
        kernel().debug().info("Interrupt setup done!");
    }

    // Exit
    pub fn exit(&self) {
        // Clear ready flag
        *self.is_ready.lock() = false;

        // Exit exception
        self.exception.exit();
    }
}

// Impl interrupt for concrete interrupt
impl Interrupt for ConcreteInterrupt {
     // Set ISR function callback
    fn set_isr_fn_cb(&self, irq: isize, func: FnCallback, args: *mut()) {

    }

    // Set ISR method callback
    fn set_isr_meth_cb(&mut self, irq: isize, method: MethodCb, data: *mut ()) {

    }

    // Add ISR function callback
    fn add_isr_fn_cb(&self, irq: isize, func: FnCallback, args: *mut()) {

    }

    // Add ISR method callback
    fn add_isr_meth_cb(&mut self, irq: isize, method: MethodCb, data: *mut ()) {

    }

    // Del ISR function callback
    fn del_isr_fn_cb(&self, irq: isize, func: FnCallback, args: *mut()) {

    }

    // Del ISR method callback
    fn del_isr_meth_cb(&mut self, irq: isize, method: MethodCb, data: *mut ()) {

    }
    
    // Clear ISR callbacks
    fn clear_isr_cb(&self, irq: isize) {

    }
    
    // Replace ISR handler
    fn replace(&self, handler: usize) {

    }
    
    // Feature Methods
    fn handler(&self, irq: isize) {

    }
}
