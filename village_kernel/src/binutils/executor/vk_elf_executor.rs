//###########################################################################
// vk_elf_executor.rs
// The specific implementation of functions related to elf executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_elf_loader::ElfLoader;
use crate::binutils::executor::vk_prog_executor::ProgExecutor;
use crate::traits::vk_executor::{BaseExecutor, Executor};
use crate::register_executor;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;

// Struct ElfExecutor
struct ElfExecutor;

// Impl executor for elf executro
impl Executor for ElfExecutor {
    // Get suffixes
    fn suffixes(&self) -> Vec<&str> {
        #[cfg(not(feature = "binding_exec_elf"))]
        return vec![".elf"];

        #[cfg(feature = "binding_exec_elf")]
        return vec![".elf", ".exec"];
    }

    // Create
    fn create(&self) -> Box<dyn BaseExecutor> {
        Box::new(ProgExecutor::new(Box::new(ElfLoader::new())))
    }
}

// Register executor
register_executor!(ElfExecutor, elf_executor);
