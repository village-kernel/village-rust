//###########################################################################
// vk_cmd_kill.rs
// The specific implementation of functions related to cmd kill
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_command::{Cmd, CmdBase};

// Struct cmd kill
struct CmdKill {
    base: CmdBase
}

// Impl cmd kill
impl CmdKill {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd kill
impl Cmd for CmdKill {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            if argv.len() < 2 {
                console.println("Usage: ");
                console.println("kill -p <pid>");
                console.println("kill -t <tid>");
                return;
            }

            if argv[1] == "-p" {
                kernel().process().kill_by_pid(argv[2].parse().unwrap());
            } else if argv[1] == "-t" {
                kernel().thread().stop_task(argv[2].parse().unwrap());
            }
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd kill: kill process");
        }
    }
}

// Register cmd
register_cmd!(CmdKill::new(), kill);
