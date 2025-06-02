//###########################################################################
// vk_workqueue.rs
// The specific implementation of functions related to workqueue
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::WorkQueue;

/// struct concrete work queue
pub struct ConcreteWorkQueue;

/// impl concrete work queue
impl ConcreteWorkQueue {
    /// setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Work queue setup done!");
    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl work queue for concrete work queue
impl WorkQueue for ConcreteWorkQueue {
    /// create
    fn create(&self) {

    }

    /// delete
    fn delete(&self) {

    }

    /// sched
    fn sched(&self) {
        
    }
}
