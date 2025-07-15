//###########################################################################
// module.rs
// The specific implementation of functions related to module
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_module;
use crate::village::kernel;
use crate::traits::vk_module::{Module, ModuleID};

// Struct ModuleDemo
struct ModuleDemo;

// Module for ModuleDemo
impl Module for ModuleDemo {
    // Setup
    fn setup(&mut self) {
        kernel().debug().info("hello module demo");
    }

    // Exit
    fn exit(&mut self) {
        kernel().debug().info("exit module demo");
    }
}

// Register module
register_module!(ModuleDemo, ModuleID::Feature, module_demo);
