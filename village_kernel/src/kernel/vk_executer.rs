//###########################################################################
// vk_executer.rs
// The specific implementation of functions related to executer
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_executor::{BaseRunner, ExecutorWrapper};
use crate::traits::vk_kernel::Executer;
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;
use alloc::boxed::Box;

// Struct village executer
pub struct VillageExecuter {
    executors: LinkedList<ExecutorWrapper>,
}

// Impl village executer
impl VillageExecuter {
    pub const fn new() -> Self {
        Self {
            executors: LinkedList::new(),
        }
    }
}

// Impl village executer
impl VillageExecuter {
    // Setup
    pub fn setup(&mut self) {
        // Output debug info
        kernel().debug().info("Executer setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear executors
        self.executors.clear();
    }
}

// 
impl Executer for VillageExecuter {
    // Register executor
    fn register_executor(&mut self, executor: ExecutorWrapper) {
        self.executors.push(executor);
    }

    // Unregister executor
    fn unregister_executor(&mut self, name: &str) {
        self.executors
            .retain_mut(|executor| !(executor.name() == name));
    }

    // Create runner
    fn create_runner(&mut self, path: &str) -> Option<Box<dyn BaseRunner>> {
        let suffix = match path.rfind('.') {
            Some(pos) => &path[pos..],
            None => return None,
        };

        for executor in self.executors.iter_mut() {
            if executor.suffixes().contains(&suffix) {
                return executor.create(suffix);
            }
        }

        None
    }
}