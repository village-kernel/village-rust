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
use crate::kernel;
use crate::traits::vk_kernel::DebugLevel;
use crate::traits::vk_linkedlist::LinkedList;
use crate::traits::vk_filesys::FileMode;
use crate::misc::fopts::vk_file_fopt::FileFopt;
use super::vk_elf_defines::{DynamicType, DynamicHeader, RelocationCode, RelocationEntry, to_function};

// Const members
const SEG_BASE: usize = 16;

// Flag RecordType
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RecordType(u8);

// Impl RecordType
impl RecordType {
    pub const DATA: Self              = RecordType(0);
    pub const END_OF_FILE: Self       = RecordType(1);
    pub const EXT_SEG_ADDR: Self      = RecordType(2);
    pub const START_SEG_ADDR: Self    = RecordType(3);
    pub const EXT_LINEAR_ADDR: Self   = RecordType(4);
    pub const START_LINEAR_ADDR: Self = RecordType(4);

    // Contains
    pub fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    // Insert
    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0
    }

    // as u8
    pub fn as_u8(self) -> u8 {
        self.0
    }
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

// Struct Hex
struct Hex {
    text: String,
    data: Vec<u8>,

    load: u32,
    base: u32,
    exec: u32,

    offset: u32,
    dynamic: u32,
    entry: u32,
}

// Impl Hex
impl Hex {
    // New
    pub const fn new() -> Self {
        Self {
            text: String::new(),
            data: Vec::new(),

            load: 0,
            base: 0,
            exec: 0,

            offset: 0,
            dynamic: 0,
            entry: 0,
        }
    }
}

// Struct HexLoader
pub struct HexLoader {
    hex: Hex,
    filename: String,
}

// Impl HexLoader
impl HexLoader {
    // New
    pub const fn new() -> Self {
        Self {
            hex: Hex::new(),
            filename: String::new(),
        }
    }

    // Load
    pub fn load(&mut self, filename: &str) -> bool {
        //Save filename in local
        self.filename = filename.to_string();

        // Load and mapping
        if !self.load_hex()     { return false; }
        if !self.load_program() { return false; }
        if !self.clean_up()     { return false; }
        if !self.post_parser()  { return false; }
        if !self.rel_entries()  { return false; }

        // Output debug info
        kernel().debug().output(DebugLevel::Lv2, &format!("load at 0x{:08x}, {} load done", self.hex.base, self.filename));
        true
    }

    // Load hex
    fn load_hex(&mut self) -> bool {
        let mut file = FileFopt::new();
        
        if file.open(&self.filename, FileMode::READ) {
            let size = file.size();
            let mut buff = vec![0u8; size];

            if file.read(&mut buff, size, 0) == size {
                self.hex.text = String::from_utf8(buff).unwrap();
                kernel().debug().output(DebugLevel::Lv1, &format!("{} hex file load successful", self.filename));
                file.close();
                return true;
            }
            
            file.close();
        }

        kernel().debug().error(&format!("{} no such file!", self.filename));
        false
    }

    // load_program
    fn load_program(&mut self) -> bool {
        // Records
        let mut records: LinkedList<Record> = LinkedList::new();

        // Split text into record strings
        let record_strs: Vec<&str> = self.hex.text.split(":").collect();

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
                if record.typ == RecordType::DATA.as_u8() {
                    data_size = segment + record.addr as usize + record.len as usize;
                }
                // Caclutate segment 
                else if record.typ == RecordType::EXT_SEG_ADDR.as_u8() {
                    segment += u16::from_str_radix(&record.data[0..4], 16).unwrap() as usize * SEG_BASE;
                }
                // Clear segment and break
                else if record.typ == RecordType::END_OF_FILE.as_u8() {
                    segment = 0;
                    break;
                }

                // Add record into list
                records.add(record);
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

        // Start addr
        let mut start_addr: usize = 0;

        // Alloc hex data space
        if self.hex.data.len() == 0 {
            start_addr = records.iter_mut().nth(0).unwrap().addr as usize;
            self.hex.data = vec![0u8; data_size - start_addr];
        }

        // Load program
        for record in records.iter_mut() {
            // Load data
            if record.typ == RecordType::DATA.as_u8() {
                for pos in 0..record.len as usize {
                    let offset = pos * 2;
                    let value = u8::from_str_radix(&record.data[offset..offset+2], 16).unwrap();
                    let addr = (record.addr as usize + segment + pos) - start_addr;
                    self.hex.data[addr] = value;
                }
            } else if record.typ == RecordType::EXT_SEG_ADDR.as_u8() {
                segment += u16::from_str_radix(&record.data[0..4], 16).unwrap() as usize * SEG_BASE;
            }
        }

        // Clear records
        records.clear();
        true
    }

