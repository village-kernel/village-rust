//###########################################################################
// vK_executor.rs
// The interfaces of functions related to executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// Struct executor info
pub struct ExecInfo {
    pub path: String,
    pub argv: Vec<String>,
    pub tid: i32,
}

// Impl executor info
impl ExecInfo {
    // New
    pub const fn new() -> Self {
        Self {
            path: String::new(),
            argv: Vec::new(),
            tid: 0,
        }
    }
}

// Executor
pub trait Executor {
    // trait methods
    fn base(&mut self) -> &mut ExecInfo;
    fn initiate(&mut self) -> i32;
    fn release(&mut self) -> bool;

    // Run
    fn run(&mut self, path: &str, argv: Vec<&str>) -> i32 {
        // Set path and argv
        self.base().path = path.to_string();
        self.base().argv = argv.into_iter().map(|s| s.to_string()).collect();

        // Load, parser file and create task
        self.base().tid = self.initiate();

        // Start task
        kernel().thread().start_task(self.base().tid);

        self.base().tid
    }

    // Wait
    fn wait(&mut self) {
        kernel().thread().wait_for_task(self.base().tid);
    }

    // Kill
    fn kill(&mut self) {
        kernel().thread().stop_task(self.base().tid);
        self.release();
    }
}

// Struct executor factory info
pub struct ExecFtyInfo {
    name: String,
}

// Impl executor factory info
impl ExecFtyInfo {
    // New
    pub const fn new() -> Self {
        Self {
            name: String::new(),
        }
    }

    // Set name
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // Get name
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// Executor factory
pub trait ExecutorFty {
    fn info(&mut self) -> &mut ExecFtyInfo;
    fn get_suffixes(&mut self) -> Vec<&str>;
    fn create(&mut self) -> Box<dyn Executor>;
}

// Register exec factory macro
#[macro_export]
macro_rules! register_exec_factory {
    ($fty:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let mut factory = Box::new($fty);
                factory.info().set_name(stringify!($name));
                kernel().process().register_exec_factory(factory);
            }

            fn [<$name _exit>]() {
                kernel().process().unregister_exec_factory(stringify!($name));
            }
        }
    };
}
