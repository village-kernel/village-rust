//###########################################################################
// vk_exec_runner.rs
// The specific implementation of functions related to exec runner
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_callback::Callback;
use crate::traits::vk_builder::{ProgLoader, ProgDecoder, ProgContainer};
use crate::village::kernel;
use crate::debug_error;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};

// Sturct ProgRunner
pub struct ExecRunner {
    loader: Box<dyn ProgLoader>,
    decoder: Box<dyn ProgDecoder>,
    path: String,
    argv: Vec<String>,
    tid: i32,
}

// Impl ExecRunner
impl ExecRunner {
    // New
    pub const fn new(loader: Box<dyn ProgLoader>, decoder: Box<dyn ProgDecoder>) -> Self {
        Self {
            loader,
            decoder,
            path: String::new(),
            argv: Vec::new(),
            tid: 0,
        }
    }
}

// Impl ExecRunner
impl ExecRunner {
    // Sandbox
    fn sandbox(&mut self) {
        let argv = self.argv.iter_mut().map(|s| s.as_str()).collect();
        self.decoder.exec(argv);
        self.decoder.exit();
    }
}

// Impl ProgRunner for ExecRunner
impl ProgContainer for ExecRunner {
    // Run
    fn run(&mut self, path: &str, argv: Vec<&str>) -> i32 {
        // Set path and argv
        self.path = path.to_string();
        self.argv = argv.into_iter().map(|s| s.to_string()).collect();

        // New program data
        let mut data: Vec<u8> = Vec::new();

        // Load program data
        if !self.loader.init(&self.path, &mut data) {
            debug_error!("{} program load failed", self.path);
            return -1;
        }

        // Decoder program data
        if !self.decoder.init(&self.path, data) {
            debug_error!("{} program decode failed", self.path);
            return -1;
        }

        // Create a sandbox thread to run the app
        let sandbox_cb = Callback::new(Self::sandbox as u32).with_instance(self);
        self.tid = kernel().thread().create_task(&self.path, sandbox_cb);

        // Start task
        kernel().thread().start_task(self.tid);

        self.tid
    }

    // Wait
    fn wait(&mut self) {
        kernel().thread().wait_for_task(self.tid);
    }

    // Kill
    fn kill(&mut self) {
        kernel().thread().stop_task(self.tid);
        self.decoder.exit();
    }
}
