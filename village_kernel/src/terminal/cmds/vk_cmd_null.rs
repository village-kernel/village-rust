//###########################################################################
// vk_cmd_null.rs
// The specific implementation of functions related to cmd null
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_command::{Cmd, CmdBase};

// Struct cmd null
struct CmdNull {
    base: CmdBase
}

// Impl cmd null
impl CmdNull {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd null
impl Cmd for CmdNull {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {

    }
    
    // Help
    fn help(&mut self) {

    }
}

// Register cmd
register_cmd!(CmdNull::new(), null);
