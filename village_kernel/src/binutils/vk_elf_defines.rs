//###########################################################################
// vk_elf_defines.rs
// The specific implementation of functions related to elf defines
//
// $Copyright: Copyright (C) village
//###########################################################################

// Type aliases for function
pub type Function = extern "C" fn();

// Type aliases for start entry
pub type StartEntry = extern "C" fn(*mut(), usize, *mut *mut u8);

// Type aliase for function entry
pub type FuncEntry = extern "C" fn(usize, *mut *mut u8);

// Erase a function pointer to function
pub fn to_function(fn_addr: u32) -> Function {
    unsafe { core::mem::transmute::<u32, Function>( fn_addr ) }
}

// Erase a function pointer to a start entry
pub fn to_start_entry(fn_addr: u32) -> StartEntry {
    unsafe { core::mem::transmute::<u32, StartEntry>( fn_addr ) }
}

// Erase a function pointer to a function entry
pub fn to_func_entry(fn_addr: u32) -> FuncEntry {
    unsafe { core::mem::transmute::<u32, FuncEntry>( fn_addr ) }
}

// Flag ELFClass
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ELFClass(u8);

// Impl ELFClass
impl ELFClass {
    pub const NONE: Self  = ELFClass(0);
    pub const X32: Self   = ELFClass(1);
    pub const X64: Self   = ELFClass(2);

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

// Struct ELFType
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ELFType(u16);

// Impl ELFType
impl ELFType {
    pub const NONE: Self    = ELFType(0);
    pub const REL: Self     = ELFType(1);
    pub const EXEC: Self    = ELFType(2);
    pub const DYN: Self     = ELFType(3);
    pub const CORE: Self    = ELFType(4);
    pub const LO_PROC: Self = ELFType(5);
    pub const HI_PROC: Self = ELFType(6);

    // Contains
    pub fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    // Insert
    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0
    }

    // as u16
    pub fn as_u16(self) -> u16 {
        self.0
    }
}

// Struct ELFMachine
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ELFMachine(u16);

// Impl ELFMachine
impl ELFMachine {
    pub const NONE: Self     = ELFMachine(0x00);
    pub const X86: Self      = ELFMachine(0x03);
    pub const ARM: Self      = ELFMachine(0x28);
    pub const ARM_64: Self   = ELFMachine(0xb7);
    pub const RISC_V: Self   = ELFMachine(0xf3);

    // Contains
    pub fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    // Insert
    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0
    }

    // as u16
    pub fn as_u16(self) -> u16 {
        self.0
    }
}

// Struct ELFVersion
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ELFVersion(u32);

// Impl ELFVersion
impl ELFVersion {
    pub const NONE: Self     = ELFVersion(0x00);
    pub const CURRENT: Self  = ELFVersion(0x01);

    // Contains
    pub fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    // Insert
    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0
    }

    // as u32
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

// Struct RelocationCode
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RelocationCode(u8);

