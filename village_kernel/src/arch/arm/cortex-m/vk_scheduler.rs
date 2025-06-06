//###########################################################################
// vk_scheduler.rs
// The specific implementation of functions related to scheduler
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Scheduler;

/// struct concrete scheduler
pub struct ConcreteScheduler;

// impl concrete scheduler
impl ConcreteScheduler {    
    pub const fn new() -> Self {
        Self { }
    }
}

/// impl concrete scheduler
impl ConcreteScheduler {
    /// setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Scheduler setup done!");
    }

    /// exit
    pub fn exit(&mut self) {

    }
}

/// impl scheduler for concrete scheduler
impl Scheduler for ConcreteScheduler {
    /// start
    fn start(&mut self) {

    }
    
    /// sched
    fn sched(&mut self) {

    }
}
