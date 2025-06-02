//###########################################################################
// vk_interrupt.rs
// The specific implementation of functions related to interrupt
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel::traits::vk_kernel::Interrupt;

/// struct concrete interrupt
pub struct ConcreteInterrupt;

/// impl concrete interrupt
impl ConcreteInterrupt {
    /// setup
    pub fn setup(&self) {

    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl interrupt for concrete interrupt
impl Interrupt for ConcreteInterrupt {
    /// set isr
    fn set_isr(&self) {

    }

    /// append isr
    fn append_isr(&self) {

    }

    /// remove isr
    fn remove_isr(&self) {

    }

    /// clear isr
    fn clear_isr(&self) {

    }

    /// replace
    fn replace(&self) {

    }

    /// handler
    fn handler(&self) {

    }
}