// Impl RelocationCode
#[cfg(feature = "arch_i386")]
impl RelocationCode {
    pub const I386_NONE: Self       = RelocationCode(0);
    pub const I386_32: Self         = RelocationCode(1);
    pub const I386_PC32: Self       = RelocationCode(2);
    pub const I386_GOT32: Self      = RelocationCode(3);
    pub const I386_PLT32: Self      = RelocationCode(4);
    pub const I386_COPY: Self       = RelocationCode(5);
    pub const TYPE_COPY: Self       = RelocationCode::I386_COPY;
    pub const I386_GLOB_DAT: Self   = RelocationCode(6);
    pub const I386_JMP_SLOT: Self   = RelocationCode(7);
    pub const I386_RELATIVE: Self   = RelocationCode(8);
    pub const TYPE_RELATIVE: Self   = RelocationCode::I386_RELATIVE;
    pub const I386_GOTOFF: Self     = RelocationCode(9);
    pub const I386_GOTPC: Self      = RelocationCode(10);
    pub const I386_32PLT: Self      = RelocationCode(11);
    pub const I386_16: Self         = RelocationCode(20);
    pub const I386_PC16: Self       = RelocationCode(21);
    pub const I386_8: Self          = RelocationCode(22);
    pub const I386_PC8: Self        = RelocationCode(23);
    pub const I386_SIZE32: Self     = RelocationCode(38);

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

// Impl RelocationCode
#[cfg(feature = "arch_arm")]
impl RelocationCode {
    pub const ARM_NONE: Self        = RelocationCode(0);
    pub const ARM_ABS32: Self       = RelocationCode(2);
    pub const ARM_THM_CALL: Self    = RelocationCode(10);
    pub const ARM_COPY: Self        = RelocationCode(20);
    pub const TYPE_COPY: Self       = RelocationCode::ARM_COPY;
    pub const ARM_GLOB_DAT: Self    = RelocationCode(21);
    pub const ARM_JUMP_SLOT: Self   = RelocationCode(22);
    pub const ARM_RELATIVE: Self    = RelocationCode(23);
    pub const TYPE_RELATIVE: Self   = RelocationCode::ARM_RELATIVE;
    pub const ARM_THM_JUMP24: Self  = RelocationCode(30);
    pub const ARM_TARGET1: Self     = RelocationCode(38);
    pub const ARM_THM_JUMP11: Self  = RelocationCode(102);

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

// Struct ProgHdrType
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ProgHdrType(u32);

// Impl ProgHdrType
impl ProgHdrType {
    pub const PT_NULL: Self           = ProgHdrType(0x00);
    pub const PT_LOAD: Self           = ProgHdrType(0x01);
    pub const PT_DYNAMIC: Self        = ProgHdrType(0x02);
    pub const PT_INTERP: Self         = ProgHdrType(0x03);
    pub const PT_NOTE: Self           = ProgHdrType(0x04);
    pub const PT_SHLIB: Self          = ProgHdrType(0x05);
    pub const PT_PHDR: Self           = ProgHdrType(0x06);
    pub const PT_TLS: Self            = ProgHdrType(0x07);
    pub const PT_NUM: Self            = ProgHdrType(0x08);
    pub const PT_LOOS: Self           = ProgHdrType(0x60000000);
    pub const PT_GNU_EH_FRAME: Self   = ProgHdrType(0x6474e550);
    pub const PT_GNU_STACK: Self      = ProgHdrType(0x6474e551);
    pub const PT_GNU_RELRO: Self      = ProgHdrType(0x6474e552);
    pub const PT_LOSUNW: Self         = ProgHdrType(0x6ffffffa);
    pub const PT_SUNWBSS: Self        = ProgHdrType(0x6ffffffa);
    pub const PT_SUNWSTACK: Self      = ProgHdrType(0x6ffffffb);
    pub const PT_HISUNW: Self         = ProgHdrType(0x6fffffff);
    pub const PT_HIOS: Self           = ProgHdrType(0x6fffffff);
    pub const PT_LOPROC: Self         = ProgHdrType(0x70000000);
    pub const PT_HIPROC: Self         = ProgHdrType(0x7fffffff);

    // Contains
    pub fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    // Insert
    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0
    }

    // as u32
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

// Struct SectionHdrType
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SectionHdrType(u32);

// Impl SectionHdrType
impl SectionHdrType {
    pub const SHT_NULL: Self          = SectionHdrType(0x00);
    pub const SHT_PROGBITS: Self      = SectionHdrType(0x01);
    pub const SHT_SYMTAB: Self        = SectionHdrType(0x02);
    pub const SHT_STRTAB: Self        = SectionHdrType(0x03);
    pub const SHT_RELA: Self          = SectionHdrType(0x04);
    pub const SHT_HASH: Self          = SectionHdrType(0x05);
    pub const SHT_DYNAMIC: Self       = SectionHdrType(0x06);
    pub const SHT_NOTE: Self          = SectionHdrType(0x07);
    pub const SHT_NOBITS: Self        = SectionHdrType(0x08);
    pub const SHT_REL: Self           = SectionHdrType(0x09);
    pub const SHT_SHLIB: Self         = SectionHdrType(0x0a);
    pub const SHT_DYNSYM: Self        = SectionHdrType(0x0b);
    pub const SHT_INIT_ARRAY: Self    = SectionHdrType(0x0e);
    pub const SHT_FINI_ARRAY: Self    = SectionHdrType(0x0f);
    pub const SHT_PREINIT_ARRAY: Self = SectionHdrType(0x10);
    pub const SHT_GROUP: Self         = SectionHdrType(0x11);
    pub const SHT_SYMTAB_SHNDX: Self  = SectionHdrType(0x12);
    pub const SHT_NUM: Self           = SectionHdrType(0x13);
    pub const SHT_LOOS: Self          = SectionHdrType(0x60000000);

