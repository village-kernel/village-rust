//###########################################################################
// vk_interrupt.rs
// The specific implementation of functions related to interrupt
//
// $Copyright: Copyright (C) village
//###########################################################################
use spin::Mutex;
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Interrupt;
use crate::arch::ia32::legacy::vk_exception::ConcreteException;

// struct concrete interrupt
pub struct ConcreteInterrupt {
    is_ready: Mutex<bool>,
    exception: ConcreteException
}

// impl concrete interrupt
impl ConcreteInterrupt {
    pub const fn new() -> Self {
        Self {
            is_ready: Mutex::new(false),
            exception: ConcreteException,
        }
    }
}

// impl concrete interrupt
impl ConcreteInterrupt {
    // setup
    pub fn setup(&self) {
        //setupt exception
        self.exception.setup();

        //set ready flag
        *self.is_ready.lock() = true;

        //output debug info
        kernel().debug().info("Interrupt setup done!");
    }

    // exit
    pub fn exit(&self) {
        // clear ready flag
        *self.is_ready.lock() = false;

        //exit exception
        self.exception.exit();
    }
}

// impl interrupt for concrete interrupt
impl Interrupt for ConcreteInterrupt {
    // set isr
    fn set_isr(&self) {

    }

    // append isr
    fn append_isr(&self) {

    }

    // remove isr
    fn remove_isr(&self) {

    }

    // clear isr
    fn clear_isr(&self) {

    }

    // replace
    fn replace(&self) {

    }

    // handler
    fn handler(&self) {

    }
}
