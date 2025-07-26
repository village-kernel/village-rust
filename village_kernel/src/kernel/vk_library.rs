//###########################################################################
// vk_library.rs
// The specific implementation of functions related to library
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::parser::vk_rc_parser::RcParser;
use crate::traits::vk_kernel::{LibraryData, Library};
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;
use crate::debug_error;
use crate::debug_info;
use crate::debug_warning;
use alloc::string::ToString;

// Struct village library
pub struct VillageLibrary {
    libs: LinkedList<LibraryData>,
}

// Impl village library
impl VillageLibrary {
    // New
    pub const fn new() -> Self {
        Self {
            libs: LinkedList::new(),
        }
    }
}

// Impl village library
impl VillageLibrary {
    // Setup
    pub fn setup(&mut self) {
        // Loading libraries
        self.load_libs("/libraries/_load_.rc");

        // Output debug info
        debug_info!("Library setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Unloading libraries
        self.unload_libs("/libraries/_load_.rc");
    }
}

// Impl village library
impl VillageLibrary {
    // Load libs
    fn load_libs(&mut self, filename: &str) {
        let mut parser = RcParser::new();

        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for lib_name in run_cmds.iter_mut().rev() {
                self.install(lib_name);
            }
        }
    }

    // Unload libs
    fn unload_libs(&mut self, filename: &str) {
        let mut parser = RcParser::new();

        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for lib_name in run_cmds.iter_mut() {
                self.uninstall(lib_name);
            }
        }
    }
}

// Impl library for village library
impl Library for VillageLibrary {
    // Install
    fn install(&mut self, path: &str) -> bool {
        // Check the library if it has been installed
        for lib in self.libs.iter_mut() {
            if lib.path == path {
                debug_warning!("{} has already been installed!", path);
                return true;
            }
        }

        // Install library if it has not install
        let mut lib = LibraryData::new();

        // Set the path
        lib.path = path.to_string();

        // Create loader
        lib.container = kernel().director().create_lib_container(path);
        if lib.container.is_none() {
            debug_error!("{} unsupported file type!", path);
            return false;
        }

        // Init library
        if !lib.container.as_mut().unwrap().init(path) {
            debug_error!("{} install failed!", path);
            return false;
        }

        // Add into list
        self.libs.push(lib);

        // Output debug info
        debug_info!("{} install successful!", path);

        true
    }

    // Uninstall
    fn uninstall(&mut self, path: &str) -> bool {
        let mut is_unistall = false;

        self.libs.retain_mut(|lib| {
            if lib.path == path {
                is_unistall = true;
                lib.container.as_mut().unwrap().exit();
                debug_info!("{} uninstall successful!", path);
                false
            } else {
                true
            }
        });

        if !is_unistall {
            debug_error!("{} library not found!", path);
            return false;
        }
        
        true
    }

    // Search symbol
    fn search(&mut self, symbol: &str) -> usize {
        for lib in self.libs.iter_mut() {
            let addr = lib.container.as_mut().unwrap().get(symbol);
            if addr != 0 {
                return addr;
            }
        }
        0
    }

    // Get libraries
    fn get_libraries(&mut self) -> &mut LinkedList<LibraryData> {
        &mut self.libs
    }
}
