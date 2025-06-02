//###########################################################################
// vk_scheduler.rs
// The specific implementation of functions related to scheduler
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel::traits::vk_kernel::Scheduler;

/// struct concrete scheduler
pub struct ConcreteScheduler;

/// impl concrete scheduler
impl ConcreteScheduler {
    /// setup
    pub fn setup(&self) {

    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl scheduler for concrete scheduler
impl Scheduler for ConcreteScheduler {
    /// start
    fn start(&self) {

    }
    
    /// sched
    fn sched(&self) {

    }
}
