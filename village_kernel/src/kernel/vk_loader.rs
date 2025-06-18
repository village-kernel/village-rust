use crate::misc::parser::vk_rc_parser::RcParser;
//###########################################################################
// vk_loader.rs
// The specific implementation of functions related to loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Loader;
use crate::binutils::vk_library_tool::LibraryTool;
use crate::binutils::vk_module_tool::ModuleTool;

// Struct concrete loader
pub struct ConcreteLoader {
    libtool: LibraryTool,
    modtool: ModuleTool,
}

// Impl concrete loader
impl ConcreteLoader {
    pub const fn new() -> Self {
        Self { 
            libtool: LibraryTool::new(),
            modtool: ModuleTool::new(),
        }
    }
}

// Impl concrete loader
impl ConcreteLoader {
    // Setup
    pub fn setup(&mut self) {
        // Loading libraries
        self.load_libs("/libraries/_load_.rc");

        // Loading modules
        self.load_mods("/modules/_load_.rc");

        // Output debug info
        kernel().debug().info("Loader setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Unloading modules
        self.unload_mods("/modules/_load_.rc");

        // Unloading modules
        self.unload_libs("/libraries/_load_.rc");
    }
}

// Impl concrete loader
impl ConcreteLoader {
    // Load libs
    fn load_libs(&mut self, filename: &str) {
        let mut parser = RcParser::new();
        
        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for lib_name in run_cmds.rev_iter_mut() {
                if !self.install_lib(lib_name) {
                    break;
                }
            }
        }
    }

    // Unload libs
    fn unload_libs(&mut self, filename: &str) {
        let mut parser = RcParser::new();
        
        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for lib_name in run_cmds.iter_mut() {
                if !self.uninstall_lib(lib_name) {
                    break;
                }
            }
        }
    }

    // Load mods
    fn load_mods(&mut self, filename: &str) {
        let mut parser = RcParser::new();
        
        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for mod_name in run_cmds.iter_mut() {
                self.install_mod(mod_name);
            }
        }
    }

    // Unload mods
    fn unload_mods(&mut self, filename: &str) {
        let mut parser = RcParser::new();
        
        if parser.load(filename) {
            let mut run_cmds = parser.get_run_cmds();

            for mod_name in run_cmds.rev_iter_mut() {
                self.uninstall_mod(mod_name);
            }
        }
    }
}

// Impl loader for concrete loader
impl Loader for ConcreteLoader {
    // Install lib
    fn install_lib(&mut self, name: &str) -> bool {
        self.libtool.install(name)
    }

    // Unistall lib
    fn uninstall_lib(&mut self, name: &str) -> bool {
        self.libtool.uninstall(name)
    }

    // Search symbol
    fn search_symbol(&mut self, symbol: &str) -> usize {
        self.libtool.search_symbol(symbol)
    }

    // Install mod
    fn install_mod(&mut self, name: &str) -> bool {
        self.modtool.install(name)
    }

    // Uninstall mod
    fn uninstall_mod(&mut self, name: &str) -> bool {
        self.modtool.uninstall(name)
    }
}
