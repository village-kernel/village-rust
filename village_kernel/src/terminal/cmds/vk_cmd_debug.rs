//###########################################################################
// vk_cmd_debug.rs
// The specific implementation of functions related to cmd debug
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::traits::vk_kernel::DebugLevel;
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Struct cmd debug
struct CmdDebug;

// Impl cmd for cmd debug
impl Cmd for CmdDebug {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 1 {
            console.println("Usage: debug [lv0|lv1|lv2|lv3|lv4|lv5]");
            return;
        }

        if let Some(level) = DebugLevel::from_str(argv[1]) {
            kernel().debug().set_debug_level(level);
        } else {
            console.println("Invalid level. Valid levels: [lv0|lv1|lv2|lv3|lv4|lv5]");
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd debug: settings debug output level");
    }
}

// Register cmd
register_cmd!(CmdDebug, debug);
