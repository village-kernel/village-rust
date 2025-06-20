//###########################################################################
// vk_rc_parser.rs
// The specific implementation of functions related to rc parser
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::string::String;
use crate::traits::vk_linkedlist::LinkedList;
use crate::traits::vk_filesys::FileMode;
use crate::misc::fopts::vk_file_fopt::FileFopt;

// Struct RcParser
pub struct RcParser {
    runcmds: LinkedList<String>,
}

// Impl RcParser
impl RcParser {
    // New
    pub const fn new() -> Self {
        Self {
            runcmds: LinkedList::new(),
        }
    }

    // Load
    pub fn load(&mut self, filename: &str) -> bool {
        let mut file = FileFopt::new();
        
        if file.open(filename, FileMode::Read) {
        
            self.decode("shabi");
        }

        false
    }

    // Decode
    fn decode(&mut self, rcstring: &str) {
        let _ = rcstring;
    }

    // Get run cmds
    pub fn get_run_cmds(&mut self) -> LinkedList<&str> {
        self.runcmds.iter_mut().map(|s| s.as_str()).collect()
    }
}
