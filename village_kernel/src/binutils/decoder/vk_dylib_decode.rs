//###########################################################################
// vk_dylib_decode.rs
// The specific implementation of functions related to dylib decode
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::decoder::vk_defs_elf::*;
use crate::traits::vk_builder::LibDecoder;
use crate::traits::vk_kernel::DebugLevel;
use crate::village::kernel;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// Struct DylibDecoder
pub struct DylibDecoder {
    data: Vec<u8>,

    hdr: ELFHeader,

    load: u32,
    base: u32,
    dynamic: u32,

    rel: u32,
    relsz: u32,
    relent: u32,
    relcount: u32,

    jmprel: u32,
    pltrelsz: u32,
    pltgot: u32,
    pltcount: u32,

    symtab: u32,
    symtabsz: u32,
    syment: u32,
    symcount: u32,
    strtab: u32,
    strsz: u32,

    filename: String,
    is_ignore_unresolved_symbols: bool,
}

// Impl DylibDecoder
impl DylibDecoder {
    // New
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),

            hdr: ELFHeader::new(),

            load: 0,
            base: 0,
            dynamic: 0,

            rel: 0,
            relsz: 0,
            relent: 0,
            relcount: 0,

            jmprel: 0,
            pltrelsz: 0,
            pltgot: 0,
            pltcount: 0,

            symtab: 0,
            symtabsz: 0,
            syment: 0,
            symcount: 0,
            strtab: 0,
            strsz: 0,

            filename: String::new(),
            is_ignore_unresolved_symbols: true,
        }
    }

    // Decode
    fn decode(&mut self, data: Vec<u8>) -> bool {
        // Set data
        self.data = data;
        self.load = self.data.as_ptr() as u32;
        self.base = self.load;

        // Set elf header
        self.hdr = ELFHeader::from(&self.data[0..ELFHeader::SIZE]);

        // Get dynmaic offset
        for i in 0..self.hdr.prog_hdr_num as usize {
            let prog_start = self.hdr.prog_hdr_off as usize + i * self.hdr.prog_hdr_size as usize;
            let prog_end = prog_start + self.hdr.prog_hdr_size as usize;
            let phdr = ProgramHeader::from(&&self.data[prog_start..prog_end]);

            if phdr.typ == ProgHdrType::PT_DYNAMIC {
                self.dynamic = phdr.vaddr;
                break;
            }
        }

        // Create dt neededs vector
        let mut dt_neededs = Vec::new();

        // Gets the relocate section address and the relcount
        let mut i = 0;
        loop {
            // Calc dynamic entry offset
            let dhdr_offset = self.dynamic as usize + i * 8;
            if dhdr_offset + 8 > self.data.len() {
                break;
            }

            // Convert bytes into dynamic header
            let dhdr = DynamicHeader::from(&self.data[dhdr_offset..dhdr_offset + 8]);

            // Get info
            match dhdr.tag {
                DynamicType::DT_NEEDED   => dt_neededs.push(dhdr),
                DynamicType::DT_REL      => self.rel      = dhdr.val,
                DynamicType::DT_RELSZ    => self.relsz    = dhdr.val,
                DynamicType::DT_RELENT   => self.relent   = dhdr.val,
                DynamicType::DT_RELCOUNT => self.relcount = dhdr.val,
                DynamicType::DT_JMPREL   => self.jmprel   = dhdr.val,
                DynamicType::DT_PLTGOT   => self.pltgot   = dhdr.val,
                DynamicType::DT_PLTRELSZ => self.pltrelsz = dhdr.val,
                DynamicType::DT_SYMTAB   => self.symtab   = dhdr.val,
                DynamicType::DT_SYMENT   => self.syment   = dhdr.val,
                DynamicType::DT_STRTAB   => self.strtab   = dhdr.val,
                DynamicType::DT_STRSZ    => self.strsz    = dhdr.val,
                DynamicType::DT_NULL     => break,
                _ => {}
            }

            i += 1;
        }

        // Load needed libs
        for dhdr in dt_neededs.iter_mut() {
            self.load_needed_lib(dhdr.val);
        }

        // Calc symtabsz 
        // Force the strtab section to be after 
        // the symtab section in the linked file
        self.symtabsz = self.strtab - self.symtab;

        // Calc symcount
        if self.syment != 0 {
            self.symcount = self.symtabsz / self.syment;
        }

        // Calc pltcount
        if self.relent != 0 {
            self.pltcount = self.pltrelsz / self.relent;
        }

        true
    }

    // Load needed lib
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

