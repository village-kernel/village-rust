//###########################################################################
// vk_extender.rs
// The specific implementation of functions related to extender
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::Extender;
use crate::traits::vk_linkedlist::LinkedList;
use crate::traits::vk_extension::{ExtensionID, ExtensionWrapper};
use crate::village::kernel;

// Struct village extender
pub struct VillageExtender {
    extensions: LinkedList<ExtensionWrapper>,
    is_runtime: bool,
}

// Impl village extender
impl VillageExtender {
    // New
    pub const fn new() -> Self {
        Self {
            extensions: LinkedList::new(),
            is_runtime: false,
        }
    }
}

// Impl village extender
impl VillageExtender {
    // Setup
    pub fn setup(&mut self) {
        // Clear the runtime flag
        self.is_runtime = false;

        // Setup extensions
        for id in ExtensionID::iter() {
            for extension in self.extensions.iter_mut() {
                if extension.id() == id {
                    extension.setup();
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

        // Exit extensions
        for id in ExtensionID::rev_iter() {
            for extension in self.extensions.iter_mut() {
                if extension.id() == id {
                    extension.exit();
                }
            }
        }

        // Clear extensions
        self.extensions.clear();
    }
}

// iImpl extender for village extender
impl Extender for VillageExtender {
    // Register extension
    fn register_extension(&mut self, mut extension: ExtensionWrapper) {
        if self.is_runtime {
            extension.setup();
        }
        self.extensions.push(extension);
    }

    // Unregister extension
    fn unregister_extension(&mut self, name: &str) {
        self.extensions.retain_mut(|extension| {
            if extension.name() == name {
                if self.is_runtime {
                    extension.exit();
                }
                false
            } else {
                true
            }
        });
    }

    // Get extension
    fn get_extension(&mut self, name: &str) -> Option<&mut ExtensionWrapper> {
        for extension in self.extensions.iter_mut() {
            if extension.name() == name {
                return Some(extension);
            }
        }
        None
    }
}

// Impl drop for village extender
impl Drop for VillageExtender {
    fn drop(&mut self) {
        self.exit();
    }
}
