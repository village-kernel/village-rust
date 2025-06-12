//###########################################################################
// vk_signal.rs
// The specific implementation of functions related to signal
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::{Signal, Signals};

// Struct concrete signal
pub struct ConcreteSignal;

// Impl concrete signal
impl ConcreteSignal {
    pub const fn new() -> Self {
        Self { }
    }
}

// Impl concrete signal
impl ConcreteSignal {
    // Setup
    pub fn setup(&mut self) {
        // output debug info
        kernel().debug().info("Signal setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {

    }
}

// Impl signal for concrete signal
impl Signal for ConcreteSignal {
    // Raising
    fn raising(&mut self, signal: Signals) {
        kernel().system().disable_irq();

        match signal {
            Signals::None => todo!(),
            Signals::Sleep => kernel().sleep(),
            Signals::Standby => kernel().standby(),
            Signals::Shutdown => kernel().shutdown(),
            Signals::Reboot => kernel().reboot(),
            Signals::Kill => todo!(),
        }

        kernel().system().enable_irq();
    }
}