    // Contains
    pub fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    // Insert
    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0
    }

    // as u32
    pub fn as_u32(self) -> u32 {
        self.0
    }
}


// Struct SymbolType
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SymbolType(u8);

// Impl SymbolType
impl SymbolType {
    pub const STT_NOTYPE: Self     = SymbolType(0);
    pub const STT_OBJECT: Self     = SymbolType(1);
    pub const STT_FUNC: Self       = SymbolType(2);
    pub const STT_SECTION: Self    = SymbolType(3);
    pub const STT_FILE: Self       = SymbolType(4);
    pub const STT_LOPROC: Self     = SymbolType(13);
    pub const STT_HIPROC: Self     = SymbolType(15);

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

// Struct SymbolBind
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SymbolBind(u8);

// Impl SymbolBind
impl SymbolBind {
    pub const STB_LOCAL: Self      = SymbolBind(0);
    pub const STB_GLOBAL: Self     = SymbolBind(1);
    pub const STB_WEAK: Self       = SymbolBind(2);
    pub const STB_LOPROC: Self     = SymbolBind(13);
    pub const STB_HIPROC: Self     = SymbolBind(15);

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

// Struct DynamicType
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DynamicType(u32);

// Impl DynamicType
impl DynamicType {
    pub const DT_NULL: Self        = DynamicType(0);
    pub const DT_NEEDED: Self      = DynamicType(1);
    pub const DT_PLTRELSZ: Self    = DynamicType(2);
    pub const DT_PLTGOT: Self      = DynamicType(3);
    pub const DT_HASH: Self        = DynamicType(4);
    pub const DT_STRTAB: Self      = DynamicType(5);
    pub const DT_SYMTAB: Self      = DynamicType(6);
    pub const DT_RELA: Self        = DynamicType(7);
    pub const DT_RELASZ: Self      = DynamicType(8);
    pub const DT_RELAENT: Self     = DynamicType(9);
    pub const DT_STRSZ: Self       = DynamicType(10);
    pub const DT_SYMENT: Self      = DynamicType(11);
    pub const DT_INIT: Self        = DynamicType(12);
    pub const DT_FINI: Self        = DynamicType(13);
    pub const DT_SONAME: Self      = DynamicType(14);
    pub const DT_RPATH: Self       = DynamicType(15);
    pub const DT_SYMBOLIC: Self    = DynamicType(16);
    pub const DT_REL: Self         = DynamicType(17);
    pub const DT_RELSZ: Self       = DynamicType(18);
    pub const DT_RELENT: Self      = DynamicType(19);
    pub const DT_PLTREL: Self      = DynamicType(20);
    pub const DT_DEBUG: Self       = DynamicType(21);
    pub const DT_TEXTREL: Self     = DynamicType(22);
    pub const DT_JMPREL: Self      = DynamicType(23);
    pub const DT_ENCODING: Self    = DynamicType(32);
    pub const OLD_DT_LOOS: Self    = DynamicType(0x60000000);
    pub const DT_LOOS: Self        = DynamicType(0x6000000d);
    pub const DT_HIOS: Self        = DynamicType(0x6ffff000);
    pub const DT_VALRNGLO: Self    = DynamicType(0x6ffffd00);
    pub const DT_VALRNGHI: Self    = DynamicType(0x6ffffdff);
    pub const DT_ADDRRNGLO: Self   = DynamicType(0x6ffffe00);
    pub const DT_ADDRRNGHI: Self   = DynamicType(0x6ffffeff);
    pub const DT_VERSYM: Self      = DynamicType(0x6ffffff0);
    pub const DT_RELACOUNT: Self   = DynamicType(0x6ffffff9);
    pub const DT_RELCOUNT: Self    = DynamicType(0x6ffffffa);
    pub const DT_FLAGS_1: Self     = DynamicType(0x6ffffffb);
    pub const DT_VERDEF: Self      = DynamicType(0x6ffffffc);
    pub const DT_VERDEFNUM: Self   = DynamicType(0x6ffffffd);
    pub const DT_VERNEED: Self     = DynamicType(0x6ffffffe);
    pub const DT_VERNEEDNUM: Self  = DynamicType(0x6fffffff);
    pub const OLD_DT_HIOS: Self    = DynamicType(0x6fffffff);
    pub const DT_LOPROC: Self      = DynamicType(0x70000000);
    pub const DT_HIPROC: Self      = DynamicType(0x7fffffff);

