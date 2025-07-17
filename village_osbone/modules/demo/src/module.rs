//###########################################################################
// module.rs
// The specific implementation of functions related to module
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_extension;
use crate::traits::vk_kernel::DebugLevel;
use crate::village::kernel;
use crate::traits::vk_extension::{Extension, ExtensionID};

// Struct ModuleDemo
struct ModuleDemo;

// Impl Extension for ModuleDemo
impl Extension for ModuleDemo {
    // Setup
    fn setup(&mut self) {
        kernel().debug().output(DebugLevel::Lv2, "hello module demo");
    }

    // Exit
    fn exit(&mut self) {
        kernel().debug().output(DebugLevel::Lv2, "exit module demo");
    }
}

// Register extension
register_extension!(ModuleDemo, ExtensionID::Feature, module_demo);
