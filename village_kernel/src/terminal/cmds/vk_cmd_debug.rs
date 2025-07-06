//###########################################################################
// vk_cmd_debug.rs
// The specific implementation of functions related to cmd debug
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, CmdBase};
use crate::traits::vk_kernel::DebugLevel;
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Struct cmd debug
struct CmdDebug {
    base: CmdBase,
}

// Impl cmd debug
impl CmdDebug {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd debug
impl Cmd for CmdDebug {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
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
    }

    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd debug: settings debug output level");
        }
    }
}

// Register cmd
register_cmd!(CmdDebug::new(), debug);
