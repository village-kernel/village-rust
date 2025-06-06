//###########################################################################
// vk_terminal.rs
// The specific implementation of functions related to terminal
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Terminal;

// struct concrete terminal
pub struct ConcreteTerminal;

// impl concrete terminal
impl ConcreteTerminal {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete terminal
impl ConcreteTerminal {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Terminal setup done!");
    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl terminal for concrete terminal
impl Terminal for ConcreteTerminal {
    // register cmd
    fn register_cmd(&mut self) {

    }

    // unregister cmd
    fn unregister_cmd(&mut self) {
        
    }
}
