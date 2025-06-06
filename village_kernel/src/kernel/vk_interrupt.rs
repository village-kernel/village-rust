//###########################################################################
// vk_interrupt.rs
// The specific implementation of functions related to interrupt
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
//use alloc::vec::Vec;
use spin::Mutex;
use crate::village::kernel;
use crate::traits::vk_kernel::Interrupt;
use crate::traits::vk_callback::*;
use crate::arch::ia32::legacy::vk_exception::ConcreteException;

//const WARNING_TIMES: u32 = 10;

// Struct concrete interrupt
pub struct ConcreteInterrupt {
    exception: ConcreteException,
    //isr_tabs: Vec<Vec<CbInvoker>>,
    //warnings: Vec<u8>,
    is_ready: Mutex<bool>,
}

// Impl concrete interrupt
impl ConcreteInterrupt {
    pub const fn new() -> Self {
        Self {
            exception: ConcreteException::new(),
            is_ready: Mutex::new(false),
        }
    }
}

// Impl concrete interrupt
impl ConcreteInterrupt {
    // Setup
    pub fn setup(&mut self) {
        // Setupt exception
        self.exception.setup();

        // Set ready flag
        *self.is_ready.lock() = true;

        // Output debug info
        kernel().debug().info("Interrupt setup done!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear ready flag
        *self.is_ready.lock() = false;

        // Exit exception
        self.exception.exit();
    }
}

// Impl interrupt for concrete interrupt
impl Interrupt for ConcreteInterrupt {
     // Set ISR function callback
    fn set_isr_fn_cb(&mut self, irq: isize, func: FnCallback, args: *mut()) {
        let _ = args;
        let _ = func;
        let _ = irq;
    }

    // Set ISR method callback
    fn set_isr_meth_cb(&mut self, irq: isize, method: MethodCb, args: *mut ()) {
        let _ = args;
        let _ = method;
        let _ = irq;
    }

    // Add ISR function callback
    fn add_isr_fn_cb(&mut self, irq: isize, func: FnCallback, args: *mut()) {
        let _ = args;
        let _ = func;
        let _ = irq;
    }

    // Add ISR method callback
    fn add_isr_meth_cb(&mut self, irq: isize, method: MethodCb, args: *mut ()) {
        let _ = args;
        let _ = method;
        let _ = irq;
    }

    // Del ISR function callback
    fn del_isr_fn_cb(&mut self, irq: isize, func: FnCallback, args: *mut()) {
        let _ = args;
        let _ = func;
        let _ = irq;
    }

    // Del ISR method callback
    fn del_isr_meth_cb(&mut self, irq: isize, method: MethodCb, args: *mut ()) {
        let _ = args;
        let _ = method;
        let _ = irq;
    }
    
    // Clear ISR callbacks
    fn clear_isr_cb(&mut self, irq: isize) {
        let _ = irq;
    }
    
    // Replace ISR handler
    fn replace(&mut self, handler: usize) {
        let _ = handler;

    }
    
    // Feature Methods_irq
    fn handler(&mut self, irq: isize) {
        let _ = irq;

    }
}
