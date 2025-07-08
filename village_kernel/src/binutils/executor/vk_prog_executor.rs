//###########################################################################
// vk_bin_executor.rs
// The specific implementation of functions related to bin executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_callback::Callback;
use crate::traits::vk_executor::{BaseLoader, BaseExecutor};
use crate::village::kernel;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};

// Sturct ProgExecutor
pub struct ProgExecutor {
    prog: Box<dyn BaseLoader>,
    path: String,
    argv: Vec<String>,
    tid: i32,
}

// Impl ProgExecutor
impl ProgExecutor {
    // New
    pub const fn new(prog: Box<dyn BaseLoader>) -> Self {
        Self {
            prog,
            path: String::new(),
            argv: Vec::new(),
            tid: 0,
        }
    }
}

// Impl ProgExecutor
impl ProgExecutor {
    // Sandbox
    fn sandbox(&mut self) {
        let argv = self.argv.iter_mut().map(|s| s.as_str()).collect();
        self.prog.exec(argv);
        self.prog.exit();
    }
}

// Impl base executor for prog executor
impl BaseExecutor for ProgExecutor {
    // Run
    fn run(&mut self, path: &str, argv: Vec<&str>) -> i32 {
        // Set path and argv
        self.path = path.to_string();
        self.argv = argv.into_iter().map(|s| s.to_string()).collect();

        // Load, parser and execute bin file
        if !self.prog.load(&self.path) {
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
        self.prog.exit();
    }
}
