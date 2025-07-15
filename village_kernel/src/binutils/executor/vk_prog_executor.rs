//###########################################################################
// vk_prog_executor.rs
// The specific implementation of functions related to prog executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_bin_loader::BinLoader;
use crate::binutils::loader::vk_elf_loader::ElfLoader;
use crate::binutils::loader::vk_hex_loader::HexLoader;
use crate::traits::vk_callback::Callback;
use crate::traits::vk_executor::{BaseLoader, BaseExecutor, Executor};
use crate::village::kernel;
use crate::register_executor;
use alloc::vec;
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

// Struct BinExecutor
struct BinExecutor;

// Impl executor for bin executor
impl Executor for BinExecutor {
    // Get suffixes
    fn suffixes(&self) -> Vec<&str> {
        #[cfg(not(feature = "binding_exec_bin"))]
        return vec![".bin"];

        #[cfg(feature = "binding_exec_bin")]
        return vec![".bin", ".exec"];
    }

    // Create
    fn create(&self) -> Box<dyn BaseExecutor> {
        Box::new(ProgExecutor::new(Box::new(BinLoader::new())))
    }
}

// Struct ElfExecutor
struct ElfExecutor;

// Impl executor for elf executro
impl Executor for ElfExecutor {
    // Get suffixes
    fn suffixes(&self) -> Vec<&str> {
        #[cfg(not(feature = "binding_exec_elf"))]
        return vec![".elf"];

        #[cfg(feature = "binding_exec_elf")]
        return vec![".elf", ".exec"];
    }

    // Create
    fn create(&self) -> Box<dyn BaseExecutor> {
        Box::new(ProgExecutor::new(Box::new(ElfLoader::new())))
    }
}

// Struct HexExecutor
struct HexExecutor;

// Impl executor for hex executor
impl Executor for HexExecutor {
    // Get suffixes
    fn suffixes(&self) -> Vec<&str> {
        #[cfg(not(feature = "binding_exec_hex"))]
        return vec![".hex"];

        #[cfg(feature = "binding_exec_hex")]
        return vec![".hex", ".exec"];
    }

    // Create
    fn create(&self) -> Box<dyn BaseExecutor> {
        Box::new(ProgExecutor::new(Box::new(HexLoader::new())))
    }
}

// Register executor
register_executor!(BinExecutor, bin_executor);
register_executor!(ElfExecutor, elf_executor);
register_executor!(HexExecutor, hex_executor);
