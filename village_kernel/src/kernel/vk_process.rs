//###########################################################################
// vk_process.rs
// The specific implementation of functions related to process
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Process;

// struct concrete process
pub struct ConcreteProcess;

// impl concrete process
impl ConcreteProcess {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete process
impl ConcreteProcess {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Process setup done!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl process for concrete process
impl Process for ConcreteProcess {
    // register executor
    fn register_executor(&mut self) {

    }
    
    // unregister executor
    fn unregister_executor(&mut self) {

    }
}
