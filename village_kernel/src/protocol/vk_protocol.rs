//###########################################################################
// vk_protocol.rs
// The specific implementation of functions related to protocol
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::Protocol;
use crate::debug_info;

// struct village protocol
pub struct VillageProtocol;

// impl village protocol
impl VillageProtocol {
    pub const fn new() -> Self {
        Self {}
    }
}

// impl village protocol
impl VillageProtocol {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        debug_info!("Protocol setup completed!");
    }

    // exit
    pub fn exit(&mut self) {}
}

// impl protocol for village protocol
impl Protocol for VillageProtocol {
    // register stack
    fn register_stack(&mut self) {}

    // unregister stack
    fn unregister_stack(&mut self) {}
}
