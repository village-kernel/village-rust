//###########################################################################
// vk_workqueue.rs
// The specific implementation of functions related to workqueue
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::WorkQueue;

// struct concrete work queue
pub struct ConcreteWorkQueue;

// impl concrete work queue
impl ConcreteWorkQueue {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete work queue
impl ConcreteWorkQueue {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Work queue setup done!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl work queue for concrete work queue
impl WorkQueue for ConcreteWorkQueue {
    // create
    fn create(&mut self) {

    }

    // delete
    fn delete(&mut self) {

    }

    // sched
    fn sched(&mut self) {
        
    }
}