// Impl DylibDecoder
impl DylibDecoder {
    // Relocation symbol call
    fn rel_sym_call(&mut self, rel_addr: u32, sym_addr: u32, typ: u8, size: u32) {
        unsafe {
            let a = *(rel_addr as *const u32);
            let b = self.base;
            let got = self.pltgot;
            let g = rel_addr;
            let l = sym_addr;
            let z = size;
            let p = rel_addr;
            let s = sym_addr;
            let rel_ptr = rel_addr as *mut u32;

            match typ {
                RelocateCode::I386_32       => *rel_ptr = s + a,
                RelocateCode::I386_PC32     => *rel_ptr = s + a - p,
                RelocateCode::I386_GOT32    => *rel_ptr = g + a,
                RelocateCode::I386_PLT32    => *rel_ptr = l + a - p,
                RelocateCode::I386_COPY     => {
                    let src = s as *const u8;
                    let dst = rel_ptr as *mut u8;
                    let count = size as usize;
                    core::ptr::copy_nonoverlapping(src, dst, count);
                },
                RelocateCode::I386_GLOB_DAT => *rel_ptr = s,
                RelocateCode::I386_JMP_SLOT => *rel_ptr = s,
                RelocateCode::I386_RELATIVE => *rel_ptr = b + a,
                RelocateCode::I386_GOTOFF   => *rel_ptr = s + a + got,
                RelocateCode::I386_GOTPC    => *rel_ptr = got + a - p,
                RelocateCode::I386_32PLT    => *rel_ptr = l + a,
                RelocateCode::I386_16       => *rel_ptr = s + a,
                RelocateCode::I386_PC16     => *rel_ptr = s + a - p,
                RelocateCode::I386_8        => *rel_ptr = s + a,
                RelocateCode::I386_PC8      => *rel_ptr = s + a - p,
                RelocateCode::I386_SIZE32   => *rel_ptr = z + a,
                _ => {}
            }
        }
    }

    // rel_symbol
    fn rel_symbol(&mut self, rel: u32, count: u32) -> bool {
        // Check if relocate is needed
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
            if relent_off + 8 > self.data.len() {
                continue;
            }

            // Get relocate entry
            let rel_bytes = &self.data[relent_off..relent_off + 8];
            let rel_entry = RelocateEntry::from(rel_bytes);

            // Get symbol entry
            let sym_entry = self.get_symbol_entry(rel_entry.symbol as usize);

            // Get symbol entry name
            let sym_name = self.get_symbol_name(sym_entry.name as usize);

            // Get rel addr and sym addr
            let rel_addr = self.base + rel_entry.offset;
            let mut sym_addr = 0;

            // Get the address of symbol entry when the relocate entry type is relative
            if rel_entry.typ == RelocateCode::TYPE_RELATIVE {
                sym_addr = self.base;
            }

            // Get the address of symbol entry when the relocate entry type is copy
            if rel_entry.typ == RelocateCode::TYPE_COPY {
                sym_addr = kernel().library().search(&sym_name) as u32;
            }

            // Get the address of object symbol entry
            if 0 == sym_addr && 0 != sym_entry.shndx  {
                sym_addr = self.base + sym_entry.value;
            }

            // Get the address of undefined symbol entry
            if 0 == sym_addr {
                sym_addr = kernel().symbol().search(&sym_name) as u32;
            }

            // Searching for symbol entry in libraries
            if 0 == sym_addr {
                sym_addr = kernel().library().search(&sym_name) as u32;
            }

            // Return when sym addr is 0
            if sym_addr == 0 {
                if self.is_ignore_unresolved_symbols {
                    kernel().debug().warning(
                        &format!("{} relocate symbol ignore, {} not found", self.filename, sym_name)
                    );
                } else {
                    kernel().debug().error(
                        &format!("{} relocate symbol failed, {} not found", self.filename, sym_name)
                    );
                }
            }

            // Relocate symbol
            self.rel_sym_call(rel_addr, sym_addr, rel_entry.typ, sym_entry.size);

            // Output debug message
            kernel().debug().output(DebugLevel::Lv0, &format!(
                    "{} rel name {}, relAddr 0x{:08x}, symAddr 0x{:08x}", 
                    self.filename, sym_name, rel_addr, sym_addr
            ));
        }

        // Output debug message
        kernel().debug().output(DebugLevel::Lv1, &format!(
            "{} relocate entries successful", self.filename
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

// Impl DylibDecoder
impl DylibDecoder {
    // Ignore unresolved symbols
    pub fn ignore_unresolved_symbols(&mut self, enable: bool) {
        self.is_ignore_unresolved_symbols = enable;
    }

    // Get symbol entry
    fn get_symbol_entry(&mut self, ndx: usize) -> SymbolEntry {
        let symbol_start = self.symtab as usize + ndx * self.syment as usize;
        let symbol_end = symbol_start + self.syment as usize;
        SymbolEntry::from(&self.data[symbol_start..symbol_end])
    }

    // Get dyn sym name
    fn get_symbol_name(&mut self, name: usize) -> String {
        let name_start = self.strtab as usize + name;
        let name_end = name_start + self.data[name_start..].iter().position(|v| *v == 0).unwrap();
        String::from_utf8_lossy(&self.data[name_start..name_end]).to_string()
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

// Impl LibDecoder for DylibDecoder
impl LibDecoder for DylibDecoder {
    // Init
    fn init(&mut self, path: &str, data: Vec<u8>) -> bool {
        self.filename = path.to_string();

        if !self.decode(data) {
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
        self.data.clear();
        self.data.shrink_to_fit();
        true
    }
}
