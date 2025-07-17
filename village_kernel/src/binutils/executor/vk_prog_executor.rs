//###########################################################################
// vk_prog_executor.rs
// The specific implementation of functions related to prog executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_bin_loader::BinLoader;
use crate::binutils::loader::vk_hex_loader::HexLoader;
use crate::binutils::loader::vk_elf_loader::ElfLoader;
use crate::binutils::decoder::vk_prog_decode::ProgDecoder;
use crate::binutils::runner::vk_prog_runner::ProgRunner;
use crate::traits::vk_executor::{BaseRunner, BaseExecutor};
use crate::register_executor;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;

// Struct ProgExecutor
struct ProgExecutor;

// Impl executor for ProgExecutor
impl BaseExecutor for ProgExecutor {
    // Suffixes
    fn suffixes(&self) -> Vec<&str> {
        return vec![".bin", ".hex", ".elf", ".exec"];
    }

    // Create
    fn create(&self, mut suffix: &str) -> Option<Box<dyn BaseRunner>> {
        #[cfg(feature = "binding_exec_bin")]
        if suffix == ".exec" { suffix = ".bin"; }
        
        #[cfg(feature = "binding_exec_hex")]
        if suffix == ".exec" { suffix = ".hex"; }

        #[cfg(feature = "binding_exec_elf")]
        if suffix == ".exec" { suffix = ".elf"; }

        if suffix == ".bin" {
            let loader = Box::new(BinLoader::new());
            let decoder = Box::new(ProgDecoder::new());
            return Some(Box::new(ProgRunner::new(loader, decoder)))
        }
        
        else if suffix == ".hex" {
            let loader = Box::new(HexLoader::new());
            let decoder = Box::new(ProgDecoder::new());
            return Some(Box::new(ProgRunner::new(loader, decoder)))
        }

        else if suffix == ".elf" {
            let loader = Box::new(ElfLoader::new());
            let decoder = Box::new(ProgDecoder::new());
            return Some(Box::new(ProgRunner::new(loader, decoder)))
        }
        
        None
    }
}

// Register executor
register_executor!(ProgExecutor, prog_executor);
