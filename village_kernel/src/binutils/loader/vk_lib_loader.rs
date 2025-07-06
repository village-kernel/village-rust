//###########################################################################
// vk_lib_loader.rs
// The specific implementation of functions related to lib loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::string::{String, ToString};

// Struct LibLoader
pub struct LibLoader {
    filename: String,
    is_ignore_unresolved_symbols: bool,
}

// Impl LibLoader
impl LibLoader {
    // New
    pub const fn new() -> Self {
        Self {
            filename: String::new(),
            is_ignore_unresolved_symbols: false,
        }
    }

    // Load
    pub fn load(&mut self, filename: &str) -> bool {
        self.filename = filename.to_string();
        false
    }

    // Fill bss zero
    pub fn fill_bss_zero(&mut self) {}

    // Init array
    pub fn init_array(&mut self) {}

    // Fini array
    pub fn fini_array(&mut self) {}

    // Exit
    pub fn exit(&mut self) -> bool {
        false
    }
}

// Impl LibLoader
impl LibLoader {
    // Get filename
    pub fn get_filename(&mut self) -> &str {
        &self.filename
    }

    // Get dynamic string
    pub fn get_dynamic_string(&mut self, index: usize) -> &str {
        let _ = index;
        "none"
    }

    // Get section name
    pub fn get_section_name(&mut self, index: usize) -> &str {
        let _ = index;
        "none"
    }

    // Get symbol name
    pub fn get_symbol_name(&mut self, index: usize) -> &str {
        let _ = index;
        "none"
    }

    // Get dyn sym name
    pub fn get_dyn_sym_name(&mut self, index: usize) -> &str {
        let _ = index;
        "none"
    }

    // Get symbol addr
    pub fn get_symbol_addr(&mut self, index: usize) -> usize {
        let _ = index;
        0
    }

    // Get dym sym addr
    pub fn get_dym_sym_addr(&mut self, index: usize) -> usize {
        let _ = index;
        0
    }

    // Get symbol addr by name
    pub fn get_symbol_addr_by_name(&mut self, symbol: &str) -> usize {
        let _ = symbol;
        0
    }

    // Get dym sym addr by name
    pub fn get_dym_sym_addr_by_name(&mut self, symbol: &str) -> usize {
        let _ = symbol;
        0
    }

    // Ignore unresolved symbols
    pub fn ignore_unresolved_symbols(&mut self, enable: bool) {
        self.is_ignore_unresolved_symbols = enable;
    }
}
