//###########################################################################
// vk_mod_runner.rs
// The specific implementation of functions related to mod runner
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_executor::{BaseLoader, BaseDecoder, BaseRunner};
use crate::village::kernel;
use alloc::format;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};

// Sturct ModRunner
pub struct ModRunner {
    loader: Box<dyn BaseLoader>,
    decoder: Box<dyn BaseDecoder>,
    path: String,
}

// Impl ModRunner
impl ModRunner {
    // New
    pub const fn new(loader: Box<dyn BaseLoader>, decoder: Box<dyn BaseDecoder>) -> Self {
        Self {
            loader,
            decoder,
            path: String::new(),
        }
    }
}

// Impl base runner for mod runner
impl BaseRunner for ModRunner {
    // Run
    fn run(&mut self, path: &str, _argv: Vec<&str>) -> i32 {
        // Set path and argv
        self.path = path.to_string();

        // New module data
        let mut data: Vec<u8> = Vec::new();

        // Load module data
        if !self.loader.init(&self.path, &mut data) {
            kernel().debug().error(&format!("{} module load failed", self.path));
            return -1;
        }

        // Decoder module data
        if !self.decoder.init(data) {
            kernel().debug().error(&format!("{} module decode failed", self.path));
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
