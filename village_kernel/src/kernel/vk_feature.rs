//###########################################################################
// vk_feature.rs
// The specific implementation of functions related to feature
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::Feature;
use crate::traits::vk_linkedlist::LinkedList;
use crate::traits::vk_module::{ModuleID, ModuleWrapper};
use crate::village::kernel;

// Struct village feature
pub struct VillageFeature {
    modules: LinkedList<ModuleWrapper>,
    is_runtime: bool,
}

// Impl village feature
impl VillageFeature {
    // New
    pub const fn new() -> Self {
        Self {
            modules: LinkedList::new(),
            is_runtime: false,
        }
    }
}

// Impl village feature
impl VillageFeature {
    // Setup
    pub fn setup(&mut self) {
        // Clear the runtime flag
        self.is_runtime = false;

        // Setup modules
        for id in ModuleID::iter() {
            for module in self.modules.iter_mut() {
                if module.id() == id {
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
                if module.id() == id {
                    module.exit();
                }
            }
        }

        // Clear modules
        self.modules.clear();
    }
}

// iImpl feature for village feature
impl Feature for VillageFeature {
    // Register module
    fn register_module(&mut self, mut module: ModuleWrapper) {
        if self.is_runtime {
            module.setup();
        }
        self.modules.push(module);
    }

    // Unregister module
    fn unregister_module(&mut self, name: &str) {
        self.modules.retain_mut(|module| {
            if module.name() == name {
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
    fn get_module(&mut self, name: &str) -> Option<&mut ModuleWrapper> {
        for module in self.modules.iter_mut() {
            if module.name() == name {
                return Some(module);
            }
        }
        None
    }
}

// Impl drop for village feature
impl Drop for VillageFeature {
    fn drop(&mut self) {
        self.exit();
    }
}
