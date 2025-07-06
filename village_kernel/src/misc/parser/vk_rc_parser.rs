//###########################################################################
// vk_rc_parser.rs
// The specific implementation of functions related to rc parser
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::misc::fopts::vk_file_fopt::FileFopt;
use crate::traits::vk_filesys::FileMode;
use crate::traits::vk_linkedlist::LinkedList;
use alloc::string::{String, ToString};
use alloc::vec;

// Enum ParserStatus
enum ParserStatus {
    RecordCmd,
    SaveCmd,
    NotRecord,
}

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
        let mut result = false;

        if file.open(filename, FileMode::READ) {
            let size = file.size();
            let mut data = vec![0u8; size];

            if file.read(&mut data, size, 0) == size {
                self.decode(&String::from_utf8_lossy(&data));
                result = true;
            }

            file.close();
        }

        result
    }

    // Decode
    fn decode(&mut self, rc_string: &str) {
        const START_DELIMITER: isize = -1;

        let mut status = ParserStatus::RecordCmd;
        let mut start_index = START_DELIMITER;
        let mut record_bytes = 0;

        for (i, byte) in rc_string.chars().enumerate() {
            match byte {
                '#' => {
                    status = ParserStatus::NotRecord;
                }
                ' ' => {
                    if let ParserStatus::RecordCmd = status {
                        status = ParserStatus::SaveCmd;
                    }
                }
                '\r' => continue,
                '\n' => match status {
                    ParserStatus::RecordCmd => status = ParserStatus::SaveCmd,
                    ParserStatus::NotRecord => status = ParserStatus::RecordCmd,
                    _ => {}
                },
                _ => {
                    if let ParserStatus::RecordCmd = status {
                        if byte > ' ' && byte <= '~' {
                            if START_DELIMITER == start_index {
                                start_index = i as isize;
                            }
                            record_bytes += 1;
                        }
                    }
                }
            }

            // Save cmd
            if matches!(status, ParserStatus::SaveCmd) && START_DELIMITER != start_index {
                if let Some(start) = usize::try_from(start_index).ok() {
                    let cmd = rc_string[start..start + record_bytes].to_string();

                    // Reset state
                    status = ParserStatus::RecordCmd;
                    start_index = START_DELIMITER;
                    record_bytes = 0;

                    // Add cmd to run cmd list
                    self.runcmds.push(cmd);
                }
            }
        }
    }

    // Get run cmds
    pub fn get_run_cmds(&mut self) -> LinkedList<&str> {
        self.runcmds.iter_mut().map(|s| s.as_str()).collect()
    }
}
