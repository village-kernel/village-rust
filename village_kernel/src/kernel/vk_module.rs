//###########################################################################
// vk_module.rs
// The specific implementation of functions related to module
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::parser::vk_rc_parser::RcParser;
use crate::traits::vk_kernel::{ModuleData, Module};
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;

// Struct village module
pub struct VillageModule {
    mods: LinkedList<ModuleData>,
}

// Impl village module
impl VillageModule {
    // New
    pub const fn new() -> Self {
        Self {
            mods: LinkedList::new(),
        }
    }
}

// Impl village module
impl VillageModule {
    // Setup
    pub fn setup(&mut self) {
        // Loading modules
        self.load_mods("/modules/_load_.rc");

        // Output debug info
        kernel().debug().info("Module setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Unloading modules
        self.unload_mods("/modules/_load_.rc");
    }
}

// Impl village module
impl VillageModule {
    // Load mods
    fn load_mods(&mut self, filename: &str) {
        let mut parser = RcParser::new();

        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for mod_name in run_cmds.iter_mut() {
                self.install(mod_name);
            }
        }
    }

    // Unload mods
    fn unload_mods(&mut self, filename: &str) {
        let mut parser = RcParser::new();

        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for mod_name in run_cmds.iter_mut().rev() {
                self.uninstall(mod_name);
            }
        }
    }
}

// Impl module for village module
impl Module for VillageModule {
    // Install
    fn install(&mut self, path: &str) -> bool {
        // Check the module if it has been installed
        for module in self.mods.iter_mut() {
            if module.path == path {
                kernel().debug().warning(&format!("{} has already been installed!", path));
                return true;
            }
        }

        // Install module if it has not install
        let mut module = ModuleData::new();

        // Set the path
        module.path = path.to_string();

        // Create runner
        module.runner = kernel().director().create_runner(path);
        if module.runner.is_none() {
            kernel().debug().error(&format!("{} unsupported file type!", path));
            return false;
        }

        // Run module without argv
        if module.runner.as_mut().unwrap().run(path, Vec::new()) < 0 {
            kernel().debug().error(&format!("{} install failed!", path));
            return false;
        }

        // Add into list
        self.mods.push(module);

        // Output debug info
        kernel().debug().info(&format!("{} install successful!", path));

        true
    }

    // Uninstall
    fn uninstall(&mut self, path: &str) -> bool {
        let mut is_unistall = false;

        self.mods.retain_mut(|module| {
            if module.path == path {
                is_unistall = true;
                module.runner.as_mut().unwrap().kill();
                kernel().debug().info(&format!("{} uninstall successful!", path));
                false
            } else {
                true
            }
        });

        if !is_unistall {
            kernel().debug().error(&format!("{} module not found!", path));
            return false;
        }
        
        true
    }

    // Get modules
    fn get_modules(&mut self) -> &mut LinkedList<ModuleData> {
        &mut self.mods
    }
}
