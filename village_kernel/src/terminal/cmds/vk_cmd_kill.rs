//###########################################################################
// vk_cmd_kill.rs
// The specific implementation of functions related to cmd kill
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Struct cmd kill
struct CmdKill;

// Impl cmd for cmd kill
impl Cmd for CmdKill {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
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

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd kill: kill process");
    }
}

// Register cmd
register_cmd!(CmdKill, kill);
