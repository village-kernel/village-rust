//###########################################################################
// vk_feature.rs
// The specific implementation of functions related to feature
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Feature;

// struct concrete feature
pub struct ConcreteFeature;

// impl concrete feature
impl ConcreteFeature {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete feature
impl ConcreteFeature {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Feature setup done!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl feature for concrete feature
impl Feature for ConcreteFeature {
    // register module
    fn register_module(&mut self) {

    }

    // unregister module
    fn unregister_module(&mut self) {

    }

    // get module
    fn get_module(&mut self, name: &str) {
        let _ = name;
    }
}
