//###########################################################################
// vk_bin_executor.rs
// The specific implementation of functions related to bin executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::village::kernel;
use crate::register_exec_factory;
use crate::traits::vk_executor::{ExecFtyInfo, ExecInfo, Executor, ExecutorFty};
use crate::traits::vk_callback::Callback;
use super::vk_bin_loader::BinLoader;

// Sturct BinExecutor
struct BinExecutor {
    info: ExecInfo,
    bin: BinLoader,
}

// Impl BinExecutor
impl BinExecutor {
    // New
    const fn new() -> Self {
        Self {
            info: ExecInfo::new(),
            bin: BinLoader::new(),
        }
    }
}

// Impl BinExecutor
impl BinExecutor {
    // Sandbox
    fn sandbox(&mut self) {
        let argv = self.info.argv.iter_mut().map(|s| s.as_str()).collect();
        self.bin.execute(argv);
        self.bin.exit();
    }
}

// Impl executor for bin executor
impl Executor for BinExecutor {
    // Base
    fn base(&mut self) -> &mut ExecInfo {
        &mut self.info
    }

    // Initiate
    fn initiate(&mut self) -> i32 {
        // Load, parser and execute bin file
        if !self.bin.load(&self.info.path) {
            return -1;
        }

        // Create a sandbox thread to run the app
        let sandbox_cb = Callback::new(Self::sandbox as u32).with_instance(self);
        kernel().thread().create_task(&self.info.path, sandbox_cb)
    }

    // Release
    fn release(&mut self) -> bool {
        self.bin.exit()
    }
}

// Struct BinExecutorFty
struct BinExecutorFty {
    info: ExecFtyInfo,
}

// Impl BinExecutorFty
impl BinExecutorFty {
    const fn new() -> Self {
        Self {
            info: ExecFtyInfo::new(),
        }
    }
}

// Impl executor fty for bin executro fty
impl ExecutorFty for BinExecutorFty {
    // Info
    fn info(&mut self) -> &mut ExecFtyInfo {
        &mut self.info
    }

    // Get suffixes
    fn get_suffixes(&mut self) -> Vec<&str> {
        let mut suffixes = Vec::<&str>::new();
        suffixes.push(".bin");
        suffixes
    }

    // Create
    fn create(&mut self) -> Box<dyn Executor> {
        Box::new(BinExecutor::new())
    }
}

// Register executor factory
register_exec_factory!(BinExecutorFty::new(), bin_executor_fty);
