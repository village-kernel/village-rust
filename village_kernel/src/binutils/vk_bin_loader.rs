//###########################################################################
// vk_bin_loader.rs
// The specific implementation of functions related to bin loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use crate::kernel;
use crate::traits::vk_kernel::DebugLevel;
use crate::traits::vk_filesys::FileMode;
use crate::misc::fopts::vk_file_fopt::FileFopt;
use super::vk_elf_defines::{DynamicType, DynamicHeader, RelocationCode, RelocationEntry, to_function};

// Struct Bin 
struct Bin {
    prog: Vec<u8>,

    load: u32,
    base: u32,
    exec: u32,

    offset: u32,
    dynamic: u32,
    entry: u32,
}

// Impl Bin
impl Bin {
    // New
    pub const fn new() -> Self {
        Self {
            prog: Vec::new(),
            
            load: 0,
            base: 0,
            exec: 0,

            offset: 0,
            dynamic: 0,
            entry: 0,
        }
    }
}

// Struct BinLoader
pub struct BinLoader {
    bin: Bin,
    filename: String,
}

// Impl BinLoader
impl BinLoader {
    // New
    pub const fn new() -> Self {
        Self {
            bin: Bin::new(),
            filename: String::new(),
        }
    }

    // Load
    pub fn load(&mut self, filename: &str) -> bool {
        //Save filename in local
        self.filename = filename.to_string();

        // Load and mapping
        if !self.load_bin()    { return false; }
        if !self.post_parser() { return false; }
        if !self.rel_entries() { return false; }

        // Output debug info
        kernel().debug().output(DebugLevel::Lv2, &format!("load at 0x{:08x}, {} load done", self.bin.base, self.filename));
        true
    }

    // Load bin
    fn load_bin(&mut self) -> bool {
        let mut file = FileFopt::new();
        
        if file.open(&self.filename, FileMode::READ) {
            let size = file.size();
            self.bin.prog = vec![0u8; size];
            
            if file.read(&mut self.bin.prog, size, 0) == size {
                kernel().debug().output(DebugLevel::Lv1, &format!("{} bin file load successful", self.filename));
                file.close();
                return true;
            }
            
            file.close();
        }

        kernel().debug().error(&format!("{} no such file!", self.filename));
        false
    }

    // Post parser
    fn post_parser(&mut self) -> bool {
        if self.bin.prog.len() < 12 {
            return false;
        }

        self.bin.load = self.bin.prog.as_ptr() as u32;
        self.bin.offset = u32::from_le_bytes(self.bin.prog[0..4].try_into().unwrap());
        self.bin.dynamic = u32::from_le_bytes(self.bin.prog[4..8].try_into().unwrap());
        self.bin.entry = u32::from_le_bytes(self.bin.prog[8..12].try_into().unwrap());
        
        self.bin.base = self.bin.load - self.bin.offset;
        self.bin.exec = self.bin.base + self.bin.entry;

        true
    }

    // Rel entries
    fn rel_entries(&mut self) -> bool {
        let mut relcount: u32 = 0;
        let mut relocate: Option<u32> = None;
        
        // Calc dynamic section offset in bin data
        let dynamic_start = (self.bin.dynamic - self.bin.offset) as usize;
        if dynamic_start + 8 > self.bin.prog.len() { return false; }

        // Gets dynamic bytes from bin data
        let dynamic_bytes = &self.bin.prog[dynamic_start..];
        
        // Gets the relocate section address and the relcount
        let mut i = 0;
        loop {
            // Calc dynamic offset
            let dynamic_offset = i * 8;
            if dynamic_offset + 8 > dynamic_bytes.len() { break; }
            
            // Convert bytes into dynamic header
            let dynamic = DynamicHeader::from(&dynamic_bytes[dynamic_offset..dynamic_offset+8]);
            
            // Get relocate section
            if dynamic.tag == DynamicType::DT_REL {
                relocate = Some(dynamic.val);
            } else if dynamic.tag == DynamicType::DT_RELCOUNT {
                relcount = dynamic.val;
            } else if dynamic.tag == DynamicType::DT_NULL {
                 break;
            }
            
            i += 1;
        }
        
        // Check if relocation is needed
        if relocate.is_none() && relcount == 0 { return true; }
        if relocate.is_none() || relcount == 0 { return false; }
        
        // Calc relocate start offset
        let relocate_start = (relocate.unwrap() - self.bin.offset) as usize;

        // Relocate the value of relative type
        for i in 0..relcount {
            let relocate_offset = relocate_start + (i * 8) as usize;
            if relocate_offset + 8 > self.bin.prog.len() { continue; }
            
            let relocate_entry = RelocationEntry::from(&self.bin.prog[relocate_offset..relocate_offset+8]);
            
            if relocate_entry.typ == RelocationCode::TYPE_RELATIVE {
                let rel_addr_offset = (relocate_entry.offset - self.bin.offset) as usize;
                if rel_addr_offset + 4 > self.bin.prog.len() { continue; }
                
                // Read original relative value
                let original_relative = u32::from_le_bytes(
                    self.bin.prog[rel_addr_offset..rel_addr_offset+4].try_into().unwrap()
                );
                
                // Calc relocated value, absolute address
                let absolute_addr = self.bin.base + original_relative;
                
                // Write relocated value back
                let absolute_bytes = absolute_addr.to_le_bytes();
                self.bin.prog[rel_addr_offset..rel_addr_offset+4].copy_from_slice(&absolute_bytes);
            }
        }
        
        true
    }

    // Execute
    pub fn execute(&mut self, argv: Vec<&str>) -> bool {
        let _ = argv;
        if self.bin.exec != 0 {
            (to_function(self.bin.exec))();
            kernel().debug().output(DebugLevel::Lv2, &format!("{} exit", self.filename));
            return true;
        }
        kernel().debug().error(&format!("{} execute failed!", self.filename));
        false
    }

    // Exit
    pub fn exit(&mut self) -> bool {
        self.bin.prog.clear();
        self.bin.prog.shrink_to_fit();
        true
    }
}

// Impl Drop for BinLoader
impl Drop for BinLoader {
    fn drop(&mut self) {
        self.exit();
    }
}
