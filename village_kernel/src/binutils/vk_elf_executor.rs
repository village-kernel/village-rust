//###########################################################################
// vk_elf_executor.rs
// The specific implementation of functions related to elf executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::village::kernel;
use crate::register_exec_factory;
use crate::traits::vk_executor::{ExecFtyInfo, ExecInfo, Executor, ExecutorFty};
use crate::traits::vk_callback::Callback;
use super::vk_elf_loader::ElfLoader;

// Sturct ElfExecutor
struct ElfExecutor {
    info: ExecInfo,
    elf: ElfLoader,
}

// Impl ElfExecutor
impl ElfExecutor {
    // New
    const fn new() -> Self {
        Self {
            info: ExecInfo::new(),
            elf: ElfLoader::new(),
        }
    }
}

// Impl ElfExecutor
impl ElfExecutor {
    // Sandbox
    fn sandbox(&mut self) {
        let argv = self.info.argv.iter_mut().map(|s| s.as_str()).collect();
        self.elf.execute(argv);
        self.elf.exit();
    }
}

// Impl executor for elf executor
impl Executor for ElfExecutor {
    // Base
    fn base(&mut self) -> &mut ExecInfo {
        &mut self.info
    }

    // Initiate
    fn initiate(&mut self) -> i32 {
        // Load, parser and execute elf file
        if !self.elf.load(&self.info.path) {
            return -1;
        }

        // Create a sandbox thread to run the app
        let sandbox_cb = Callback::new(Self::sandbox as u32).with_instance(self);
        kernel().thread().create_task(&self.info.path, sandbox_cb)
    }

    // Release
    fn release(&mut self) -> bool {
        self.elf.exit()
    }
}

// Struct ElfExecutorFty
struct ElfExecutorFty {
    info: ExecFtyInfo,
}

// Impl ElfExecutorFty
impl ElfExecutorFty {
    const fn new() -> Self {
        Self {
            info: ExecFtyInfo::new(),
        }
    }
}

// Impl executor fty for elf executro fty
impl ExecutorFty for ElfExecutorFty {
    // Info
    fn info(&mut self) -> &mut ExecFtyInfo {
        &mut self.info
    }

    // Get suffixes
    fn get_suffixes(&mut self) -> Vec<&str> {
        let mut suffixes = Vec::<&str>::new();
        suffixes.push(".elf");
        suffixes
    }

    // Create
    fn create(&mut self) -> Box<dyn Executor> {
        Box::new(ElfExecutor::new())
    }
}

// Register executor factory
register_exec_factory!(ElfExecutorFty::new(), elf_executor_fty);
