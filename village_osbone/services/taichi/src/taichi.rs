//###########################################################################
// main.rs
// The specific implementation of functions related to taichi
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::ProcessBehavior;
use crate::misc::parser::vk_rc_parser::RcParser;

// Struct Taichi
pub struct Taichi;

// Impl Taichi
impl Taichi {
    // Load
    fn load(&mut self, filename: &str) {
        let mut parser = RcParser::new();

        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for args in run_cmds.iter_mut() {
                kernel().process().run_with_args(ProcessBehavior::Background, &args);
            }
        }
    }

    // Load
    fn unload(&mut self, filename: &str) {
        let mut parser = RcParser::new();

        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for path in run_cmds.iter_mut() {
                kernel().process().kill_by_path(path);
            }
        }
    }
}

// Impl Taichi
impl Taichi {
    // Setup
    pub fn setup(&mut self) {
        // Load services
        self.load("/services/_load_.rc");

        // Load programs
        self.load("/programs/_load_.rc");
    }

    // Execute
    pub fn execute(&mut self) {
        kernel().thread().blocked();
    }

    // Exit
    pub fn exit(&mut self) {
        // Unload programs
        self.unload("/programs/_load_.rc");

        // Load services
        self.unload("/services/_load_.rc");
    }
}
