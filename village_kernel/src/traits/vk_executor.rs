//###########################################################################
// vK_executor.rs
// The interfaces of functions related to executor
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::vec::Vec;

// BaseLoader
pub trait BaseLoader {
    fn init(&mut self, filename: &str, data: &mut Vec<u8>) -> bool;
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
    name: &'static str,
    inner: Box<dyn Executor>,
}

// Impl ExecutorWrapper
impl ExecutorWrapper {
    // New with name
    #[inline]
    pub fn new(inner: Box<dyn Executor>, name: &'static str) -> Self {
        Self {
            name,
            inner,
        }
    }

    // Get name
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    // Get suffixes
    #[inline]
    pub fn suffixes(&self) -> Vec<&str> {
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
                let executor = crate::traits::vk_executor::ExecutorWrapper::new(
                    Box::new($fty), stringify!($name)
                );
                crate::village::kernel().process().register_executor(executor);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().process().unregister_executor(stringify!($name));
            }
        }
    };
}
