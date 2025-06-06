//###########################################################################
// vk_signal.rs
// The specific implementation of functions related to signal
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Signal;

// struct concrete signal
pub struct ConcreteSignal;

// impl concrete signal
impl ConcreteSignal {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete signal
impl ConcreteSignal {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Signal setup done!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl signal for concrete signal
impl Signal for ConcreteSignal {
    // raising
    fn raising(&mut self, signal: i32) {
        let _ = signal;
    }
}
