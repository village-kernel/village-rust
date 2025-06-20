//###########################################################################
// vk_cmd_power.rs
// The specific implementation of functions related to cmd power
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_command::{Cmd, CmdBase};

// Struct cmd sleep
struct CmdSleep {
    base: CmdBase
}

// Impl cmd sleep
impl CmdSleep {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd sleep
impl Cmd for CmdSleep {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        kernel().sleep();
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd sleep: enter sleep mode");
        }
    }
}

// Struct cmd standby
struct CmdStandby {
    base: CmdBase
}

// Impl cmd standby
impl CmdStandby {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd standby
impl Cmd for CmdStandby {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        kernel().standby();
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd standby: enter standby mode");
        }
    }
}

// Struct cmd shutdown
struct CmdShutdown {
    base: CmdBase
}

// Impl cmd shutdown
impl CmdShutdown {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd shutdown
impl Cmd for CmdShutdown {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        kernel().shutdown();
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd shutdown: shutdown device");
        }
    }
}

// Struct cmd reboot
struct CmdReboot {
    base: CmdBase
}

// Impl cmd reboot
impl CmdReboot {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for cmd reboot
impl Cmd for CmdReboot {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, _argv: Vec<&str>) {
        kernel().reboot();
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd reboot: reboot device");
        }
    }
}

// Register cmd
register_cmd!(CmdSleep::new(), sleep);
register_cmd!(CmdStandby::new(), standby);
register_cmd!(CmdShutdown::new(), shutdown);
register_cmd!(CmdReboot::new(), reboot);
