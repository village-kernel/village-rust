//###########################################################################
// vk_dylib_builder.rs
// The specific implementation of functions related to prog builder
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_so_loader::SoLoader;
use crate::traits::vk_builder::{LibBuilder, LibLoader};
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
    fn create(&self, suffix: &str) -> Option<Box<dyn LibLoader>> {
        if suffix == ".so" {
            return Some(Box::new(SoLoader::new()))
        }
        None
    }
}

// Register lib builder
register_lib_builder!(DylibBuilder, dylib_builder);
