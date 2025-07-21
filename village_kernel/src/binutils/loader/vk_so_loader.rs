//###########################################################################
// vk_so_loader.rs
// The specific implementation of functions related to so loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::decoder::vk_elf_defines::{ELFClass, ELFVersion, ELFMachine, ELFType};
use crate::binutils::decoder::vk_elf_defines::{ELFHeader, ProgHdrType, ProgramHeader};
use crate::binutils::decoder::vk_elf_defines::{SectionHdrType, SectionHeader};
use crate::binutils::decoder::vk_elf_defines::{DynamicHeader, DynamicType, RelocateCode};
use crate::binutils::decoder::vk_elf_defines::{RelocateEntry, SymbolEntry};
use crate::misc::fopts::vk_file_fopt::FileFopt;
use crate::traits::vk_builder::LibLoader;
use crate::traits::vk_filesys::FileMode;
use crate::traits::vk_kernel::DebugLevel;
use crate::village::kernel;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// Struct SoLoader
pub struct SoLoader {
    elf: Vec<u8>,
    prog: Vec<u8>,

    hdr: ELFHeader,

    load: u32,
    base: u32,
    dynamic: u32,
    dynamicsz: u32,

    rel: u32,
    relsz: u32,
    relent: u32,
    relcount: u32,

    jmprel: u32,
    pltrelsz: u32,
    pltgot: u32,
    pltcount: u32,

    symtab: u32,
    syment: u32,
    symcount: u32,
    strtab: u32,
    strsz: u32,

    filename: String,
    is_ignore_unresolved_symbols: bool,
}

// Impl SoLoader
impl SoLoader {
    // New
    pub const fn new() -> Self {
        Self {
            elf: Vec::new(),
            prog: Vec::new(),

            hdr: ELFHeader::new(),

            load: 0,
            base: 0,
            dynamic: 0,
            dynamicsz: 0,

            rel: 0,
            relsz: 0,
            relent: 0,
            relcount: 0,

            jmprel: 0,
            pltrelsz: 0,
            pltgot: 0,
            pltcount: 0,

            symtab: 0,
            syment: 0,
            symcount: 0,
            strtab: 0,
            strsz: 0,

            filename: String::new(),
            is_ignore_unresolved_symbols: true,
        }
    }

    // Load elf
    fn load_elf(&mut self) -> bool {
        let mut file = FileFopt::new();
        let mut result = false;

        if file.open(&self.filename, FileMode::READ) {
            let size = file.size();
            self.elf = vec![0u8; size];
            result = file.read(&mut self.elf, size, 0) == size;
            file.close();
        }

        if !result {
            kernel()
                .debug()
                .error(&format!("{} no such file!", self.filename));
        }

        result
    }

    // Check elf
    fn check_elf(&mut self) -> bool {
        // Set elf header
        self.hdr = ELFHeader::from(&self.elf[0..ELFHeader::SIZE]);

        // Check if it is a valid elf file
        let elf_magic: [u8; 4] = [0x7f, b'E', b'L', b'F'];
        if self.hdr.ident[0] != elf_magic[0] {
            return false;
        }
        if self.hdr.ident[1] != elf_magic[1] {
            return false;
        }
        if self.hdr.ident[2] != elf_magic[2] {
            return false;
        }
        if self.hdr.ident[3] != elf_magic[3] {
            return false;
        }
        if self.hdr.ident[4] != ELFClass::X32 {
            return false;
        }
        if self.hdr.version != ELFVersion::CURRENT {
            return false;
        }

        // Check machine type
        #[cfg(feature = "arch_i386")]
        if self.hdr.machine != ELFMachine::X86 {
            return false;
        }
        #[cfg(feature = "arch_arm")]
        if self.elf.hdr.machine != ELFMachine::ARM {
            return false;
        }

        // Check elf type
        if self.hdr.typ != ELFType::DYN {
            kernel().debug().error(&format!(
                "{} is not Position-Independent Executable file",
                self.filename
            ));
            return false;
        }

        // Output debug info
        kernel().debug().output(
            DebugLevel::Lv1,
            &format!("{} pre parser successful", self.filename),
        );
        true
    }

