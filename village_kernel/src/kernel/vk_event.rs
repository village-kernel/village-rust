//###########################################################################
// vk_event.rs
// The specific implementation of functions related to event
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Event;

/// struct concrete event
pub struct ConcreteEvent;

/// impl concrete event
impl ConcreteEvent {
    /// setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Input event setup done!");
    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl event for concrete event
impl Event for ConcreteEvent {
    /// init input device
    fn init_input_device(&self) {

    }

    /// exit input device
    fn exit_input_device(&self) {

    }
}
