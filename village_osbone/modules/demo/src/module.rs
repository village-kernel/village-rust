//###########################################################################
// module.rs
// The specific implementation of functions related to module
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_module;
use crate::traits::vk_kernel::DebugLevel;
use crate::village::kernel;
use crate::traits::vk_module::{Module, ModuleID};

// Struct ModuleDemo
struct ModuleDemo;

// Module for ModuleDemo
impl Module for ModuleDemo {
    // Setup
    fn setup(&mut self) {
        kernel().debug().output(DebugLevel::Lv2, "hello module demo");
    }

    // Exit
    fn exit(&mut self) {
        kernel().debug().output(DebugLevel::Lv2, "exit module demo");
    }
}

// Register module
register_module!(ModuleDemo, ModuleID::Feature, module_demo);
