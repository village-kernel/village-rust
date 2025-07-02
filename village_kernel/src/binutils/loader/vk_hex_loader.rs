//###########################################################################
// vk_hex_loader.rs
// The specific implementation of functions related to hex loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use crate::village::kernel;
use crate::traits::vk_kernel::DebugLevel;
use crate::traits::vk_filesys::FileMode;
use crate::misc::fopts::vk_file_fopt::FileFopt;
use super::vk_prog_decode::Program;

// Const members
const SEG_BASE: usize = 16;

// Flag RecordType
pub struct RecordType;

// Impl RecordType
impl RecordType {
    pub const DATA: u8              = 0;
    pub const END_OF_FILE: u8       = 1;
    pub const EXT_SEG_ADDR: u8      = 2;
    pub const START_SEG_ADDR: u8    = 3;
    pub const EXT_LINEAR_ADDR: u8   = 4;
    pub const START_LINEAR_ADDR: u8 = 4;
}

// Struct record
struct Record<'a> {
    len: u8,
    addr: u16,
    typ: u8,
    data: &'a str,
}

// Impl Record
impl<'a> Record<'a> {
    // New
    pub const fn new() -> Self {
        Self {
            len: 0,
            addr: 0,
            typ : 0,
            data: "",
        }
    }

    /// From string
    /// hex format:              |
    /// :llaaaatt[dd...]cc       |    type:
    /// [:]     : start flag     |    00 - Data
    /// [ll]    : length         |    01 - End Of File
    /// [aaaa]  : address        |    02 - Extended Segment Address
    /// [tt]    : type           |    03 - Start Segment Address
    /// [dd...] : data           |    04 - Extended Linear Address
    /// [cc]    : check          |    05 - Start Linear Address
    ///
    /// example:
    /// :0C00B400B4000000B4020000760200005E
    /// [:]   [0C]   [00 B4] [00] [B4 00 00 00 B4 02 00 00 76 02 00 00] [5E]
    /// start length address type data                                  check
    ///
    pub fn from(text: &'a str) -> Option<Self> {
        if !Self::check_sum(text) { return None; }

        let mut record = Self::new();

        record.len = u8::from_str_radix(&text[0..2], 16).unwrap();
        record.addr = u16::from_str_radix(&text[2..6], 16).unwrap();
        record.typ = u8::from_str_radix(&text[6..8], 16).unwrap();
        record.data = &text[8..record.len as usize * 2 + 8];

        Some(record)
    }

    // Check sum
    // len(1 byte) + addr(2 bype) + type(1 byte) + data(len byte)
    pub fn check_sum(text: &str) -> bool {
        let mut sum: u8 = 0;
        
        // Size of the record 
        let size = u8::from_str_radix(&text[0..2], 16).unwrap() as usize + 4;

        // Sum of all decoded byte values
        for pos in 0..size {
            sum = sum.wrapping_add(u8::from_str_radix(&text[pos*2..pos*2+2], 16).unwrap());
        }

        // Two's complement
        sum = (!sum).wrapping_add(1);

        // Crc of the record
        let crc_offset = size * 2;
        let crc = u8::from_str_radix(&text[crc_offset..crc_offset+2], 16).unwrap();

        sum == crc
    }
}

// Struct HexLoader
pub struct HexLoader {
    text: String,
    filename: String,
    program: Program,
}

// Impl HexLoader
impl HexLoader {
    // New
    pub const fn new() -> Self {
        Self {
            text: String::new(),
            filename: String::new(),
            program: Program::new(),
        }
    }

    // Load
    pub fn load(&mut self, filename: &str) -> bool {
        //Save filename in local
        self.filename = filename.to_string();

        // Load and mapping
        if !self.load_hex()     { return false; }
        if !self.load_program() { return false; }

        // Output debug info
        kernel().debug().output(DebugLevel::Lv2, &format!("{} load at 0x{:08x}", self.filename, self.program.base()));
        true
    }

    // Load hex
    fn load_hex(&mut self) -> bool {
        let mut file = FileFopt::new();
        let mut data = Vec::new();
        let mut result = false;
        
        if file.open(&self.filename, FileMode::READ) {
            let size = file.size();
            data = vec![0u8; size];
            result = file.read(&mut data, size, 0) == size;
            file.close();
        }

        if result {
            self.text = String::from_utf8(data).unwrap();
        } else {
            kernel().debug().error(&format!("{} no such file!", self.filename));
        }
        
        result
    }

    // load_program
    fn load_program(&mut self) -> bool {
        // Records
        let mut records: Vec<Record> = Vec::new();

        // Split text into record strings
        let record_strs: Vec<&str> = self.text.split(":").collect();

        // hex segment and data size
        let mut segment: usize = 0;
        let mut data_size: usize = 0;

        // Decode records
        for record_str in record_strs {
            // Skip empty record string
            if record_str.is_empty() { continue; }

            // Decode record
            if let Some(record) = Record::from(&record_str) {
                // Calculate data size
                if record.typ == RecordType::DATA {
                    data_size = segment + record.addr as usize + record.len as usize;
                }
                // Caclutate segment 
                else if record.typ == RecordType::EXT_SEG_ADDR {
                    segment += u16::from_str_radix(&record.data[0..4], 16).unwrap() as usize * SEG_BASE;
                }
                // Clear segment and break
                else if record.typ == RecordType::END_OF_FILE {
                    segment = 0;
                    break;
                }

                // Add record into list
                records.push(record);
            }
            // Reture false when decode failed
            else {
                kernel().debug().error(&format!("{} hex file pre parser failed", self.filename));
                return false;
            }
        }

        // Return false when records is empty
        if records.len() == 0 {
            kernel().debug().error(&format!("{} hex file no valid record", self.filename));
            return false;
        }

        // Allocate the memory space required by the program
        let start_addr = records[0].addr as usize;
        let mut data = vec![0u8; data_size - start_addr];
        
        // Load program data
        for record in records.iter_mut() {
            if record.typ == RecordType::DATA {
                for pos in 0..record.len as usize {
                    let offset = pos * 2;
                    let value = u8::from_str_radix(&record.data[offset..offset+2], 16).unwrap();
                    let addr = (record.addr as usize + segment + pos) - start_addr;
                    data[addr] = value;
                }
            } else if record.typ == RecordType::EXT_SEG_ADDR {
                segment += u16::from_str_radix(&record.data[0..4], 16).unwrap() as usize * SEG_BASE;
            }
        }

        // Init program
        if !self.program.init(data) {
            kernel().debug().error(&format!("{} program load failed", self.filename));
            return false;
        }

        true
    }

    // Execute
    pub fn execute(&mut self, argv: Vec<&str>) -> bool {
        let result = self.program.execute(argv);

        if result {
            kernel().debug().output(DebugLevel::Lv2, &format!("{} exit", self.filename));
        } else {
            kernel().debug().error(&format!("{} execute failed!", self.filename));
        }
        
        result
    }

    // Exit
    pub fn exit(&mut self) -> bool {
        self.program.exit()
    }
}
