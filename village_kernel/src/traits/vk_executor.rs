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

// BaseDecoder
pub trait BaseDecoder {
    fn init(&mut self, data: Vec<u8>) -> bool;
    fn exec(&mut self, argv: Vec<&str>) -> bool;
    fn exit(&mut self) -> bool;
}

// BaseRunner
pub trait BaseRunner {
    fn run(&mut self, path: &str, argv: Vec<&str>) -> i32;
    fn wait(&mut self);
    fn kill(&mut self);
}

// BaseExecutor
pub trait BaseExecutor {
    fn suffixes(&self) -> Vec<&str>;
    fn create(&self, suffix: &str) -> Option<Box<dyn BaseRunner>>;
}

// Struct ExecutorWrapper
pub struct ExecutorWrapper {
    name: &'static str,
    inner: Box<dyn BaseExecutor>,
}

// Impl ExecutorWrapper
impl ExecutorWrapper {
    // New with name
    #[inline]
    pub fn new(inner: Box<dyn BaseExecutor>, name: &'static str) -> Self {
        Self { name, inner, }
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
    pub fn create(&self, suffix: &str) -> Option<Box<dyn BaseRunner>> {
        self.inner.create(suffix)
    }
}

// Register executor macro
#[macro_export]
macro_rules! register_executor {
    ($exec:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let executor = crate::traits::vk_executor::ExecutorWrapper::new(
                    Box::new($exec), stringify!($name)
                );
                crate::village::kernel().process().register_executor(executor);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().process().unregister_executor(stringify!($name));
            }
        }
    };
}
