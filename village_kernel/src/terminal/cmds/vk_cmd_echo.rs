//###########################################################################
// vk_cmd_echo.rs
// The specific implementation of functions related to cmd echo
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::register_cmd;
use crate::village::kernel;
use crate::traits::vk_command::{Cmd, CmdBase};
use crate::traits::vk_filesys::FileMode;
use crate::misc::fopts::vk_file_fopt::FileFopt;

// Struct cmd echo
struct CmdEcho {
    base: CmdBase
}

// Impl cmd echo
impl CmdEcho {
    // New
    pub const fn new() -> Self {
        Self {
            base: CmdBase::new(),
        }
    }
}

// Impl cmd echo
impl CmdEcho {
    // Echo
    fn echo(&mut self, size: usize, data: Vec<&str>, mode: &str, path: &str) {
        if let Some(console) = self.base.get_console() {
            if path != "" {
                // Set mode
                let mut filemode = FileMode::CreateNew as u8;

                if mode == ">" {
                    filemode |= FileMode::Write as u8;
                } else if mode == ">>" {
                    filemode |= FileMode::OpenAppend as u8;
                } else {
                    console.error("parse error near \'\n\'");
                    return;
                }

                // Set path
                let filepath = console.absolute_path(path);

                // Write data
                let mut file = FileFopt::new();

                if file.open(&filepath, FileMode::from(filemode)) {

                    if file.size() != 0 && mode == ">>" {
                        file.write("\r\n".as_bytes(), 3, 0);
                    }

                    for i in 0..size {
                        file.write(data[i].as_bytes(), data[i].len(), 0);
                        file.write(" ".as_bytes(), 1, 0);
                    }

                    file.flush();                   
                    file.close();
                }
            } else {
                console.println(data[0]);
            }
        }
    }
}

// Impl cmd for cmd echo
impl Cmd for CmdEcho {
    // Base
    fn base(&mut self) -> &mut CmdBase {
        &mut self.base
    }

    // Execute
    fn execute(&mut self, argv: Vec<&str>) {
        if let Some(console) = self.base.get_console() {
            if argv.len() < 2 {
                console.println("Usage: echo <string> [>]|[>>] [path]");
                return;
            }

            if argv.len() >= 4 {
                self.echo(argv.len() - 3, argv[1..].to_vec(), argv[argv.len() - 2], argv[argv.len() - 1]);
            } else {
                self.echo(argv.len() - 1, argv[1..].to_vec(), "", "");
            }
        }
    }
    
    // Help
    fn help(&mut self) {
        if let Some(console) = self.base.get_console() {
            console.println("cmd echo: write arguments to the standard output");
        }
    }
}

// Register cmd
register_cmd!(CmdEcho::new(), echo);
