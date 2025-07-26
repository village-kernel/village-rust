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
    fn init(&mut self, path: &str, data: &mut Vec<u8>) -> bool;
    fn exit(&mut self) -> bool;
}

// LibDecoder
pub trait LibDecoder {
    fn init(&mut self, path: &str, data: Vec<u8>) -> bool;
    fn get(&mut self, symbol: &str) -> usize;
    fn exit(&mut self) -> bool;
}

// LibContainer
pub trait LibContainer {
    fn init(&mut self, path: &str) -> bool;
    fn get(&mut self, symbol: &str) -> usize;
    fn exit(&mut self) -> bool;
}

// LibBuiulder
pub trait LibBuilder {
    fn suffixes(&self) -> Vec<&str>;
    fn create(&self, suffix: &str) -> Option<Box<dyn LibContainer>>;
}

// ProgLoader
pub trait ProgLoader {
    fn init(&mut self, path: &str, data: &mut Vec<u8>) -> bool;
    fn exit(&mut self) -> bool;
}

// ProgDecoder
pub trait ProgDecoder {
    fn init(&mut self, path: &str, data: Vec<u8>) -> bool;
    fn exec(&mut self, argv: Vec<&str>) -> bool;
    fn exit(&mut self) -> bool;
}

// ProgContainer
pub trait ProgContainer {
    fn run(&mut self, path: &str, argv: Vec<&str>) -> i32;
    fn wait(&mut self);
    fn kill(&mut self);
}

// ProgBuilder
pub trait ProgBuilder {
    fn suffixes(&self) -> Vec<&str>;
    fn create(&self, suffix: &str) -> Option<Box<dyn ProgContainer>>;
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
    pub fn create(&self, suffix: &str) -> Option<Box<dyn LibContainer>> {
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
    pub fn create(&self, suffix: &str) -> Option<Box<dyn ProgContainer>> {
        self.inner.create(suffix)
    }
}
