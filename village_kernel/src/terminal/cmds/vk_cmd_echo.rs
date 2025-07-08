//###########################################################################
// vk_cmd_echo.rs
// The specific implementation of functions related to cmd echo
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::fopts::vk_file_fopt::FileFopt;
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::traits::vk_filesys::FileMode;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Struct cmd echo
struct CmdEcho;

// Impl cmd echo
impl CmdEcho {
    // Echo
    fn echo(&mut self, console: &mut dyn Console, size: usize, data: Vec<&str>, mode: &str, path: &str) {
        if path != "" {
            // Set mode
            let mut filemode = FileMode::CREATE_NEW;

            if mode == ">" {
                filemode.insert(FileMode::WRITE);
            } else if mode == ">>" {
                filemode.insert(FileMode::OPEN_APPEND);
            } else {
                console.error("parse error near \'\n\'");
                return;
            }

            // Set path
            let filepath = console.real_path(path);

            // Write data
            let mut file = FileFopt::new();

            if file.open(&filepath, filemode) {
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

// Impl cmd for cmd echo
impl Cmd for CmdEcho {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: echo <string> [>]|[>>] [path]");
            return;
        }

        if argv.len() >= 4 {
            self.echo(
                console,
                argv.len() - 3,
                argv[1..].to_vec(),
                argv[argv.len() - 2],
                argv[argv.len() - 1],
            );
        } else {
            self.echo(console, argv.len() - 1, argv[1..].to_vec(), "", "");
        }
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd echo: write arguments to the standard output");
    }
}

// Register cmd
register_cmd!(CmdEcho, echo);
