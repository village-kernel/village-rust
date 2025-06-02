//###########################################################################
// vk_protocol.rs
// The specific implementation of functions related to protocol
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Protocol;

/// struct concrete protocol
pub struct ConcreteProtocol;

/// impl concrete protocol
impl ConcreteProtocol {
    /// setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Protocol setup done!");
    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl protocol for concrete protocol
impl Protocol for ConcreteProtocol {
    /// register stack
    fn register_stack(&self) {

    }

    /// unregister stack
    fn unregister_stack(&self) {
        
    }
}
