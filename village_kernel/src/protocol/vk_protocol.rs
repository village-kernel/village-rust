//###########################################################################
// vk_protocol.rs
// The specific implementation of functions related to protocol
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::Protocol;
use crate::village::kernel;

// struct concrete protocol
pub struct ConcreteProtocol;

// impl concrete protocol
impl ConcreteProtocol {
    pub const fn new() -> Self {
        Self {}
    }
}

// impl concrete protocol
impl ConcreteProtocol {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Protocol setup completed!");
    }

    // exit
    pub fn exit(&mut self) {}
}

// impl protocol for concrete protocol
impl Protocol for ConcreteProtocol {
    // register stack
    fn register_stack(&mut self) {}

    // unregister stack
    fn unregister_stack(&mut self) {}
}
