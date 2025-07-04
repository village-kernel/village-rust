//###########################################################################
// vk_cmd_tasker.rs
// The specific implementation of functions related to cmd tasker
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_command::{Cmd, CmdBase};

// Struct cmd tasker
struct CmdTasker {
    base: CmdBase
}

// Impl cmd tasker
impl CmdTasker {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd tasker
impl Cmd for CmdTasker {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            console.println("tid   stack  start  ~  end   psp         size        used        state        name ");
            for task in kernel().thread().get_tasks().iter_mut() {
                console.println(&format!("{:<4}  0x{:08x}~0x{:08x}  0x{:08x}  0x{:08x}  0x{:08x}  {:<12} {}",
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
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd ts: list thread task");
        }
    }
}

// Register cmd
register_cmd!(CmdTasker::new(), ts);
