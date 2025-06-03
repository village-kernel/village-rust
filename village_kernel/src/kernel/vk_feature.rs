//###########################################################################
// vk_feature.rs
// The specific implementation of functions related to feature
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Feature;

// struct concrete feature
pub struct ConcreteFeature;

// impl concrete feature
impl ConcreteFeature {
    // setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Feature setup done!");
    }

    // exit
    pub fn exit(&self) {

    }
}

// impl feature for concrete feature
impl Feature for ConcreteFeature {
    // register module
    fn register_module(&self) {

    }

    // unregister module
    fn unregister_module(&self) {

    }

    // get module
    fn get_module(&self, name: &str) {
        
    }
}
