//###########################################################################
// vk_bin_loader.rs
// The specific implementation of functions related to bin loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_prog_decode::Program;
use crate::misc::fopts::vk_file_fopt::FileFopt;
use crate::traits::vk_filesys::FileMode;
use crate::traits::vk_kernel::DebugLevel;
use crate::village::kernel;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// Struct BinLoader
pub struct BinLoader {
    filename: String,
    program: Program,
}

// Impl BinLoader
impl BinLoader {
    // New
    pub const fn new() -> Self {
        Self {
            filename: String::new(),
            program: Program::new(),
        }
    }

    // Load
    pub fn load(&mut self, filename: &str) -> bool {
        // Save filename in local
        self.filename = filename.to_string();

        // Load and mapping
        if !self.load_bin() {
            return false;
        }

        // Output debug info
        kernel().debug().output(
            DebugLevel::Lv2,
            &format!("{} load at 0x{:08x}", self.filename, self.program.base()),
        );
        true
    }

    // Load bin
    fn load_bin(&mut self) -> bool {
        let mut file = FileFopt::new();
        let mut data = Vec::new();
        let mut result = false;

        // Read bin file
        if file.open(&self.filename, FileMode::READ) {
            let size = file.size();
            data = vec![0u8; size];
            result = file.read(&mut data, size, 0) == size;
            file.close();
        }

        // Return false when read bin file failed
        if !result {
            kernel()
                .debug()
                .error(&format!("{} no such file!", self.filename));
            return false;
        }

        // Init program
        if !self.program.init(data) {
            kernel()
                .debug()
                .error(&format!("{} load failed!", self.filename));
            return false;
        }

        true
    }

    // Execute
    pub fn execute(&mut self, argv: Vec<&str>) -> bool {
        let result = self.program.execute(argv);

        if result {
            kernel()
                .debug()
                .output(DebugLevel::Lv2, &format!("{} exit", self.filename));
        } else {
            kernel()
                .debug()
                .error(&format!("{} execute failed!", self.filename));
        }

        result
    }

    // Exit
    pub fn exit(&mut self) -> bool {
        self.program.exit()
    }
}
