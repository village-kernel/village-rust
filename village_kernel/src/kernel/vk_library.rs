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
use alloc::format;
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
        kernel().debug().info("Library setup completed!");
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
                kernel().debug().warning(&format!("{} has already been installed!", path));
                return true;
            }
        }

        // Install library if it has not install
        let mut lib = LibraryData::new();

        // Set the path
        lib.path = path.to_string();

        // Create loader
        lib.loader = kernel().director().create_loader(path);
        if lib.loader.is_none() {
            kernel().debug().error(&format!("{} unsupported file type!", path));
            return false;
        }

        // Init library
        if !lib.loader.as_mut().unwrap().init(path) {
            kernel().debug().error(&format!("{} install failed!", path));
            return false;
        }

        // Add into list
        self.libs.push(lib);

        // Output debug info
        kernel().debug().info(&format!("{} install successful!", path));

        true
    }

    // Uninstall
    fn uninstall(&mut self, path: &str) -> bool {
        let mut is_unistall = false;

        self.libs.retain_mut(|lib| {
            if lib.path == path {
                is_unistall = true;
                lib.loader.as_mut().unwrap().exit();
                kernel().debug().info(&format!("{} uninstall successful!", path));
                false
            } else {
                true
            }
        });

        if !is_unistall {
            kernel().debug().error(&format!("{} library not found!", path));
            return false;
        }
        
        true
    }

    // Search symbol
    fn search(&mut self, symbol: &str) -> usize {
        for lib in self.libs.iter_mut() {
            let addr = lib.loader.as_mut().unwrap().get(symbol);
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
