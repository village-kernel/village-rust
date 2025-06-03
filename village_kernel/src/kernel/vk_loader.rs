
//###########################################################################
// vk_loader.rs
// The specific implementation of functions related to loader
//
// $Copyright: Copyright (C) village
//###########################################################################use crate::kernel::traits::vk_kernel::Loader;
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Loader;

// struct concrete loader
pub struct ConcreteLoader;

// impl concrete loader
impl ConcreteLoader {
    // setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Loader setup done!");
    }

    // exit
    pub fn exit(&self) {

    }
}

// impl loader for concrete loader
impl Loader for ConcreteLoader {
    // load
    fn load(&self) {

    }

    // unload
    fn unload(&self) {
        
    }
}
