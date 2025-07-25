//###########################################################################
// vk_cmd_mod.rs
// The specific implementation of functions related to cmd mod
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

// Struct CmdListMod
struct CmdListMod;

// Impl cmd for CmdListMod
impl Cmd for CmdListMod {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 1 {
            console.println("Usage: lsmod");
            return;
        }

        for module in kernel().module().get_modules().iter_mut() {
            console.println(&format!("name {}", module.path));
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd lsmod: list installed module information");
    }
}

// Struct CmdInsMod
struct CmdInsMod;

// Impl cmd for CmdInsMod
impl Cmd for CmdInsMod {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: insmod [module]");
            return;
        }

        let path = console.real_path(argv[1]);

        if !kernel().module().install(&path) {
            console.error(&format!("Install module {} failed", path));
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd insmod: install module");
    }
}

// Struct CmdRmMod
struct CmdRmMod;

// Impl cmd for CmdRmMod
impl Cmd for CmdRmMod {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: rmmod [module]");
            return;
        }

        let path = console.real_path(argv[1]);

        if !kernel().module().uninstall(&path) {
            console.error(&format!("Uninstall module {} failed", path));
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd rmmod: remove module");
    }
}

// Register cmd
register_cmd!(CmdListMod, lsmod);
register_cmd!(CmdInsMod, insmod);
register_cmd!(CmdRmMod, rmmod);
