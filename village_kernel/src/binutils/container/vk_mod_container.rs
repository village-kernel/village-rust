//###########################################################################
// vk_mod_runner.rs
// The specific implementation of functions related to mod runner
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_builder::{ProgLoader, ProgDecoder, ProgContainer};
use crate::debug_error;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};

// Sturct ModRunner
pub struct ModRunner {
    loader: Box<dyn ProgLoader>,
    decoder: Box<dyn ProgDecoder>,
    path: String,
}

// Impl ModRunner
impl ModRunner {
    // New
    pub const fn new(loader: Box<dyn ProgLoader>, decoder: Box<dyn ProgDecoder>) -> Self {
        Self {
            loader,
            decoder,
            path: String::new(),
        }
    }
}

// Impl ExecRunner for ModRunner
impl ProgContainer for ModRunner {
    // Run
    fn run(&mut self, path: &str, _argv: Vec<&str>) -> i32 {
        // Set path and argv
        self.path = path.to_string();

        // New module data
        let mut data: Vec<u8> = Vec::new();

        // Load module data
        if !self.loader.init(&self.path, &mut data) {
            debug_error!("{} module load failed", self.path);
            return -1;
        }

        // Decoder module data
        if !self.decoder.init(&self.path, data) {
            debug_error!("{} module decode failed", self.path);
            return -1;
        }

        0
    }

    // Wait
    fn wait(&mut self) {
        
    }

    // Kill
    fn kill(&mut self) {
        self.decoder.exit();
    }
}
