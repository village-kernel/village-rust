//###########################################################################
// vk_memory.rs
// The specific implementation of functions related to memory
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Memory;

/// sturct concrete memory
pub struct ConcreteMemory;

/// impl concrete memory
impl ConcreteMemory {
    /// setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Memory setup done!");
    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl memory for concrete memory
impl Memory for ConcreteMemory {
    /// heap alloc
    fn heap_alloc(&self, size: u32) -> u32 {
        0
    }
    
    /// stack alloc
    fn stack_alloc(&self, size: u32) -> u32 {
        0
    }

    /// free
    fn free(&self, address: u32, size: u32) {
        
    }

    /// get size
    fn get_size(&self) -> u32 {
        0
    }

    /// get used
    fn get_used(&self) -> u32 {
        0
    }

    /// get curr addr
    fn get_curr_addr(&self) -> u32 {
        0
    }
}
