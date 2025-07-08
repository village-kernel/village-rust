//###########################################################################
// vk_cmd_null.rs
// The specific implementation of functions related to cmd null
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Struct cmd null
struct CmdNull;

// Impl cmd for cmd null
impl Cmd for CmdNull {
    // Execute
    fn exec(&mut self, _console: &mut dyn Console, _argv: Vec<&str>) {}

    // Help
    fn help(&mut self, _console: &mut dyn Console) {}
}

// Register cmd
register_cmd!(CmdNull, null);
