//###########################################################################
// vk_cmd_about.rs
// The specific implementation of functions related to cmd about
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_commad::{Cmd, CmdBase};

// Struct cmd about
struct CmdAbout {
    base: CmdBase
}

// Impl cmd about
impl CmdAbout {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd about
impl Cmd for CmdAbout {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            console.println("village kernel base on rust.");
            console.println("village kernel Copyright (C) village.");
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd about: display build information");
        }
    }
}

// Register cmd
register_cmd!(CmdAbout::new(), about);
