//###########################################################################
// vk_timer.rs
// The specific implementation of functions related to timer
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Timer;

// struct concrete timer
pub struct ConcreteTimer;

// impl concrete timer
impl ConcreteTimer {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete timer
impl ConcreteTimer {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Timer setup done!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl timer for concrete timer
impl Timer for ConcreteTimer {
    // create
    fn create(&mut self) {

    }

    // modify
    fn modify(&mut self) {

    }

    // delete
    fn delete(&mut self) {
        
    }
}
