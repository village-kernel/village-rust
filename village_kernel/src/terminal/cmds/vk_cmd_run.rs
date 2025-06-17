//###########################################################################
// vk_cmd_run.rs
// The specific implementation of functions related to cmd run
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_commad::{Cmd, CmdBase};
use crate::traits::vk_kernel::ProcessBehavior;

// Struct cmd run
struct CmdRun {
    base: CmdBase
}

// Impl cmd run
impl CmdRun {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd run
impl Cmd for CmdRun {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            if argv.len() < 2 {
                console.println("Usage: run <program> [arg1] [arg2] [...] [&]");
                return;
            }

            let mut behavior = ProcessBehavior::Foreground;

            if argv[argv.len() - 1] == "&" {
                behavior = ProcessBehavior::Background;
            }

            let path = console.absolute_path(argv[1]);
            let mut argv = argv; argv.remove(0);
            kernel().process().run_with_argv(behavior, &path, argv);
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd run: execute application");
        }
    }
}

// Register cmd
register_cmd!(CmdRun::new(), run);
