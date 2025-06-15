//###########################################################################
// vk_cmd_tasker.rs
// The specific implementation of functions related to cmd tasker
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_commad::{Cmd, CmdBase};

// Struct cmd tasker
struct CmdTasker {
    base: CmdBase
}

// Impl cmd tasker
impl CmdTasker {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd tasker
impl Cmd for CmdTasker {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            for task in kernel().thread().get_tasks().iter_mut() {
                console.println(&format!("tid {:<2}, stack 0x{:08X}, psp 0x{:08X}, state {:<12} ticks {:>8}, name {}",
                    task.id, 
                    task.stack, 
                    task.psp, 
                    format!("{},", task.state.as_str()),
                    task.ticks, 
                    task.name
                ));
            }
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd ts: list thread task");
        }
    }
}

// Register cmd
register_cmd!(CmdTasker::new(), ts);
