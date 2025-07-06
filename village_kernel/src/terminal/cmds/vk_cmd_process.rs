//###########################################################################
// vk_cmd_process.rs
// The specific implementation of functions related to cmd process
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, CmdBase};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

// Struct cmd process
struct CmdProcess {
    base: CmdBase,
}

// Impl cmd process
impl CmdProcess {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd process
impl Cmd for CmdProcess {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            for process in kernel().process().get_processes().iter_mut() {
                console.println(&format!(
                    "pid {:<2}, pid {:<2}, path {}",
                    process.pid, process.tid, process.path,
                ));
            }
        }
    }

    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd ps: list process infos");
        }
    }
}

// Register cmd
register_cmd!(CmdProcess::new(), ps);
