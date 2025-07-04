//###########################################################################
// vk_cmd_about.rs
// The specific implementation of functions related to cmd about
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::format;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_command::{Cmd, CmdBase};

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
            console.println(&format!("build date      : {}", kernel().build_info().date));
            console.println(&format!("build time      : {}", kernel().build_info().time));
            console.println(&format!("build version   : {}", kernel().build_info().version));
            console.println(&format!("build gitcommit : {}", kernel().build_info().git_sha));
            console.println("village kernel is based on Rust.");
            console.println("A fast, safe and efficient operating system kernel.");
            console.println(&format!("License: GPL-3.0, Copyright (C) {} village.", kernel().build_info().year));
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
