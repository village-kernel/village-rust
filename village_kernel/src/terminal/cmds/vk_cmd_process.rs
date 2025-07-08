//###########################################################################
// vk_cmd_process.rs
// The specific implementation of functions related to cmd process
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

// Struct cmd process
struct CmdProcess;

// Impl cmd for cmd process
impl Cmd for CmdProcess {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, _argv: Vec<&str>) {
        for process in kernel().process().get_processes().iter_mut() {
            console.println(&format!(
                "pid {:<2}, pid {:<2}, path {}",
                process.pid, process.tid, process.path,
            ));
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd ps: list process infos");
    }
}

// Register cmd
register_cmd!(CmdProcess, ps);