    // Contains
    pub fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    // Insert
    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0
    }

    // as u32
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

// Struct SymbolEntry
pub struct SymbolEntry {
    pub name: u32,
    pub value: u32,
    pub size: u32,
    pub type_bind: u8,
    pub other: u8,
    pub shndx: u16,
}

// Impl SymbolEntry
impl SymbolEntry {
    // New
    pub const fn new() -> Self {
        Self {
            name: 0,
            value: 0,
            size: 0,
            type_bind: 0,
            other: 0,
            shndx: 0,
        }
    }

    // From
    pub fn from(bytes: &[u8]) -> Self {
        let mut symtab = Self::new();

        if bytes.len() < 16 { return symtab; }

        symtab.name = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        symtab.value = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        symtab.size = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        symtab.type_bind = bytes[12];
        symtab.other = bytes[13];
        symtab.shndx = u16::from_le_bytes(bytes[14..16].try_into().unwrap());

        symtab
    }
}

// Struct RelocationEntry
pub struct RelocationEntry {
    pub offset: u32,
    pub typ: u8,
    pub symbol: u8,
    pub reversed: u16,
}

// Impl RelocationEntry
impl RelocationEntry {
    // New
    pub const fn new() -> Self {
        Self {
            offset: 0,
            typ: 0,
            symbol: 0,
            reversed: 0,
        }
    }

    // From
    pub fn from(bytes: &[u8]) -> Self {
        let mut reltab = Self::new();

        if bytes.len() < 8 { return reltab; }

        reltab.offset = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        reltab.typ = bytes[4];
        reltab.symbol = bytes[5];
        reltab.reversed = u16::from_le_bytes(bytes[6..8].try_into().unwrap());

        reltab
    }
}

// Struct ELFHeader
pub struct ELFHeader {
    pub ident: [u8; 16],
    pub typ: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u32,
    pub prog_hdr_off: u32,
    pub sect_hdr_off: u32,
    pub flags: u32,
    pub elf_hdr_size: u16,
    pub prog_hdr_size: u16,
    pub prog_hdr_num: u16,
    pub sect_hdr_size: u16,
    pub sect_hdr_num: u16,
    pub sect_hdr_str_tab_idx: u16,
}

// Impl ELFHeader
impl ELFHeader {
    // New
    pub const fn new() -> Self {
        Self {
            ident: [0u8; 16],
            typ: 0,
            machine: 0,
            version: 0,
            entry: 0,
            prog_hdr_off: 0,
            sect_hdr_off: 0,
            flags: 0,
            elf_hdr_size: 0,
            prog_hdr_size: 0,
            prog_hdr_num: 0,
            sect_hdr_size: 0,
            sect_hdr_num: 0,
            sect_hdr_str_tab_idx: 0,
        }
    }

