//###########################################################################
// vk_bin_executor.rs
// The specific implementation of functions related to bin executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_bin_loader::BinLoader;
use crate::binutils::executor::vk_prog_executor::ProgExecutor;
use crate::traits::vk_executor::{BaseExecutor, Executor};
use crate::register_executor;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;

// Struct BinExecutor
struct BinExecutor;

// Impl executor for bin executor
impl Executor for BinExecutor {
    // Suffixes
    fn suffixes(&self) -> Vec<&str> {
        #[cfg(not(feature = "binding_exec_bin"))]
        return vec![".bin"];

        #[cfg(feature = "binding_exec_bin")]
        return vec![".bin", ".exec"];
    }

    // Create
    fn create(&self) -> Box<dyn BaseExecutor> {
        Box::new(ProgExecutor::new(Box::new(BinLoader::new())))
    }
}

// Register executor
register_executor!(BinExecutor, bin_executor);
