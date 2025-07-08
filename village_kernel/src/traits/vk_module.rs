//###########################################################################
// vK_module.rs
// The interfaces of functions related to module
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::string::{String, ToString};
use alloc::boxed::Box;

// Module id
#[derive(PartialEq, Clone)]
pub enum ModuleID {
    Feature = 0,
    Service,
    Program,
}

// Impl module id
impl ModuleID {
    // Iterator
    pub fn iter() -> impl Iterator<Item = ModuleID> {
        [ModuleID::Feature, ModuleID::Service, ModuleID::Program].into_iter()
    }

    // Rev iterator
    pub fn rev_iter() -> impl Iterator<Item = ModuleID> {
        [ModuleID::Program, ModuleID::Service, ModuleID::Feature].into_iter()
    }
}

// Module
pub trait Module {
    fn setup(&mut self);
    fn exit(&mut self);
}

// Struct ModuleWrapper
pub struct ModuleWrapper {
    id: ModuleID,
    name: String,
    inner: Box<dyn Module>,
}

// Impl ModuleWrapper
impl ModuleWrapper {
    // New
    #[inline]
    pub const fn new(inner: Box<dyn Module>) -> Self {
        Self {
            id: ModuleID::Feature,
            name: String::new(),
            inner,
        }
    }

    // New with name
    #[inline]
    pub fn with_id_name(inner: Box<dyn Module>, id: ModuleID, name: &str) -> Self {
        Self {
            id,
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

    // Set id
    #[inline]
    pub fn set_id(&mut self, id: ModuleID) {
        self.id = id;
    }

    // Get id
    #[inline]
    pub fn get_id(&self) -> ModuleID {
        self.id.clone()
    }

    // box mut
    #[inline]
    pub fn box_mut(&mut self) -> &mut Box<dyn Module> {
        &mut self.inner
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


// Register module macro
#[macro_export]
macro_rules! register_module {
    ($mod:expr, $id:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[link_section = ".init_array"]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[link_section = ".fini_array"]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let module = Box::new(
                    crate::traits::vk_module::ModuleWrapper::with_id_name(
                        Box::new($mod), $id, stringify!($name)
                    )
                );
                crate::village::kernel().feature().register_module(module);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().feature().unregister_module(stringify!($name));
            }
        }
    };
}
