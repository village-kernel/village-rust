//###########################################################################
// vk_cmd_memory.rs
// The specific implementation of functions related to cmd memory
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_commad::{Cmd, CmdBase};

// Struct cmd mem
struct CmdMem {
    base: CmdBase
}

// Impl cmd mem
impl CmdMem {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd mem
impl Cmd for CmdMem {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            let size = kernel().memory().get_size();
            let used = kernel().memory().get_used();
            let per  = used as f32 * 100.0 / size as f32;
            console.println(&format!("memory size: {} Byte, memory used: {} Byte, percentage used: {:0.2} %", size, used, per));
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd memory: show memory used information");
        }
    }
}

// Register cmd
register_cmd!(CmdMem::new(), memory);
