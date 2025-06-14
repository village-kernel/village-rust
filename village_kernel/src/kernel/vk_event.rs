//###########################################################################
// vk_event.rs
// The specific implementation of functions related to event
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Event;

// struct concrete event
pub struct ConcreteEvent;

// impl concrete event
impl ConcreteEvent {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete event
impl ConcreteEvent {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Input event setup completed!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl event for concrete event
impl Event for ConcreteEvent {
    // init input device
    fn init_input_device(&mut self, input: &str) {
        let _ = input;
    }

    // exit input device
    fn exit_input_device(&mut self, input: &str) {
        let _ = input;
    }
}
