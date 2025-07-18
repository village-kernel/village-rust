//###########################################################################
// vk_cmd_lib.rs
// The specific implementation of functions related to cmd lib
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

// Struct CmdListLib
struct CmdListLib;

// Impl cmd for CmdListLib
impl Cmd for CmdListLib {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 1 {
            console.println("Usage: lslib");
            return;
        }

        for library in kernel().library().get_libraries().iter_mut() {
            console.println(&format!("name {}", library.path));
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd lsmod: list installed library information");
    }
}

// Struct CmdInsLib
struct CmdInsLib;

// Impl cmd for CmdInsLib
impl Cmd for CmdInsLib {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: inslib [library]");
            return;
        }

        let path = console.real_path(argv[1]);

        if !kernel().library().install(&path) {
            console.error(&format!("Install library {} failed", path));
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd inslib: install library");
    }
}

// Struct CmdRmMod
struct CmdRmLib;

// Impl cmd for CmdRmLib
impl Cmd for CmdRmLib {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: rmlib [library]");
            return;
        }

        let path = console.real_path(argv[1]);

        if !kernel().library().uninstall(&path) {
            console.error(&format!("Uninstall library {} failed", path));
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd rmlib: remove library");
    }
}

// Register cmd
register_cmd!(CmdListLib, lslib);
register_cmd!(CmdInsLib, inslib);
register_cmd!(CmdRmLib, rmlib);
