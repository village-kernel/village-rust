//###########################################################################
// vK_builder.rs
// The interfaces of functions related to builder
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::vec::Vec;

// LibLoader
pub trait LibLoader {
    fn init(&mut self, filename: &str) -> bool;
    fn get(&mut self, symbol: &str) -> usize;
    fn exit(&mut self) -> bool;
}

// LibBuiulder
pub trait LibBuilder {
    fn suffixes(&self) -> Vec<&str>;
    fn create(&self, suffix: &str) -> Option<Box<dyn LibLoader>>;
}

// ProgLoader
pub trait ProgLoader {
    fn init(&mut self, filename: &str, data: &mut Vec<u8>) -> bool;
    fn exit(&mut self) -> bool;
}

// ProgDecoder
pub trait ProgDecoder {
    fn init(&mut self, data: Vec<u8>) -> bool;
    fn exec(&mut self, argv: Vec<&str>) -> bool;
    fn exit(&mut self) -> bool;
}

// ProgRunner
pub trait ProgRunner {
    fn run(&mut self, path: &str, argv: Vec<&str>) -> i32;
    fn wait(&mut self);
    fn kill(&mut self);
}

// ProgBuilder
pub trait ProgBuilder {
    fn suffixes(&self) -> Vec<&str>;
    fn create(&self, suffix: &str) -> Option<Box<dyn ProgRunner>>;
}

// Struct LibBuilderWrapper
pub struct LibBuilderWrapper {
    name: &'static str,
    inner: Box<dyn LibBuilder>,
}

// Impl LibBuilderWrapper
impl LibBuilderWrapper {
    // New with name
    #[inline]
    pub fn new(inner: Box<dyn LibBuilder>, name: &'static str) -> Self {
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
    pub fn create(&self, suffix: &str) -> Option<Box<dyn LibLoader>> {
        self.inner.create(suffix)
    }
}

// Struct ProgBuilderWrapper
pub struct ProgBuilderWrapper {
    name: &'static str,
    inner: Box<dyn ProgBuilder>,
}

// Impl ProgBuilderWrapper
impl ProgBuilderWrapper {
    // New with name
    #[inline]
    pub fn new(inner: Box<dyn ProgBuilder>, name: &'static str) -> Self {
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
    pub fn create(&self, suffix: &str) -> Option<Box<dyn ProgRunner>> {
        self.inner.create(suffix)
    }
}

// Register lib builder macro
#[macro_export]
macro_rules! register_lib_builder {
    ($lib:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let builder = crate::traits::vk_builder::LibBuilderWrapper::new(
                    Box::new($lib), stringify!($name)
                );
                crate::village::kernel().director().register_lib_builder(builder);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().director().unregister_lib_builder(stringify!($name));
            }
        }
    };
}

// Register prog builder macro
#[macro_export]
macro_rules! register_prog_builder {
    ($prog:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let builder = crate::traits::vk_builder::ProgBuilderWrapper::new(
                    Box::new($prog), stringify!($name)
                );
                crate::village::kernel().director().register_prog_builder(builder);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().director().unregister_prog_builder(stringify!($name));
            }
        }
    };
}
