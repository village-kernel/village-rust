//###########################################################################
// vk_symbol.rs
// The specific implementation of functions related to symbol
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Symbol;

// struct concrete symbol
pub struct ConcreteSymbol;

// impl concrete symbol
impl ConcreteSymbol {
    // setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Symbol setup done!");
    }

    // exit
    pub fn exit(&self) {

    }
}

// impl symbol for concrete symbol
impl Symbol for ConcreteSymbol {
    // export
    fn export(&self, sym_addr: u32, name: &str) {

    }

    // unexport
    fn unexport(&self, name: &str) {

    }
    
    // search
    fn search(&self, name: &str) {

    }
}