    // From
    pub fn from(bytes: &[u8]) -> Self {
        let mut hdr = Self::new();

        if bytes.len() < 52 { return hdr; }

        hdr.ident.copy_from_slice(&bytes[0..16]);
        hdr.typ = u16::from_le_bytes(bytes[16..18].try_into().unwrap());
        hdr.machine = u16::from_le_bytes(bytes[18..20].try_into().unwrap());
        hdr.version = u32::from_le_bytes(bytes[20..24].try_into().unwrap());
        hdr.entry = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
        hdr.prog_hdr_off = u32::from_le_bytes(bytes[28..32].try_into().unwrap());
        hdr.sect_hdr_off = u32::from_le_bytes(bytes[32..36].try_into().unwrap());
        hdr.flags = u32::from_le_bytes(bytes[36..40].try_into().unwrap());
        hdr.elf_hdr_size = u16::from_le_bytes(bytes[40..42].try_into().unwrap());
        hdr.prog_hdr_size = u16::from_le_bytes(bytes[42..44].try_into().unwrap());
        hdr.prog_hdr_num = u16::from_le_bytes(bytes[44..46].try_into().unwrap());
        hdr.sect_hdr_size = u16::from_le_bytes(bytes[46..48].try_into().unwrap());
        hdr.sect_hdr_num = u16::from_le_bytes(bytes[48..50].try_into().unwrap());
        hdr.sect_hdr_str_tab_idx = u16::from_le_bytes(bytes[50..52].try_into().unwrap());

        hdr
    }
}

// Struct ProgramHeader
pub struct ProgramHeader {
    pub typ: u32,
    pub offset: u32,
    pub vaddr: u32,
    pub paddr: u32,
    pub file_size: u32,
    pub mem_size: u32,
    pub flags: u32,
    pub align: u32,
}

// Impl ProgramHeader
impl ProgramHeader {
    // New
    pub const fn new() -> Self {
        Self {
            typ: 0,
            offset: 0,
            vaddr: 0,
            paddr: 0,
            file_size: 0,
            mem_size: 0,
            flags: 0,
            align: 0,
        }
    }

    // From
    pub fn from(bytes: &[u8]) -> Self {
        let mut phdr = Self::new();

        if bytes.len() < 32 { return phdr; }

        phdr.typ = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        phdr.offset = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        phdr.vaddr = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        phdr.paddr = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        phdr.file_size = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        phdr.mem_size = u32::from_le_bytes(bytes[20..24].try_into().unwrap());
        phdr.flags = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
        phdr.align = u32::from_le_bytes(bytes[28..32].try_into().unwrap());

        phdr
    }
}

// Struct DynamicHeader
pub struct DynamicHeader {
    pub tag: u32,
    pub val: u32,
}

// Impl DynamicHeader
impl DynamicHeader {
    // New
    pub const fn new() -> Self {
        Self {
            tag: 0,
            val: 0,
        }
    }

    // From
    pub fn from(bytes: &[u8]) -> Self {
        let mut dynamic = Self::new();

        if bytes.len() < 8 { return dynamic; }

        dynamic.tag = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        dynamic.val = u32::from_le_bytes(bytes[4..8].try_into().unwrap());

        dynamic
    }
}

// Struct SectionData
pub struct SectionData {
    pub value: u32,
}

// Impl SectionData
impl SectionData {
    // New
    pub const fn new(value: u32) -> Self {
        Self {
            value,
        }
    }

    // As addr
    pub fn as_addr(&self) -> u32 {
        self.value
    }

    // As data
    pub fn as_data(&self) -> *mut u8 {
        self.value as *mut u8
    }

    // As dynstr
    pub fn as_dynstr(&self) -> *mut u8 {
        self.value as *mut u8
    }

    // As strtab
    pub fn as_strtab(&self) -> *mut u8 {
        self.value as *mut u8
    }

    // As shstrtab
    pub fn as_shstrtab(&self) -> *mut u8 {
        self.value as *mut u8
    }

    // As funcs
    pub fn as_funcs(&self) -> *mut Function {
        self.value as *mut Function
    }

    // As symtab
    pub fn as_symtab(&self) -> *mut SymbolEntry {
        self.value as *mut SymbolEntry
    }

    // As dynsym
    pub fn as_dynsym(&self) -> *mut SymbolEntry {
        self.value as *mut SymbolEntry
    }

    // As dynamic
    pub fn as_dynamic(&self) -> *mut DynamicHeader {
        self.value as *mut DynamicHeader
    }

    // As reltab
    pub fn as_reltab(&self) -> *mut RelocationEntry {
        self.value as *mut RelocationEntry
    }
}
