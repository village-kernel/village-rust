//###########################################################################
// vk_signal.rs
// The specific implementation of functions related to signal
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel::traits::vk_kernel::Signal;

/// struct concrete signal
pub struct ConcreteSignal;

/// impl concrete signal
impl ConcreteSignal {
    /// setup
    pub fn setup(&self) {

    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl signal for concrete signal
impl Signal for ConcreteSignal {
    /// raising
    fn raising(&self, signal: i32) {
        
    }
}
