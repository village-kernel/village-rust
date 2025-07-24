//###########################################################################
// vk_dylib_container.rs
// The specific implementation of functions related to dylib container
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_builder::{LibLoader, LibDecoder, LibContainer};
use crate::village::kernel;
use alloc::format;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};

// Sturct DylibContainer
pub struct DylibContainer {
    loader: Box<dyn LibLoader>,
    decoder: Box<dyn LibDecoder>,
    path: String,
}

// Impl DylibContainer
impl DylibContainer {
    // New
    pub const fn new(loader: Box<dyn LibLoader>, decoder: Box<dyn LibDecoder>) -> Self {
        Self {
            loader,
            decoder,
            path: String::new(),
        }
    }
}

// Impl LibContainer for DylibContainer
impl LibContainer for DylibContainer {
    // Run
    fn init(&mut self, path: &str) -> bool {
        // Set path and argv
        self.path = path.to_string();

        // New library data
        let mut prog: Vec<u8> = Vec::new();

        // Load library data
        if !self.loader.init(&self.path, &mut prog) {
            kernel().debug().error(&format!("{} library load failed", self.path));
            return false;
        }

        // Decoder library data
        if !self.decoder.init(&self.path, prog) {
            kernel().debug().error(&format!("{} library decode failed", self.path));
            return false;
        }

        true
    }

    // Get symbol
    fn get(&mut self, symbol: &str) -> usize {
        self.decoder.get(symbol)
    }

    // Exit
    fn exit(&mut self) -> bool {
        self.loader.exit();
        self.decoder.exit();
        true
    }
}
