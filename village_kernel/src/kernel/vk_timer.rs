//###########################################################################
// vk_timer.rs
// The specific implementation of functions related to timer
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Timer;

// struct concrete timer
pub struct ConcreteTimer;

// impl concrete timer
impl ConcreteTimer {
    // setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Timer setup done!");
    }

    // exit
    pub fn exit(&self) {

    }
}

// impl timer for concrete timer
impl Timer for ConcreteTimer {
    // create
    fn create(&self) {

    }

    // modify
    fn modify(&self) {

    }

    // delete
    fn delete(&self) {
        
    }
}
