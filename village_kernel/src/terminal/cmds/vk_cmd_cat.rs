//###########################################################################
// vk_cmd_cat.rs
// The specific implementation of functions related to cmd cat
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::format;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_commad::{Cmd, CmdBase};
use crate::misc::fopts::vk_file_fopt::FileFopt;

// Struct cmd cat
struct CmdCat {
    base: CmdBase
}

// Impl cmd cat
impl CmdCat {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd cat
impl CmdCat {
    // Cat
    fn cat(&mut self, path: &str) {
        if let Some(console) = self.base.get_console() {
            let mut file = FileFopt::new();

            if file.open(path) {
                let size = file.size();
                let mut data = vec![0u8; size];

                if file.read(&mut data, size, 0) == size {
                    console.println(&String::from_utf8_lossy(&data));
                }
                
                file.close();
            } else {
                console.println(&format!("File {} not found", path));
            }
        }
    }
}

// Impl cmd for cmd cat
impl Cmd for CmdCat {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            if argv.len() < 1 {
                console.println("Usage: cat <filename>");
                return;
            }

            let path = console.absolute_path(argv[1]);
            self.cat(&path);
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd cat: concatenate and print files");
        }
    }
}

// Register cmd
register_cmd!(CmdCat::new(), cat);