    // Pre load
    fn pre_load(&mut self) -> bool {
        //Parser section headers
        for i in 0..self.hdr.sect_hdr_num as usize {
            let sect_start = self.hdr.sect_hdr_off as usize + i * self.hdr.sect_hdr_size as usize;
            let sect_end = sect_start + self.hdr.sect_hdr_size as usize;
            let shdr = SectionHeader::from(&self.elf[sect_start..sect_end]);

            // Get the dynmaic offset and size
            if shdr.typ == SectionHdrType::SHT_DYNAMIC {
                self.dynamic = shdr.offset;
                self.dynamicsz = shdr.size;
            }
        }

        true
    }

    // Load prog
    fn load_prog(&mut self) -> bool {
        // Program headers
        let mut phdrs: Vec<ProgramHeader> = Vec::new();

        // Prog size
        let mut prog_size = 0;

        // To estimate how much memory space this program needs.
        for i in 0..self.hdr.prog_hdr_num as usize {
            let prog_start = self.hdr.prog_hdr_off as usize + i * self.hdr.prog_hdr_size as usize;
            let prog_end = prog_start + self.hdr.prog_hdr_size as usize;
            let phdr = ProgramHeader::from(&self.elf[prog_start..prog_end]);

            // Overwrite the previously obtained data
            if phdr.typ == ProgHdrType::PT_LOAD {
                let need_size = (phdr.vaddr + phdr.mem_size) + (phdr.align - 1);
                let align_size = need_size / phdr.align * phdr.align;
                if prog_size < align_size {
                    prog_size = align_size;
                };
            }
            // Get the dynmaic offset
            else if phdr.typ == ProgHdrType::PT_DYNAMIC {
                self.dynamic = phdr.vaddr;
            }

            // Add phdr into list
            phdrs.push(phdr);
        } 

        // Return false when phdrs is empty
        if phdrs.len() == 0 {
            kernel().debug().error(&format!(
                "{} elf file no valid program section",
                self.filename
            ));
            return false;
        }

        // Allocate the memory space required by the program
        self.prog = vec![0u8; prog_size as usize];

        // Load the program from the ELF file
        for phdr in phdrs.iter_mut() {
            for i in 0..phdr.mem_size {
                let vaddr = (phdr.vaddr + i) as usize;
                let offset = (phdr.offset + i) as usize;
                self.prog[vaddr] = self.elf[offset];
            }
        }

        true
    }

