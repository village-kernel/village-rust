//###########################################################################
// vk_module_tool.rs
// The specific implementation of functions related to module tool
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::loader::vk_mod_loader::ModLoader;
use crate::traits::vk_kernel::DebugLevel;
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;

// Struct ModuleTool
pub struct ModuleTool {
    mods: LinkedList<Box<ModLoader>>,
}

// Impl ModuleTool
impl ModuleTool {
    // New
    pub const fn new() -> Self {
        Self {
            mods: LinkedList::new(),
        }
    }

    // Install
    pub fn install(&mut self, filename: &str) -> bool {
        // Check the module if it has been installed
        for module in self.mods.iter_mut() {
            if module.get_filename() == filename {
                kernel().debug().output(
                    DebugLevel::Lv2,
                    &format!("{} module has already been installed", filename),
                );
                return true;
            }
        }

        // Install module if it has not install
        let mut module = Box::new(ModLoader::new());

        // Load module
        if module.load(filename) {
            module.fill_bss_zero();
            module.init_array();
            self.mods.push(module);
            kernel().debug().output(
                DebugLevel::Lv2,
                &format!("{} module install successful", filename),
            );
            return true;
        }

        // Install failed
        kernel()
            .debug()
            .error(&format!("{} module install failed", filename));
        false
    }

    // Uninstall
    pub fn uninstall(&mut self, filename: &str) -> bool {
        let mut is_unistall = false;

        self.mods.retain_mut(|module| {
            if module.get_filename() == filename {
                is_unistall = true;
                module.fini_array();
                kernel().debug().output(
                    DebugLevel::Lv2,
                    &format!("{} module uninstall successful", filename),
                );
                false
            } else {
                true
            }
        });

        if !is_unistall {
            kernel()
                .debug()
                .error(&format!("{} module not found", filename));
            return false;
        }
        true
    }

    // Get modules
    pub fn get_modules(&mut self) -> &mut LinkedList<Box<ModLoader>> {
        &mut self.mods
    }
}
