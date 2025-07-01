//###########################################################################
// vk_elf_loader.rs
// The specific implementation of functions related to elf loader
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
use super::vk_elf_defines::*;

// Struct Elf
struct Elf {
    data: Vec<u8>,
    prog: Vec<u8>,

    load: u32,
    base: u32,
    exec: u32,

    offset: u32,
    dynamic: u32,
    entry: u32,
    
    hdr: ELFHeader,
}

// Impl Elf
impl Elf {
    // New
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            prog: Vec::new(),

            load: 0,
            base: 0,
            exec: 0,

            offset: 0,
            dynamic: 0,
            entry: 0,

            hdr: ELFHeader::new(),
        }
    }
}

// Struct ElfLoader
pub struct ElfLoader {
    elf: Elf,
    filename: String,
}

// Impl ElfLoader
impl ElfLoader {
    // New
    pub const fn new() -> Self {
        Self {
            elf: Elf::new(),
            filename: String::new(),
        }
    }

    // Load
    pub fn load(&mut self, filename: &str) -> bool {
        // Save filename in local
        self.filename = filename.to_string();

        // Load and mapping
        if !self.load_elf()      { return false; }
        if !self.pre_parser()    { return false; }
        if !self.load_program()  { return false; }
        if !self.post_parser()   { return false; }
        if !self.rel_entries()   { return false; }

        // Output debug info
        kernel().debug().output(DebugLevel::Lv2, &format!("load at 0x{:08x}, {} load done", self.elf.base, self.filename));
        true
    }

    // Load elf
    fn load_elf(&mut self) -> bool {
        let mut file = FileFopt::new();
        
        if file.open(&self.filename, FileMode::READ) {
            let size = file.size();
            self.elf.data = vec![0u8; size];

            if file.read(&mut self.elf.data, size, 0) == size {
                kernel().debug().output(DebugLevel::Lv1, &format!("{} elf file load successful", self.filename));
                file.close();
                return true;
            }
            
            file.close();
        }

        kernel().debug().error(&format!("{} no such file!", self.filename));
        false
    }

    // Pre parser
    fn pre_parser(&mut self) -> bool {
        // Set elf header
        self.elf.hdr = ELFHeader::from(&self.elf.data[0..ELFHeader::SIZE]);

        // Check if it is a valid elf file
        let elf_magic: [u8; 4] = [0x7f, b'E', b'L', b'F'];
        if self.elf.hdr.ident[0] != elf_magic[0]        { return false; }
        if self.elf.hdr.ident[1] != elf_magic[1]        { return false; }
        if self.elf.hdr.ident[2] != elf_magic[2]        { return false; }
        if self.elf.hdr.ident[3] != elf_magic[3]        { return false; }
        if self.elf.hdr.ident[4] != ELFClass::X32       { return false; }
        if self.elf.hdr.version  != ELFVersion::CURRENT { return false; }

        // Check machine type
        #[cfg(feature = "arch_i386")]
        if self.elf.hdr.machine  != ELFMachine::X86     { return false; }
        #[cfg(feature = "arch_arm")]
        if self.elf.hdr.machine  != ELFMachine::ARM     { return false; }

        // Check elf type
        if self.elf.hdr.typ != ELFType::DYN {
            kernel().debug().error(&format!("{} is not Position-Independent Executable file", self.filename));
            return false;
        }

        // Set executable entry
        self.elf.exec = self.elf.hdr.entry;

        // Output debug info
        kernel().debug().output(DebugLevel::Lv1, &format!("{} pre parser successful", self.filename));
        true
    }

