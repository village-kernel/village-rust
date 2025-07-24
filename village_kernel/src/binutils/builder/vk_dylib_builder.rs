//###########################################################################
// vk_dylib_builder.rs
// The specific implementation of functions related to prog builder
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_elf_loader::ElfLoader;
use crate::binutils::decoder::vk_dylib_decode::DylibDecoder;
use crate::binutils::container::vk_dylib_container::DylibContainer;
use crate::traits::vk_builder::{LibBuilder, LibContainer};
use crate::register_lib_builder;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;

// Struct DylibBuilder
struct DylibBuilder;

// Impl LibBuilder for DylibBuilder
impl LibBuilder for DylibBuilder {
    // Suffixes
    fn suffixes(&self) -> Vec<&str> {
        return vec![".so"];
    }

    // Create
    fn create(&self, suffix: &str) -> Option<Box<dyn LibContainer>> {
        if suffix == ".so" {
            let loader = Box::new(ElfLoader::new());
            let decoder = Box::new(DylibDecoder::new());
            return Some(Box::new(DylibContainer::new(loader, decoder)))
        }
        None
    }
}

// Register lib builder
register_lib_builder!(DylibBuilder, dylib_builder);
