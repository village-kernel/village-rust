//###########################################################################
// vk_cmd_cat.rs
// The specific implementation of functions related to cmd cat
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::fopts::vk_file_fopt::FileFopt;
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::traits::vk_filesys::FileMode;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

// Struct cmd cat
struct CmdCat;

// Impl cmd cat
impl CmdCat {
    // Cat
    fn cat(&mut self, console: &mut dyn Console, path: &str) {
        let mut file = FileFopt::new();

        if file.open(path, FileMode::READ) {
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

// Impl cmd for cmd cat
impl Cmd for CmdCat {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 1 {
            console.println("Usage: cat <filename>");
            return;
        }

        let path = console.real_path(argv[1]);
        self.cat(console, &path);
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd cat: concatenate and print files");
    }
}

// Register cmd
register_cmd!(CmdCat, cat);
