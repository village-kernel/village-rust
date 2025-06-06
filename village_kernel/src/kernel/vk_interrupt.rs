//###########################################################################
// vk_interrupt.rs
// The specific implementation of functions related to interrupt
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::format;
use alloc::vec::Vec;
use crate::village::kernel;
use crate::traits::vk_kernel::Interrupt;
use crate::traits::vk_callback::{CbInvoker, FnCallback, MethodCb};
use crate::arch::ia32::legacy::vk_exception::{ConcreteException, ISR_NUM, RSVD_ISR_SIZE};

// Struct concrete interrupt
pub struct ConcreteInterrupt {
    exception: ConcreteException,
    warnings: [u8; ISR_NUM],
    isr_tabs: [Vec<CbInvoker>; ISR_NUM],
    is_ready: bool,
}

// Impl concrete interrupt
impl ConcreteInterrupt {
    pub const fn new() -> Self {
        Self {
            exception: ConcreteException::new(),
            warnings: [0; ISR_NUM],
            isr_tabs: [const { Vec::new() }; ISR_NUM],
            is_ready: false,
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
        self.is_ready = true;

        // Output debug info
        kernel().debug().info("Interrupt setup done!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear ready flag
        self.is_ready = false;

        // Clear isr table
        for tab in &mut self.isr_tabs {
            tab.clear();
        }

        // Exit exception
        self.exception.exit();
    }
}

// Impl interrupt for concrete interrupt
impl Interrupt for ConcreteInterrupt {
    // Set ISR function callback
    fn set_isr_fn_cb(&mut self, irq: isize, func: FnCallback) {
        self.clear_isr_cb(irq);
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let mut invoker = CbInvoker::new();
        invoker.register_fn(func);
        self.isr_tabs[irq_idx].push(invoker);
    }

    // Set ISR method callback
    fn set_isr_meth_cb(&mut self, irq: isize, method: MethodCb) {
        self.clear_isr_cb(irq);
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let mut invoker = CbInvoker::new();
        invoker.register_method(method);
        self.isr_tabs[irq_idx].push(invoker);
    }

    // Set ISR function callback with data
    fn set_isr_fn_cb_with_data(&mut self, irq: isize, func: FnCallback, args: *mut()) {
        self.clear_isr_cb(irq);
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let mut invoker = CbInvoker::new();
        invoker.register_fn_with_data(func, args);
        self.isr_tabs[irq_idx].push(invoker);
    }

    // Set ISR method callback
    fn set_isr_meth_cb_with_data(&mut self, irq: isize, method: MethodCb, args: *mut ()) {
        self.clear_isr_cb(irq);
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let mut invoker = CbInvoker::new();
        invoker.register_method_with_data(method, args);
        self.isr_tabs[irq_idx].push(invoker);
    }

    // Add ISR function callback
    fn add_isr_fn_cb(&mut self, irq: isize, func: FnCallback) {
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let mut invoker = CbInvoker::new();
        invoker.register_fn(func);
        self.isr_tabs[irq_idx].push(invoker);
    }

    // Add ISR method callback
    fn add_isr_meth_cb(&mut self, irq: isize, method: MethodCb) {
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let mut invoker = CbInvoker::new();
        invoker.register_method(method);
        self.isr_tabs[irq_idx].push(invoker);
    }

    // Add ISR function callback with data
    fn add_isr_fn_cb_with_data(&mut self, irq: isize, func: FnCallback, args: *mut()) {
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let mut invoker = CbInvoker::new();
        invoker.register_fn_with_data(func, args);
        self.isr_tabs[irq_idx].push(invoker);
    }

    // Add ISR method callback with data
    fn add_isr_meth_cb_with_data(&mut self, irq: isize, method: MethodCb, args: *mut ()) {
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let mut invoker = CbInvoker::new();
        invoker.register_method_with_data(method, args);
        self.isr_tabs[irq_idx].push(invoker);
    }

    // Del ISR function callback
    fn del_isr_fn_cb(&mut self, irq: isize, func: FnCallback) {
        let _ = irq;
        let _ = func;
    }

    // Del ISR method callback
    fn del_isr_meth_cb(&mut self, irq: isize, method: MethodCb) {
        let _ = irq;
        let _ = method;
    }
    
    // Clear ISR callbacks
    fn clear_isr_cb(&mut self, irq: isize) {
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        self.isr_tabs[irq_idx].clear();
    }
    
    // Replace ISR handler
    fn replace(&mut self, irq: isize, handler: usize) {
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        self.exception.install(irq_idx, handler);
    }
    
    // Interrupt handler
    fn handler(&mut self, irq: isize) {
        if !self.is_ready { return; }
        
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let isrs = &mut self.isr_tabs[irq_idx];
        
        if isrs.is_empty() {
            if self.warnings[irq_idx] >= 10 {
                kernel().debug().error(&format!("IRQ {} no being handled correctly, system will halt on here", irq));
                loop {}
            }
            kernel().debug().warn(&format!("IRQ {} has no interrupt service function", irq));
            self.warnings[irq_idx] += 1;
            return;
        } else {
            self.warnings[irq_idx] = 0;
        }
        
        for invoker in isrs.iter_mut() {
            invoker.invoke();
        }
    }
}
