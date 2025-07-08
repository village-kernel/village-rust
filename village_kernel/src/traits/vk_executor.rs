//###########################################################################
// vK_executor.rs
// The interfaces of functions related to executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// BaseLoader
pub trait BaseLoader {
    fn load(&mut self, filename: &str) -> bool;
    fn exec(&mut self, argv: Vec<&str>) -> bool;
    fn exit(&mut self) -> bool;
}

// BaseExecutor
pub trait BaseExecutor {
    fn run(&mut self, path: &str, argv: Vec<&str>) -> i32;
    fn wait(&mut self);
    fn kill(&mut self);
}

// Executor
pub trait Executor {
    fn suffixes(&self) -> Vec<&str>;
    fn create(&self) -> Box<dyn BaseExecutor>;
}

// Struct ExecutorWrapper
pub struct ExecutorWrapper {
    name: String,
    inner: Box<dyn Executor>,
}

// Impl ExecutorWrapper
impl ExecutorWrapper {
    // New
    #[inline]
    pub const fn new(inner: Box<dyn Executor>) -> Self {
        Self {
            name: String::new(),
            inner,
        }
    }

    // New with name
    #[inline]
    pub fn with_name(inner: Box<dyn Executor>, name: &str) -> Self {
        Self {
            name: name.to_string(),
            inner,
        }
    }

    // Set name
    #[inline]
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    // Get name
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // get_suffixes
    #[inline]
    pub fn get_suffixes(&self) -> Vec<&str> {
        self.inner.suffixes()
    }

    // create
    #[inline]
    pub fn create(&self) -> Box<dyn BaseExecutor> {
        self.inner.create()
    }
}

// Register executor macro
#[macro_export]
macro_rules! register_executor {
    ($fty:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let factory = Box::new(
                    crate::traits::vk_executor::ExecutorWrapper::with_name(
                        Box::new($fty), stringify!($name)
                    )
                );
                crate::village::kernel().process().register_executor(factory);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().process().unregister_executor(stringify!($name));
            }
        }
    };
}
