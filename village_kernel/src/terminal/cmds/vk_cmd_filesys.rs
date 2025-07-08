//###########################################################################
// vk_cmd_filesys.rs
// The specific implementation of functions related to cmd filesys
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::fopts::vk_dir_fopt::DirFopt;
use crate::misc::fopts::vk_file_fopt::FileFopt;
use crate::misc::fopts::vk_filesys_fopt::FilesysFopt;
use crate::register_cmd;
use crate::traits::vk_command::{Cmd, Console};
use crate::traits::vk_filesys::FileAttr;
use crate::traits::vk_filesys::FileDir;
use crate::traits::vk_filesys::FileMode;
use crate::traits::vk_filesys::FileType;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// Struct cmd cd
struct CmdCd;

// Impl cmd cd
impl CmdCd {
    // Change directory
    fn change_directory(&mut self, console: &mut dyn Console, path: &str) {
        let mut dir = DirFopt::new();

        if dir.is_exist(path) {
            let mut new_path = path.to_string();

            if let Some(last_slash_pos) = path.rfind('/') {
                let dir_part = &path[last_slash_pos..];

                // Handle "." dir
                if dir_part == "/." {
                    new_path.truncate(last_slash_pos);
                }
                // Handle ".." dir
                else if dir_part == "/.." {
                    new_path.truncate(last_slash_pos);
                    if let Some(prev_slash) = new_path.rfind('/') {
                        // If we're at root, keep the slash, otherwise truncate
                        if prev_slash == 0 {
                            new_path.truncate(1);
                        } else {
                            new_path.truncate(prev_slash);
                        }
                    }
                }
            }

            console.set_path(&new_path);
        } else {
            console.error(&format!(
                "{} is not a valid path, please confirm whether the path is correct",
                path
            ));
        }
    }
}

// Impl cmd for cmd cd
impl Cmd for CmdCd {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: cd <directory>");
            return;
        }

        let path = console.real_path(argv[1]);
        self.change_directory(console, &path);
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd cd: change directory");
    }
}

// Struct cmd list
struct CmdList;

// Impl cmd list
impl CmdList {
    // List directory
    fn list_directory(&mut self, console: &mut dyn Console, path: &str) {
        let mut dir = DirFopt::new();

        if dir.open(path, FileMode::READ) {
            let size = dir.size();
            let mut dirs = vec![FileDir::new(); size];

            if dir.read(&mut dirs, size) == size {
                for i in 0..size {
                    if dirs[i].attr == FileAttr::Visible {
                        if FileType::Directory == dirs[i].typid
                            || FileType::File == dirs[i].typid
                        {
                            console.print(&format!("{}  ", dirs[i].name));
                        }
                    }
                }
                console.print("\r\n");
            }
        } else {
            console.error(&format!(
                "{} is not a valid path, please confirm whether the path is correct",
                path
            ));
        }
    }
}

// Impl cmd for cmd list
impl Cmd for CmdList {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 1 {
            console.println("Usage: ls [directory]");
            return;
        }

        let path: String;
        if argv.len() == 1 {
            path = console.get_path().to_string();
        } else {
            path = console.real_path(argv[1]);
        }
        self.list_directory(console, &path);
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd ls: list directory");
    }
}

// Struct cmd touch
struct CmdTouch;

// Impl cmd touch
impl CmdTouch {
    // Create file
    fn create_file(&mut self, console: &mut dyn Console, path: &str) {
        let mut file = FileFopt::new();

        if !file.is_exist(path) {
            if !file.open(path, FileMode::CREATE_NEW) {
                console.error(&format!("Create file {} failed", path));
            }
        } else {
            console.error(&format!("The file {} already exists", path));
        }
    }
}

// Impl cmd for cmd touch
impl Cmd for CmdTouch {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: touch <filename>");
            return;
        }

        let path = console.real_path(argv[1]);
        self.create_file(console, &path);
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd touch: create file");
    }
}

// Struct cmd mkdir
struct CmdMkdir;

// Impl cmd mkdir
impl CmdMkdir {
    // Create directory
    fn create_dir(&mut self, console: &mut dyn Console, path: &str) {
        let mut dir = DirFopt::new();

        if !dir.is_exist(path) {
            if !dir.open(path, FileMode::CREATE_NEW) {
                console.error(&format!("Create directory {} failed", path));
            }
        } else {
            console.error(&format!("The directory {} already exists", path));
        }
    }
}

// Impl cmd for cmd mkdir
impl Cmd for CmdMkdir {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: mkdir <dirname>");
            return;
        }

        let path = console.real_path(argv[1]);
        self.create_dir(console, &path);
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd mkdir: create directory");
    }
}

// Struct cmd move
struct CmdMove;

// Impl cmd move
impl CmdMove {
    // Move
    fn moving(&mut self, console: &mut dyn Console, source: &str, target: &str) {
        let mut filesys_opt = FilesysFopt::new();

        if !filesys_opt.moving(source, target) {
            console.error(&format!("move {} to {} failed!", source, target));
        }
    }
}

// Impl cmd for cmd move
impl Cmd for CmdMove {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 3 {
            console.println("Usage: mv <source> <target>");
            return;
        }

        let path1 = console.real_path(argv[1]);
        let path2 = console.real_path(argv[2]);
        self.moving(console, &path1, &path2);
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd mv: move file or directory");
    }
}

// Struct cmd copy
struct CmdCopy;

// Impl cmd copy
impl CmdCopy {
    // Change directory
    fn copy(&mut self, console: &mut dyn Console, source: &str, target: &str) {
        let mut filesys_opt = FilesysFopt::new();

        if !filesys_opt.copy(source, target) {
            console.error(&format!("copy {} to {} failed!", source, target));
        }
    }
}

// Impl cmd for cmd copy
impl Cmd for CmdCopy {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 3 {
            console.println("Usage: cp <source> <target>");
            return;
        }

        let path1 = console.real_path(argv[1]);
        let path2 = console.real_path(argv[2]);
        self.copy(console,&path1, &path2);
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd cp: copy file or directory");
    }
}

// Struct cmd remove
struct CmdRemove;

// Impl cmd remove
impl CmdRemove {
    // Remove
    fn remove(&mut self, console: &mut dyn Console, path: &str) {
        let mut filesys_opt = FilesysFopt::new();

        if !filesys_opt.remove(path) {
            console.error(&format!("Remove {} failed!", path));
        }
    }
}

// Impl cmd for cmd remove
impl Cmd for CmdRemove {
    // Execute
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        if argv.len() < 2 {
            console.println("Usage: rm <file/directory>");
            return;
        }

        let path = console.real_path(argv[1]);
        self.remove(console, &path);
    }

    // Help
    fn help(&mut self, console: &mut dyn Console) {
        console.println("cmd rm: remove file or directory");
    }
}

// Register cmd
register_cmd!(CmdCd, cd);
register_cmd!(CmdList, ls);
register_cmd!(CmdTouch, touch);
register_cmd!(CmdMkdir, mkdir);
register_cmd!(CmdMove, mv);
register_cmd!(CmdCopy, cp);
register_cmd!(CmdRemove, rm);
