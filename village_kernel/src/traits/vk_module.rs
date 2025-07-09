//###########################################################################
// vK_module.rs
// The interfaces of functions related to module
//
// $Copyright: Copyright (C) village
//###########################################################################
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
    name: &'static str,
    inner: Box<dyn Module>,
}

// Impl ModuleWrapper
impl ModuleWrapper {
    // New with name
    #[inline]
    pub fn new(inner: Box<dyn Module>, id: ModuleID, name: &'static str) -> Self {
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
    pub fn id(&self) -> ModuleID {
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
                let module = crate::traits::vk_module::ModuleWrapper::with_id_name(
                    Box::new($mod), $id, stringify!($name)
                );
                crate::village::kernel().feature().register_module(module);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().feature().unregister_module(stringify!($name));
            }
        }
    };
}
