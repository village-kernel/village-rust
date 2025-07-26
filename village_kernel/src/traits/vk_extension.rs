//###########################################################################
// vK_extension.rs
// The interfaces of functions related to extension
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;

// extension id
#[derive(PartialEq, Clone)]
pub enum ExtensionID {
    Feature = 0,
    Service,
    Program,
}

// Impl extension id
impl ExtensionID {
    // Iterator
    pub fn iter() -> impl Iterator<Item = ExtensionID> {
        [ExtensionID::Feature, ExtensionID::Service, ExtensionID::Program].into_iter()
    }

    // Rev iterator
    pub fn rev_iter() -> impl Iterator<Item = ExtensionID> {
        [ExtensionID::Program, ExtensionID::Service, ExtensionID::Feature].into_iter()
    }
}

// Extension
pub trait Extension {
    fn setup(&mut self);
    fn exit(&mut self);
}

// Struct ExtensionWrapper
pub struct ExtensionWrapper {
    id: ExtensionID,
    name: &'static str,
    inner: Box<dyn Extension>,
}

// Impl ExtensionWrapper
impl ExtensionWrapper {
    // New with name
    #[inline]
    pub fn new(inner: Box<dyn Extension>, id: ExtensionID, name: &'static str) -> Self {
        Self {
            id,
            name,
            inner,
        }
    }

    // Get name
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    // Get id
    #[inline]
    pub fn id(&self) -> ExtensionID {
        self.id.clone()
    }

    // Setup
    #[inline]
    pub fn setup(&mut self) {
        self.inner.setup();
    }

    // Exit
    #[inline]
    pub fn exit(&mut self) {
        self.inner.exit();
    }
}
