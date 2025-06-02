//###########################################################################
// vk_process.rs
// The specific implementation of functions related to process
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Process;

/// struct concrete process
pub struct ConcreteProcess;

/// impl concrete process
impl ConcreteProcess {
    /// setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Process setup done!");
    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl process for concrete process
impl Process for ConcreteProcess {
    /// register executor
    fn register_executor(&self) {

    }
    
    /// unregister executor
    fn unregister_executor(&self) {

    }
}
