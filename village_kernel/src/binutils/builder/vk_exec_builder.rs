//###########################################################################
// vk_exec_builder.rs
// The specific implementation of functions related to exec builder
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_bin_loader::BinLoader;
use crate::binutils::loader::vk_hex_loader::HexLoader;
use crate::binutils::loader::vk_elf_loader::ElfLoader;
use crate::binutils::decoder::vk_exec_decode::ExecDecoder;
use crate::binutils::container::vk_exec_container::ExecRunner;
use crate::traits::vk_builder::{ProgContainer, ProgBuilder};
use crate::register_prog_builder;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;

// Struct ExecBuilder
struct ExecBuilder;

// Impl ProgBuilder for ExecBuilder
impl ProgBuilder for ExecBuilder {
    // Suffixes
    fn suffixes(&self) -> Vec<&str> {
        return vec![".bin", ".hex", ".elf", ".exec"];
    }

    // Create
    fn create(&self, mut suffix: &str) -> Option<Box<dyn ProgContainer>> {
        #[cfg(feature = "binding_exec_bin")]
        if suffix == ".exec" { suffix = ".bin"; }
        
        #[cfg(feature = "binding_exec_hex")]
        if suffix == ".exec" { suffix = ".hex"; }

        #[cfg(feature = "binding_exec_elf")]
        if suffix == ".exec" { suffix = ".elf"; }

        if suffix == ".bin" {
            let loader = Box::new(BinLoader::new());
            let decoder = Box::new(ExecDecoder::new());
            return Some(Box::new(ExecRunner::new(loader, decoder)))
        }
        
        else if suffix == ".hex" {
            let loader = Box::new(HexLoader::new());
            let decoder = Box::new(ExecDecoder::new());
            return Some(Box::new(ExecRunner::new(loader, decoder)))
        }

        else if suffix == ".elf" {
            let loader = Box::new(ElfLoader::new());
            let decoder = Box::new(ExecDecoder::new());
            return Some(Box::new(ExecRunner::new(loader, decoder)))
        }
        
        None
    }
}

// Register executor
register_prog_builder!(ExecBuilder, exec_builder);
