//###########################################################################
// vk_symbol.rs
// The specific implementation of functions related to symbol
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Symbol;

// struct concrete symbol
pub struct ConcreteSymbol;

// impl concrete symbol
impl ConcreteSymbol {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete symbol
impl ConcreteSymbol {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Symbol setup done!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl symbol for concrete symbol
impl Symbol for ConcreteSymbol {
    // export
    fn export(&mut self, sym_addr: u32, name: &str) {
        let _ = name;
        let _ = sym_addr;
    }

    // unexport
    fn unexport(&mut self, name: &str) {
        let _ = name;
    }
    
    // search
    fn search(&mut self, name: &str) {
        let _ = name;
    }
}