    // Clean up
    fn clean_up(&mut self) -> bool {
        self.hex.text = String::new();
        true
    }

    // Post parser
    fn post_parser(&mut self) -> bool {
        if self.hex.data.len() < 12 {
            return false;
        }

        self.hex.load = self.hex.data.as_ptr() as u32;
        self.hex.offset = u32::from_le_bytes(self.hex.data[0..4].try_into().unwrap());
        self.hex.dynamic = u32::from_le_bytes(self.hex.data[4..8].try_into().unwrap());
        self.hex.entry = u32::from_le_bytes(self.hex.data[8..12].try_into().unwrap());

        self.hex.base = self.hex.load - self.hex.offset;
        self.hex.exec = self.hex.base + self.hex.entry;

        true
    }

    // Rel entries
    fn rel_entries(&mut self) -> bool {
        let mut relcount: u32 = 0;
        let mut relocate: Option<u32> = None;
        
        // Calc dynamic section offset in hex data
        let dynamic_start = (self.hex.dynamic - self.hex.offset) as usize;
        if dynamic_start + 8 > self.hex.data.len() { return false; }

        // Gets dynamic bytes from hex data
        let dynamic_bytes = &self.hex.data[dynamic_start..];
        
        // Gets the relocate section address and the relcount
        let mut i = 0;
        loop {
            // Calc dynamic offset
            let dynamic_offset = i * 8;
            if dynamic_offset + 8 > dynamic_bytes.len() { break; }
            
            // Convert bytes into dynamic header
            let dynamic = DynamicHeader::from(&dynamic_bytes[dynamic_offset..dynamic_offset+8]);
            
            // Get relocate section
            if dynamic.tag == DynamicType::DT_REL.as_u32() {
                relocate = Some(dynamic.val);
            } else if dynamic.tag == DynamicType::DT_RELCOUNT.as_u32() {
                relcount = dynamic.val;
            } else if dynamic.tag == DynamicType::DT_NULL.as_u32() {
                 break;
            }
            
            i += 1;
        }
        
        // Check if relocation is needed
        if relocate.is_none() && relcount == 0 { return true; }
        if relocate.is_none() || relcount == 0 { return false; }
        
        // Calc relocate start offset
        let relocate_start = (relocate.unwrap() - self.hex.offset) as usize;

        // Relocate the value of relative type
        for i in 0..relcount {
            let relocate_offset = relocate_start + (i * 8) as usize;
            if relocate_offset + 8 > self.hex.data.len() { continue; }
            
            let relocate_entry = RelocationEntry::from(&self.hex.data[relocate_offset..relocate_offset+8]);
            
            if relocate_entry.typ == RelocationCode::TYPE_RELATIVE.as_u8() {
                let rel_addr_offset = (relocate_entry.offset - self.hex.offset) as usize;
                if rel_addr_offset + 4 > self.hex.data.len() { continue; }
                
                // Read original relative value
                let original_relative = u32::from_le_bytes(
                    self.hex.data[rel_addr_offset..rel_addr_offset+4].try_into().unwrap()
                );
                
                // Calc relocated value, absolute address
                let absolute_addr = self.hex.base + original_relative;
                
                // Write relocated value back
                let absolute_bytes = absolute_addr.to_le_bytes();
                self.hex.data[rel_addr_offset..rel_addr_offset+4].copy_from_slice(&absolute_bytes);
            }
        }
        
        true
    }

    // Execute
    pub fn execute(&mut self, argv: Vec<&str>) -> bool {
        let _ = argv;
        if self.hex.exec != 0 {
            (to_function(self.hex.exec))();
            kernel().debug().output(DebugLevel::Lv2, &format!("{} exit", self.filename));
            return true;
        }
        kernel().debug().error(&format!("{} execute failed!", self.filename));
        false
    }

    // Exit
    pub fn exit(&mut self) -> bool {
        false
    }
}

// Impl Drop for HexLoader
impl Drop for HexLoader {
    fn drop(&mut self) {
        self.exit();
    }
}