    // Load parogram
    fn load_program(&mut self) -> bool {
        // Programs
        let mut programs: Vec<ProgramHeader> = Vec::new();

        // Prog size
        let mut prog_size = 0;
        
        // To estimate how much memory space this program needs.
        for i in 0..self.elf.hdr.prog_hdr_num as usize {
            let prog_start = self.elf.hdr.prog_hdr_off as usize + i * ProgramHeader::SIZE;
            let prog_end = prog_start + ProgramHeader::SIZE;
            let program = ProgramHeader::from(&self.elf.data[prog_start..prog_end]);

            // Overwrite the previously obtained data
            if program.typ == ProgHdrType::PT_LOAD {
                prog_size = (program.vaddr + program.mem_size) + (program.align - 1);
                prog_size = prog_size / program.align * program.align;
            }

            // Add program into list
            programs.push(program);
        }

        // Return false when programs is empty
        if programs.len() == 0 {
            kernel().debug().error(&format!("{} elf file no valid program section", self.filename));
            return false;
        }

        // Allocate the memory space required by the program
        if self.elf.prog.len() == 0 {
            self.elf.prog = vec![0u8; prog_size as usize];
        }
        
        // Load the program from the ELF file
        for program in programs.iter_mut() {
            if program.typ == ProgHdrType::PT_LOAD {
                for i in 0..program.mem_size {
                    let vaddr = (program.vaddr + i) as usize;
                    let caddr = (program.offset + i) as usize;
                    self.elf.prog[vaddr] = self.elf.data[caddr];
                }
            }
        }

        // Clear programs
        programs.clear();
        programs.shrink_to_fit();
        
        // Output debug info
        kernel().debug().output(DebugLevel::Lv1, &format!("{} load program successful", self.filename));
        true
    }

    // Post parser
    fn post_parser(&mut self) -> bool {
        if self.elf.prog.len() < 12 {
            return false;
        }

        self.elf.load  = self.elf.prog.as_ptr() as u32;
        self.elf.offset = u32::from_le_bytes(self.elf.prog[0..4].try_into().unwrap());
        self.elf.dynamic = u32::from_le_bytes(self.elf.prog[4..8].try_into().unwrap());
        self.elf.entry = u32::from_le_bytes(self.elf.prog[8..12].try_into().unwrap());

        self.elf.base = self.elf.load - self.elf.offset;
        self.elf.exec = self.elf.base + self.elf.entry;

        true
    }

    // Rel entries
    fn rel_entries(&mut self) -> bool {
        let mut relcount: u32 = 0;
        let mut relocate: Option<u32> = None;
        
        // Calc dynamic section offset in hex data
        let dynamic_start = (self.elf.dynamic - self.elf.offset) as usize;
        if dynamic_start + 8 > self.elf.prog.len() { return false; }

        // Gets dynamic bytes from hex data
        let dynamic_bytes = &self.elf.prog[dynamic_start..];
        
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
        let relocate_start = (relocate.unwrap() - self.elf.offset) as usize;

        // Relocate the value of relative type
        for i in 0..relcount {
            let relocate_offset = relocate_start + (i * 8) as usize;
            if relocate_offset + 8 > self.elf.prog.len() { continue; }
            
            let relocate_entry = RelocationEntry::from(&self.elf.prog[relocate_offset..relocate_offset+8]);
            
            if relocate_entry.typ == RelocationCode::TYPE_RELATIVE {
                let rel_addr_offset = (relocate_entry.offset - self.elf.offset) as usize;
                if rel_addr_offset + 4 > self.elf.prog.len() { continue; }
                
                // Read original relative value
                let original_relative = u32::from_le_bytes(
                    self.elf.prog[rel_addr_offset..rel_addr_offset+4].try_into().unwrap()
                );
                
                // Calc relocated value, absolute address
                let absolute_addr = self.elf.base + original_relative;
                
                // Write relocated value back
                let absolute_bytes = absolute_addr.to_le_bytes();
                self.elf.prog[rel_addr_offset..rel_addr_offset+4].copy_from_slice(&absolute_bytes);
            }
        }
        
        true
    }

    // Execute
    pub fn execute(&mut self, argv: Vec<&str>) -> bool {
        let _ = argv;
        if self.elf.exec != 0 {
            (to_function(self.elf.exec))();
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

// Impl Drop for ElfLoader
impl Drop for ElfLoader {
    fn drop(&mut self) {
        self.exit();
    }
}
