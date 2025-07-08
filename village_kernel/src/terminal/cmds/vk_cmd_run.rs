//###########################################################################
// vk_cmd_run.rs
// The specific implementation of functions related to cmd run
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::traits::vk_kernel::ProcessBehavior;
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Struct cmd run
struct CmdRun;

// Impl cmd for cmd run
impl Cmd for CmdRun {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: run <program> [arg1] [arg2] [...] [&]");
            return;
        }

        let mut behavior = ProcessBehavior::Foreground;

        if argv[argv.len() - 1] == "&" {
            behavior = ProcessBehavior::Background;
        }

        let path = console.real_path(argv[1]);
        let mut argv = argv;
        argv.remove(0);
        kernel().process().run_with_argv(behavior, &path, argv);
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd run: execute application");
    }
}

// Register cmd
register_cmd!(CmdRun, run);