    // post_load
    fn post_load(&mut self) -> bool {
        // Set data
        self.load = self.prog.as_ptr() as u32;
        self.base = self.load;

        // Set elf header
        self.hdr = ELFHeader::from(&self.prog[0..ELFHeader::SIZE]);

        // Gets the relocate section address and the relcount
        let mut i = 0;
        loop {
            // Calc dynamic entry offset
            let dhdr_offset = self.dynamic as usize + i * 8;
            if dhdr_offset + 8 > self.prog.len() {
                break;
            }

            // Convert bytes into dynamic header
            let dhdr = DynamicHeader::from(&&self.prog[dhdr_offset..dhdr_offset + 8]);

            // Get info
            if dhdr.tag == DynamicType::DT_REL {
                self.rel = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_RELSZ {
                self.relsz = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_RELENT {
                self.relent = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_RELCOUNT {
                self.relcount = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_JMPREL {
                self.jmprel = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_PLTGOT {
                self.pltgot = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_PLTRELSZ {
                self.pltrelsz = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_SYMTAB {
                self.symtab = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_SYMENT {
                self.syment = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_STRTAB {
                self.strtab = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_STRSZ {
                self.strsz = dhdr.val;
            } else if dhdr.tag == DynamicType::DT_NEEDED {
                self.load_needed_lib(dhdr.val);
            } else if dhdr.tag == DynamicType::DT_NULL {
                break;
            }

            i += 1;
        }

        // Calc symcount
        self.symcount = self.dynamicsz / self.syment;

        // Calc pltcount
        self.pltcount = self.pltrelsz / self.relent;

        true
    }

    // load needed lib
    fn load_needed_lib(&mut self, val: u32) {
        let name = self.get_symbol_name(val as usize);
        let path = format!("/libraries/{}", name);

        if !kernel().library().install(&path) {
            kernel().debug().error(
                &format!("{} load shared object {} failed", self.filename, path)
            );
        }
    }
}

// Impl SoLoader
impl SoLoader {
    // Relocation symbol call
    fn rel_sym_call(&mut self, rel_addr: u32, sym_addr: u32, typ: u8, size: u32) {
        unsafe {
            let a = *(rel_addr as *const u32);
            let b = self.base;
            let g = 0;
            let got = 0;
            let l = 0;
            let z = 0;
            let p = rel_addr;
            let s = sym_addr;
            let rel_val = rel_addr as *mut u32;

            match typ {
                RelocateCode::I386_32 => 
                    *rel_val = s + a,

                RelocateCode::I386_PC32 =>
                    *rel_val = s + a - p,

                RelocateCode::I386_GOT32 =>
                    *rel_val = g + a,

                RelocateCode::I386_PLT32 =>
                    *rel_val = l + a - p,

                RelocateCode::I386_COPY => {
                    let src = s as *const u8;
                    let dst: *mut u8 = rel_val as *mut u8;
                    let count = size as usize;
                    core::ptr::copy_nonoverlapping(src, dst, count);
                },
                
                RelocateCode::I386_GLOB_DAT =>
                    *rel_val = s,

                RelocateCode::I386_JMP_SLOT =>
                    *rel_val = s,

                RelocateCode::I386_RELATIVE =>
                    *rel_val = b + a,

                RelocateCode::I386_GOTOFF =>
                    *rel_val = s + a + got,

                RelocateCode::I386_GOTPC =>
                    *rel_val = got + a - p,

                RelocateCode::I386_32PLT =>
                    *rel_val = l + a,

                RelocateCode::I386_16 =>
                    *rel_val = s + a,

                RelocateCode::I386_PC16 =>
                    *rel_val = s + a - p,

                RelocateCode::I386_8 =>
                    *rel_val = s + a,

                RelocateCode::I386_PC8 =>
                    *rel_val = s + a - p,

                RelocateCode::I386_SIZE32 =>
                    *rel_val = z + a,

                _ => {}
            }
        }
    }

    // rel_symbol
    fn rel_symbol(&mut self, rel: u32, count: u32) -> bool {
        // Check if relocation is needed
        if rel == 0 && count == 0 {
            return true;
        }
        if rel == 0 || count == 0 {
            return false;
        }

        // Calc relocate start offset
        let rel_off = rel as usize;

        // Relocate the value of relative type
        for i in 0..count {
            // Get rel entry offset
            let relent_off = rel_off + (i * 8) as usize;
            if relent_off + 8 > self.prog.len() {
                continue;
            }

            // Get relocation entry
            let rel_bytes = &self.prog[relent_off..relent_off + 8];
            let rel_entry = RelocateEntry::from(rel_bytes);

            // Get symbol entry
            let sym_entry = self.get_symbol_entry(rel_entry.symbol as usize);

            // Get symbol entry name
            let sym_name = self.get_symbol_name(sym_entry.name as usize);

            // Get rel addr and sym addr
            let rel_addr = self.base + rel_entry.offset;
            let mut sym_addr = 0;

            // Get the address of symbol entry when the relocation entry type is relative
            if rel_entry.typ == RelocateCode::TYPE_RELATIVE {
                sym_addr = self.base;
            }

            // Get the address of symbol entry when the relocation entry type is copy
            if rel_entry.typ == RelocateCode::TYPE_COPY {
                sym_addr = kernel().library().search_symbol(&sym_name) as u32;
            }

            // Get the address of object symbol entry
            if 0 == sym_addr && 0 != sym_entry.shndx  {
                sym_addr = self.base + sym_entry.value;
            }

            // Get the address of undefined symbol entry
            if 0 == sym_addr {
                sym_addr = kernel().symbol().search(&sym_name);
            }

            // Searching for symbol entry in libraries
            if 0 == sym_addr {
                sym_addr = kernel().library().search_symbol(&sym_name) as u32;
            }

            // Return when sym addr is 0
            if sym_addr == 0 {
                if self.is_ignore_unresolved_symbols {
                    kernel().debug().warning(
                        &format!("{} relocation symbols ignore, symbol {} not found", self.filename, sym_name)
                    );
                } else {
                    kernel().debug().error(
                        &format!("{} relocation symbols failed, symbol {} not found", self.filename, sym_name)
                    );
                }
            }

            // Relocation symbol
            self.rel_sym_call(rel_addr, sym_addr, rel_entry.typ, sym_entry.size);

            // Output debug message
            kernel().debug().output(DebugLevel::Lv0, &format!(
                    "{} rel name {}, relAddr 0x{:08x}, symAddr 0x{:08x}", 
                    self.filename, sym_name, rel_addr, sym_addr
            ));
        }

        // Output debug message
        kernel().debug().output(DebugLevel::Lv1, &format!(
            "{} relocation entries successful", self.filename
        ));
        true
    }

    // Relocate
    fn relocate(&mut self) -> bool {
        // rel.dyn
        if !self.rel_symbol(self.rel, self.relcount) {
            return false;
        }

        // rel.plt
        if !self.rel_symbol(self.jmprel, self.pltcount) {
            return false;
        }

        true
    }
}

// Impl SoLoader
impl SoLoader {
    // Ignore unresolved symbols
    pub fn ignore_unresolved_symbols(&mut self, enable: bool) {
        self.is_ignore_unresolved_symbols = enable;
    }

    // Get symbol entry
    fn get_symbol_entry(&mut self, ndx: usize) -> SymbolEntry {
        let symbol_start = self.symtab as usize + ndx * self.syment as usize;
        let symbol_end = symbol_start + self.syment as usize;
        SymbolEntry::from(&self.prog[symbol_start..symbol_end])
    }

    // Get dyn sym name
    fn get_symbol_name(&mut self, name: usize) -> String {
        let name_start = self.strtab as usize + name;
        let name_end = name_start + self.prog[name_start..].iter().position(|v| *v == 0).unwrap();
        String::from_utf8_lossy(&self.prog[name_start..name_end]).to_string()
    }

    // Get dym sym addr by name
    fn get_symbool_addr(&mut self, symbol: &str) -> usize {
        for i in 0..self.symcount as usize {
            let entry = self.get_symbol_entry(i);
            let name = self.get_symbol_name(entry.name as usize);
            if symbol == name {
                return(self.base + entry.value) as usize;
            }
        }
        0
    }
}

// Impl LibLoader for SoLoader
impl LibLoader for SoLoader {
    // Init
    fn init(&mut self, filename: &str) -> bool {
        // Save filename in local
        self.filename = filename.to_string();

        if !self.load_elf() {
            return false;
        }
        if !self.check_elf() {
            return false;
        }
        if !self.pre_load() {
            return false;
        }
        if !self.load_prog() {
            return false;
        }
        if !self.post_load() {
            return false;
        }
        if !self.relocate() {
            return false;
        }
        
        true
    }

    // Get
    fn get(&mut self, symbol: &str) -> usize {
        self.get_symbool_addr(symbol)
    }
    
    // Exit
    fn exit(&mut self) -> bool {
        self.elf.clear();
        self.prog.clear();
        self.elf.shrink_to_fit();
        self.prog.shrink_to_fit();
        true
    }
}
