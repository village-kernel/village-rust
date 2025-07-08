//###########################################################################
// vk_cmd_power.rs
// The specific implementation of functions related to cmd power
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Struct cmd sleep
struct CmdSleep;

// Impl cmd for cmd sleep
impl Cmd for CmdSleep {
    // Execute
    fn exec(&mut self, _console: &mut dyn Console, _argv: Vec<&str>) {
        kernel().system().sleep();
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd sleep: enter sleep mode");
    }
}

// Struct cmd standby
struct CmdStandby;

// Impl cmd for cmd standby
impl Cmd for CmdStandby {
    // Execute
    fn exec(&mut self, _console: &mut dyn Console, _argv: Vec<&str>) {
        kernel().system().standby();
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd standby: enter standby mode");
    }
}

// Struct cmd shutdown
struct CmdShutdown;

// Impl cmd for cmd shutdown
impl Cmd for CmdShutdown {
    // Execute
    fn exec(&mut self, _console: &mut dyn Console, _argv: Vec<&str>) {
        kernel().system().shutdown();
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd shutdown: shutdown device");
    }
}

// Struct cmd reboot
struct CmdReboot;

// Impl cmd for cmd reboot
impl Cmd for CmdReboot {
    // Execute
    fn exec(&mut self, _console: &mut dyn Console, _argv: Vec<&str>) {
        kernel().system().reboot();
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd reboot: reboot device");
    }
}

// Register cmd
register_cmd!(CmdSleep, sleep);
register_cmd!(CmdStandby, standby);
register_cmd!(CmdShutdown, shutdown);
register_cmd!(CmdReboot, reboot);
