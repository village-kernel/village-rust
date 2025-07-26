//###########################################################################
// vk_director.rs
// The specific implementation of functions related to director
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_builder::{ProgContainer, ProgBuilderWrapper, LibContainer, LibBuilderWrapper};
use crate::traits::vk_kernel::Director;
use crate::traits::vk_linkedlist::LinkedList;
use crate::debug_info;
use alloc::boxed::Box;

// Struct village director
pub struct VillageDirector {
    lib_builders: LinkedList<LibBuilderWrapper>,
    prog_builders: LinkedList<ProgBuilderWrapper>,
}

// Impl village director
impl VillageDirector {
    pub const fn new() -> Self {
        Self {
            lib_builders: LinkedList::new(),
            prog_builders: LinkedList::new(),
        }
    }
}

// Impl village director
impl VillageDirector {
    // Setup
    pub fn setup(&mut self) {
        // Output debug info
        debug_info!("Director setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear library builder
        self.lib_builders.clear();

        // Clear program builder
        self.prog_builders.clear();
    }
}

// Impl executor for village director
impl Director for VillageDirector {
    // Register lib builder
    fn register_lib_builder(&mut self, builder: LibBuilderWrapper) {
        self.lib_builders.push(builder);
    }

    // Unregister lib builder
    fn unregister_lib_builder(&mut self, name: &str) {
        self.lib_builders
            .retain_mut(|builder| !(builder.name() == name));
    }

    // Register prog builder
    fn register_prog_builder(&mut self, builder: ProgBuilderWrapper) {
        self.prog_builders.push(builder);
    }

    // Unregister prog builder
    fn unregister_prog_builder(&mut self, name: &str) {
        self.prog_builders
            .retain_mut(|builder| !(builder.name() == name));
    }

    // Create loader
    fn create_lib_container(&mut self, path: &str) -> Option<Box<dyn LibContainer>> {
        let suffix = match path.rfind('.') {
            Some(pos) => &path[pos..],
            None => return None,
        };

        for builder in self.lib_builders.iter_mut() {
            if builder.suffixes().contains(&suffix) {
                return builder.create(suffix);
            }
        }

        None
    }

    // Create runner
    fn create_prog_container(&mut self, path: &str) -> Option<Box<dyn ProgContainer>> {
        let suffix = match path.rfind('.') {
            Some(pos) => &path[pos..],
            None => return None,
        };

        for builder in self.prog_builders.iter_mut() {
            if builder.suffixes().contains(&suffix) {
                return builder.create(suffix);
            }
        }

        None
    }
}
