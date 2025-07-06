//###########################################################################
// vk_hex_executor.rs
// The specific implementation of functions related to hex executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_hex_loader::HexLoader;
use crate::register_exec_factory;
use crate::traits::vk_callback::Callback;
use crate::traits::vk_executor::{ExecFtyInfo, ExecInfo, Executor, ExecutorFty};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Sturct HexExecutor
struct HexExecutor {
    info: ExecInfo,
    hex: HexLoader,
}

// Impl HexExecutor
impl HexExecutor {
    // New
    const fn new() -> Self {
        Self {
            info: ExecInfo::new(),
            hex: HexLoader::new(),
        }
    }
}

// Impl HexExecutor
impl HexExecutor {
    // Sandbox
    fn sandbox(&mut self) {
        let argv = self.info.argv.iter_mut().map(|s| s.as_str()).collect();
        self.hex.execute(argv);
        self.hex.exit();
    }
}

// Impl executor for hex executor
impl Executor for HexExecutor {
    // Base
    fn base(&mut self) -> &mut ExecInfo {
        &mut self.info
    }

    // Initiate
    fn initiate(&mut self) -> i32 {
        // Load, parser and execute hex file
        if !self.hex.load(&self.info.path) {
            return -1;
        }

        // Create a sandbox thread to run the app
        let sandbox_cb = Callback::new(Self::sandbox as u32).with_instance(self);
        kernel().thread().create_task(&self.info.path, sandbox_cb)
    }

    // Release
    fn release(&mut self) -> bool {
        self.hex.exit()
    }
}

// Struct HexExecutorFty
struct HexExecutorFty {
    info: ExecFtyInfo,
}

// Impl HexExecutorFty
impl HexExecutorFty {
    const fn new() -> Self {
        Self {
            info: ExecFtyInfo::new(),
        }
    }
}

// Impl executor fty for hex executro fty
impl ExecutorFty for HexExecutorFty {
    // Info
    fn info(&mut self) -> &mut ExecFtyInfo {
        &mut self.info
    }

    // Get suffixes
    fn get_suffixes(&mut self) -> Vec<&str> {
        let mut suffixes = Vec::<&str>::new();
        suffixes.push(".hex");

        #[cfg(feature = "binding_exec_hex")]
        suffixes.push(".exec");

        suffixes
    }

    // Create
    fn create(&mut self) -> Box<dyn Executor> {
        Box::new(HexExecutor::new())
    }
}

// Register executor factory
register_exec_factory!(HexExecutorFty::new(), hex_executor_fty);
