//###########################################################################
// vk_interrupt.rs
// The specific implementation of functions related to interrupt
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::format;
use crate::village::kernel;
use crate::traits::vk_kernel::Interrupt;
use crate::traits::vk_callback::Callback;
use crate::traits::vk_linkedlist::LinkedList;
use crate::arch::ia32::legacy::vk_exception::{ConcreteException, ISR_NUM, RSVD_ISR_SIZE};

// Struct concrete interrupt
pub struct ConcreteInterrupt {
    exception: ConcreteException,
    warnings: [u8; ISR_NUM],
    isr_tabs: [LinkedList<Callback>; ISR_NUM],
    is_ready: bool,
}

// Impl concrete interrupt
impl ConcreteInterrupt {
    pub const fn new() -> Self {
        Self {
            exception: ConcreteException::new(),
            warnings: [0; ISR_NUM],
            isr_tabs: [const { LinkedList::new() }; ISR_NUM],
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
        kernel().debug().info("Interrupt setup completed!");
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
    fn set_isr_cb(&mut self, irq: isize, callback: Callback) {
        self.clear_isr_cb(irq);
        self.add_isr_cb(irq, callback);
    }

    // Add ISR function callback
    fn add_isr_cb(&mut self, irq: isize, callback: Callback) {
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        self.isr_tabs[irq_idx].push(callback);
    }

    // Del ISR function callback
    fn del_isr_cb(&mut self, irq: isize, callback: Callback) {
        let irq_idx = (irq + RSVD_ISR_SIZE as isize) as usize;
        let isrs = &mut self.isr_tabs[irq_idx];
        isrs.retain_mut(|cb|{
            !(cb.instance == callback.instance && core::ptr::fn_addr_eq(cb.callback , callback.callback))
        });
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
        
        for callback in isrs.iter_mut() {
            callback.call();
        }
    }
}
