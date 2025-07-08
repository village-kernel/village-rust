//###########################################################################
// vk_cmd_help.rs
// The specific implementation of functions related to cmd help
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Struct cmd help
struct CmdHelp;

// Impl cmd for cmd help
impl Cmd for CmdHelp {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, _argv: Vec<&str>) {
        for cmd in kernel().terminal().get_cmds().iter_mut() {
            cmd.help(console);
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd help: display cmd help");
    }
}

// Register cmd
register_cmd!(CmdHelp, help);
