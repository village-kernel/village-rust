//###########################################################################
// vk_cmd_memory.rs
// The specific implementation of functions related to cmd memory
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

// Struct cmd memory
struct CmdMemory;

// Impl cmd for cmd memory
impl Cmd for CmdMemory {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, _argv: Vec<&str>) {
        let size = kernel().memory().get_size();
        let used = kernel().memory().get_used();
        let per = used as f32 * 100.0 / size as f32;
        console.println(&format!("memory size: 0x{:08x} Bytes, memory used: 0x{:08x} Bytes, percentage used: {:0.2} %", size, used, per));
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd memory: show memory used information");
    }
}

// Register cmd
register_cmd!(CmdMemory, memory);
