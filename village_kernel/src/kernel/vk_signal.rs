//###########################################################################
// vk_signal.rs
// The specific implementation of functions related to signal
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::{Signal, Signals};
use crate::village::kernel;

// Struct village signal
pub struct VillageSignal;

// Impl village signal
impl VillageSignal {
    pub const fn new() -> Self {
        Self {}
    }
}

// Impl village signal
impl VillageSignal {
    // Setup
    pub fn setup(&mut self) {
        // output debug info
        kernel().debug().info("Signal setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {}
}

// Impl signal for village signal
impl Signal for VillageSignal {
    // Raising
    fn raising(&mut self, signal: Signals) {
        kernel().system().disable_irq();

        match signal {
            Signals::None => todo!(),
            Signals::Sleep => kernel().system().sleep(),
            Signals::Standby => kernel().system().standby(),
            Signals::Shutdown => kernel().system().shutdown(),
            Signals::Reboot => kernel().system().reboot(),
            Signals::Kill => todo!(),
        }

        kernel().system().enable_irq();
    }
}
