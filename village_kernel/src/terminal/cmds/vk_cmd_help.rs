//###########################################################################
// vk_cmd_help.rs
// The specific implementation of functions related to cmd help
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, CmdBase};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Struct cmd help
struct CmdHelp {
    base: CmdBase,
}

// Impl cmd help
impl CmdHelp {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd help
impl Cmd for CmdHelp {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            for cmd in kernel().terminal().get_cmds().iter_mut() {
                cmd.setup(console);
                cmd.help();
                cmd.exit();
            }
        }
    }

    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd help: display cmd help");
        }
    }
}

// Register cmd
register_cmd!(CmdHelp::new(), help);
