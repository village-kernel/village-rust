//###########################################################################
// vK_module.rs
// The interfaces of functions related to module
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::string::{String, ToString};

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

// Struct module info
pub struct ModuleInfo {
    id: ModuleID,
    name: String,
}

// Impl module data
impl ModuleInfo {
    // New
    pub const fn new() -> Self {
        Self {
            id: ModuleID::Feature,
            name: String::new(),
        }
    }

    // Set id
    pub fn set_id(&mut self, id: ModuleID) {
        self.id = id;
    }

    // Get id
    pub fn get_id(&self) -> ModuleID {
        self.id.clone()
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

// Module
pub trait Module {
    fn info(&mut self) -> &mut ModuleInfo;
    fn setup(&mut self);
    fn exit(&mut self);
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
                let mut module = Box::new($mod);
                module.info().set_name(stringify!($name));
                module.info().set_id($id);
                kernel().feature().register_module(module);
            }

            fn [<$name _exit>]() {
                kernel().feature().unregister_module(stringify!($name));
            }
        }
    };
}
