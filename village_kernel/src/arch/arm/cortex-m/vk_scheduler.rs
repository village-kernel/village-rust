//###########################################################################
// vk_scheduler.rs
// The specific implementation of functions related to scheduler
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Scheduler;

/// struct village scheduler
pub struct VillageScheduler;

// impl village scheduler
impl VillageScheduler {    
    pub const fn new() -> Self {
        Self { }
    }
}

/// impl village scheduler
impl VillageScheduler {
    /// setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Scheduler setup done!");
    }

    /// exit
    pub fn exit(&mut self) {

    }
}

/// impl scheduler for village scheduler
impl Scheduler for VillageScheduler {
    /// start
    fn start(&mut self) {

    }
    
    /// sched
    fn sched(&mut self) {

    }
}
