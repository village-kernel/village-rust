//###########################################################################
// vk_cmd_tasker.rs
// The specific implementation of functions related to cmd tasker
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

// Struct cmd tasker
struct CmdTasker;

// Impl cmd for cmd tasker
impl Cmd for CmdTasker {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, _argv: Vec<&str>) {
        console.println("tid   stack  start  ~  end   psp         size        used        state        name ");
        for task in kernel().thread().get_tasks().iter_mut() {
            console.println(&format!(
                "{:<4}  0x{:08x}~0x{:08x}  0x{:08x}  0x{:08x}  0x{:08x}  {:<12} {}",
                task.id,
                task.stack_start,
                task.stack_end,
                task.psp,
                task.stack_end.wrapping_sub(task.stack_start),
                task.stack_end.wrapping_sub(task.psp),
                format!("{}", task.state.as_str()),
                task.name
            ));
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd ts: list thread task");
    }
}

// Register cmd
register_cmd!(CmdTasker, ts);
