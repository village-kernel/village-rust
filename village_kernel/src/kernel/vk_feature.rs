//###########################################################################
// vk_feature.rs
// The specific implementation of functions related to feature
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::boxed::Box;
use crate::village::kernel;
use crate::traits::vk_kernel::Feature;
use crate::traits::vk_module::{Module, ModuleID};
use crate::traits::vk_linkedlist::LinkedList;

// Struct concrete feature
pub struct ConcreteFeature {
    modules: LinkedList<Box<dyn Module>>,
    is_runtime: bool,
}

// Impl concrete feature
impl ConcreteFeature {
    // New
    pub const fn new() -> Self {
        Self {
            modules: LinkedList::new(),
            is_runtime: false
        }
    }
}

// Impl concrete feature
impl ConcreteFeature {
    // Setup
    pub fn setup(&mut self) {
        // Clear the runtime flag
        self.is_runtime = false;

        // Setup modules
        for id in ModuleID::iter() {
            for module in self.modules.iter_mut() {
                if module.data().get_id() == id {
                    module.setup();
                }
            }
        }

        // Set the runtime flag
        self.is_runtime = true;

        // Output debug info
        kernel().debug().info("Feature setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear the runtime flag
        self.is_runtime = false;

        // Setup modules
        for id in ModuleID::rev_iter() {
            for module in self.modules.iter_mut() {
                if module.data().get_id() == id {
                    module.exit();
                }
            }
        }

        // Clear modules
        self.modules.clear();
    }
}

// iImpl feature for concrete feature
impl Feature for ConcreteFeature {
    // Register module
    fn register_module(&mut self, mut module: Box<dyn Module>) {
        if self.is_runtime {
            module.setup();
        }
        self.modules.add(module);
    }

    // Unregister module
    fn unregister_module(&mut self, name: &str) {
        self.modules.retain_mut(|module| {
            if module.data().get_name() == name {
                if self.is_runtime {
                    module.exit();
                }
                false
            } else {
                true
            }
        });
    }

    // Get module
    fn get_module(&mut self, name: &str) -> Option<&mut Box<dyn Module>> {
        for module in self.modules.iter_mut() {
            if module.data().get_name() == name {
                return Some(module);
            }
        }
        None
    }
}

// Impl drop for concrete feature
impl Drop for ConcreteFeature {
    fn drop(&mut self) {
        self.exit();
    }
}
