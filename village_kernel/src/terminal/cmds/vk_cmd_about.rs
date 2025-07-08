//###########################################################################
// vk_cmd_about.rs
// The specific implementation of functions related to cmd about
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;

// Struct cmd about
struct CmdAbout;

// Impl cmd for cmd about
impl Cmd for CmdAbout {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, _argv: Vec<&str>) {
        console.println(&format!("build date      : {}", kernel().build_info().date));
        console.println(&format!("build time      : {}", kernel().build_info().time));
        console.println(&format!(
            "build version   : {}",
            kernel().build_info().version
        ));
        console.println(&format!(
            "build gitcommit : {}",
            kernel().build_info().git_sha
        ));
        console.println("village kernel is based on Rust.");
        console.println("A fast, safe and efficient operating system kernel.");
        console.println(&format!(
            "License: GPL-3.0, Copyright (C) {} village.",
            kernel().build_info().year
        ));
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd about: display build information");
    }
}

// Register cmd
register_cmd!(CmdAbout, about);
