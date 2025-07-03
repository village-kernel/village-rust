//###########################################################################
// vk_library_tool.rs
// The specific implementation of functions related to library tool
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::format;
use alloc::boxed::Box;
use crate::village::kernel;
use crate::traits::vk_kernel::DebugLevel;
use crate::traits::vk_linkedlist::LinkedList;
use crate::binutils::loader::vk_lib_loader::LibLoader;

// Struct LibraryTool
pub struct LibraryTool {
    libs: LinkedList<Box<LibLoader>>,
}

// Impl LibraryTool
impl LibraryTool {
    // New
    pub const fn new() -> Self {
        Self {
            libs: LinkedList::new(),
        }
    }

    // Install
    pub fn install(&mut self, filename: &str) -> bool {
        // Check the library if it has been installed
        for lib in self.libs.iter_mut() {
            if lib.get_filename() == filename {
                kernel().debug().output(DebugLevel::Lv2, &format!("{} library has already been installed", filename));
                return true;
            }
        }
        
        // Install library if it has not install
        let mut lib = Box::new(LibLoader::new());

        // Ignore unresolved symbols
        lib.ignore_unresolved_symbols(true);

        // Load library
        if lib.load(filename) {
            lib.fill_bss_zero();
            lib.init_array();
            self.libs.push(lib);
            kernel().debug().output(DebugLevel::Lv2, &format!("{} library install successful", filename));
            return true;
        }

        // Install failed
        kernel().debug().error(&format!("{} library install failed", filename));
        false
    }

    // Uninstall
    pub fn uninstall(&mut self, filename: &str) -> bool {
        let mut is_unistall = false;

        self.libs.retain_mut(|lib| {
            if lib.get_filename() == filename {
                is_unistall = true;
                lib.fini_array();
                kernel().debug().output(DebugLevel::Lv2, &format!("{} library uninstall successful", filename));
                false
            } else {
                true
            }
        });

        if !is_unistall {
            kernel().debug().error(&format!("{} library not found", filename));
            return false;
        }
        true
    }

    // Search symbol
    pub fn search_symbol(&mut self, symbol: &str) -> usize {
        for lib in self.libs.iter_mut() {
            let addr = lib.get_dym_sym_addr_by_name(symbol);
            if addr != 0 {
                return addr;
            }
        }
        0
    }

    // Get libraries
    pub fn get_libraries(&mut self) -> &mut LinkedList<Box<LibLoader>> {
        &mut self.libs
    }
}
