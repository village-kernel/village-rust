//###########################################################################
// vk_terminal.rs
// The specific implementation of functions related to terminal
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel::traits::vk_kernel::Terminal;

/// struct concrete terminal
pub struct ConcreteTerminal;

/// impl concrete terminal
impl ConcreteTerminal {
    /// setup
    pub fn setup(&self) {

    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl terminal for concrete terminal
impl Terminal for ConcreteTerminal {
    /// register cmd
    fn register_cmd(&self) {

    }

    /// unregister cmd
    fn unregister_cmd(&self) {
        
    }
}
