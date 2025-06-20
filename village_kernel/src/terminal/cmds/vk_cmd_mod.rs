//###########################################################################
// vk_cmd_mod.rs
// The specific implementation of functions related to cmd mod
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::format;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_command::{Cmd, CmdBase};

// Struct CmdListMod
struct CmdListMod {
    base: CmdBase
}

// Impl CmdListMod
impl CmdListMod {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for CmdListMod
impl Cmd for CmdListMod {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            if argv.len() < 1 {
                console.println("Usage: lsmod");
                return;
            }

            for module in kernel().loader().get_modules().iter_mut() {
                console.println(&format!("name {}", module.get_filename()));
            }
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd lsmod: list installed module information");
        }
    }
}

// Struct CmdInsMod
struct CmdInsMod {
    base: CmdBase
}

// Impl CmdInsMod
impl CmdInsMod {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for CmdInsMod
impl Cmd for CmdInsMod {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            if argv.len() < 2 {
                console.println("Usage: insmod [module]");
                return;
            }
            if !kernel().loader().install_mod(argv[1]) {
                console.error(&format!("Install module {} failed", argv[1]));
            }
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd insmod: install module");
        }
    }
}

// Struct CmdRmMod
struct CmdRmMod {
    base: CmdBase
}

// Impl CmdRmMod
impl CmdRmMod {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd for CmdRmMod
impl Cmd for CmdRmMod {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            if argv.len() < 2 {
                console.println("Usage: rmmod [module]");
                return;
            }
            if !kernel().loader().uninstall_mod(argv[1]) {
                console.error(&format!("Uninstall module {} failed", argv[1]));
            }
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd rmmod: remove module");
        }
    }
}

// Register cmd
register_cmd!(CmdListMod::new(), lsmod);
register_cmd!(CmdInsMod::new(), insmod);
register_cmd!(CmdRmMod::new(), rmmod);
