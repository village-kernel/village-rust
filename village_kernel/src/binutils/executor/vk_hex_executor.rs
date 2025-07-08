//###########################################################################
// vk_hex_executor.rs
// The specific implementation of functions related to hex executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_hex_loader::HexLoader;
use crate::binutils::executor::vk_prog_executor::ProgExecutor;
use crate::traits::vk_executor::{BaseExecutor, Executor};
use crate::register_executor;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;

// Struct HexExecutor
struct HexExecutor;

// Impl executor for hex executor
impl Executor for HexExecutor {
    // Get suffixes
    fn suffixes(&self) -> Vec<&str> {
        #[cfg(not(feature = "binding_exec_hex"))]
        return vec![".hex"];

        #[cfg(feature = "binding_exec_hex")]
        return vec![".hex", ".exec"];
    }

    // Create
    fn create(&self) -> Box<dyn BaseExecutor> {
        Box::new(ProgExecutor::new(Box::new(HexLoader::new())))
    }
}

// Register executor
register_executor!(HexExecutor, hex_executor);
