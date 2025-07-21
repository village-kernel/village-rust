//###########################################################################
// vk_prog_builder.rs
// The specific implementation of functions related to prog builder
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_bin_loader::BinLoader;
use crate::binutils::loader::vk_hex_loader::HexLoader;
use crate::binutils::loader::vk_elf_loader::ElfLoader;
use crate::binutils::decoder::vk_mod_decode::ModDecoder;
use crate::binutils::runner::vk_mod_runner::ModRunner;
use crate::traits::vk_builder::{ProgRunner, ProgBuilder};
use crate::register_prog_builder;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;

// Struct ModBuilder
struct ModBuilder;

// Impl ProgBuilder for ModBuilder
impl ProgBuilder for ModBuilder {
    // Suffixes
    fn suffixes(&self) -> Vec<&str> {
        return vec![".mbin", ".mhex", ".melf", ".mod"];
    }

    // Create
    fn create(&self, mut suffix: &str) -> Option<Box<dyn ProgRunner>> {
        #[cfg(feature = "binding_mod_mbin")]
        if suffix == ".mod" { suffix = ".mbin"; }
        
        #[cfg(feature = "binding_exec_hex")]
        if suffix == ".mod" { suffix = ".mhex"; }

        #[cfg(feature = "binding_exec_elf")]
        if suffix == ".mod" { suffix = ".melf"; }

        if suffix == ".mbin" {
            let loader = Box::new(BinLoader::new());
            let decoder = Box::new(ModDecoder::new());
            return Some(Box::new(ModRunner::new(loader, decoder)))
        }
        
        else if suffix == ".mhex" {
            let loader = Box::new(HexLoader::new());
            let decoder = Box::new(ModDecoder::new());
            return Some(Box::new(ModRunner::new(loader, decoder)))
        }

        else if suffix == ".melf" {
            let loader = Box::new(ElfLoader::new());
            let decoder = Box::new(ModDecoder::new());
            return Some(Box::new(ModRunner::new(loader, decoder)))
        }
        
        None
    }
}

// Register executor
register_prog_builder!(ModBuilder, mod_builder);
