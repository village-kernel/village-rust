//###########################################################################
// vk_defs_elf.rs
// The specific implementation of functions related to elf defines
//
// $Copyright: Copyright (C) village
//###########################################################################

// Flag ELFClass
pub struct ELFClass;

// Impl ELFClass
impl ELFClass {
    pub const NONE: u8 = 0;
    pub const X32: u8 = 1;
    pub const X64: u8 = 2;
}

// Struct ELFType
pub struct ELFType;

// Impl ELFType
impl ELFType {
    pub const NONE: u16 = 0;
    pub const REL: u16 = 1;
    pub const EXEC: u16 = 2;
    pub const DYN: u16 = 3;
    pub const CORE: u16 = 4;
    pub const LO_PROC: u16 = 5;
    pub const HI_PROC: u16 = 6;
}

// Struct ELFMachine
pub struct ELFMachine;

// Impl ELFMachine
impl ELFMachine {
    pub const NONE: u16 = 0x00;
    pub const X86: u16 = 0x03;
    pub const ARM: u16 = 0x28;
    pub const ARM_64: u16 = 0xb7;
    pub const RISC_V: u16 = 0xf3;
}

// Struct ELFVersion
pub struct ELFVersion;

// Impl ELFVersion
impl ELFVersion {
    pub const NONE: u32 = 0x00;
    pub const CURRENT: u32 = 0x01;
}

// Struct RelocateCode
pub struct RelocateCode;

// Impl RelocateCode
#[cfg(feature = "arch_i386")]
impl RelocateCode {
    pub const I386_NONE: u8 = 0;
    pub const I386_32: u8 = 1;
    pub const I386_PC32: u8 = 2;
    pub const I386_GOT32: u8 = 3;
    pub const I386_PLT32: u8 = 4;
    pub const I386_COPY: u8 = 5;
    pub const TYPE_COPY: u8 = Self::I386_COPY;
    pub const I386_GLOB_DAT: u8 = 6;
    pub const I386_JMP_SLOT: u8 = 7;
    pub const I386_RELATIVE: u8 = 8;
    pub const TYPE_RELATIVE: u8 = Self::I386_RELATIVE;
    pub const I386_GOTOFF: u8 = 9;
    pub const I386_GOTPC: u8 = 10;
    pub const I386_32PLT: u8 = 11;
    pub const I386_16: u8 = 20;
    pub const I386_PC16: u8 = 21;
    pub const I386_8: u8 = 22;
    pub const I386_PC8: u8 = 23;
    pub const I386_SIZE32: u8 = 38;
}

// Impl RelocateCode
#[cfg(feature = "arch_arm")]
impl RelocateCode {
    pub const ARM_NONE: u8 = 0;
    pub const ARM_ABS32: u8 = 2;
    pub const ARM_THM_CALL: u8 = 10;
    pub const ARM_COPY: u8 = 20;
    pub const TYPE_COPY: u8 = Self::ARM_COPY;
    pub const ARM_GLOB_DAT: u8 = 21;
    pub const ARM_JUMP_SLOT: u8 = 22;
    pub const ARM_RELATIVE: u8 = 23;
    pub const TYPE_RELATIVE: u8 = Self::ARM_RELATIVE;
    pub const ARM_THM_JUMP24: u8 = 30;
    pub const ARM_TARGET1: u8 = 38;
    pub const ARM_THM_JUMP11: u8 = 102;
}

// Struct ProgHdrType
pub struct ProgHdrType;

// Impl ProgHdrType
impl ProgHdrType {
    pub const PT_NULL: u32 = 0x00;
    pub const PT_LOAD: u32 = 0x01;
    pub const PT_DYNAMIC: u32 = 0x02;
    pub const PT_INTERP: u32 = 0x03;
    pub const PT_NOTE: u32 = 0x04;
    pub const PT_SHLIB: u32 = 0x05;
    pub const PT_PHDR: u32 = 0x06;
    pub const PT_TLS: u32 = 0x07;
    pub const PT_NUM: u32 = 0x08;
    pub const PT_LOOS: u32 = 0x60000000;
    pub const PT_GNU_EH_FRAME: u32 = 0x6474e550;
    pub const PT_GNU_STACK: u32 = 0x6474e551;
    pub const PT_GNU_RELRO: u32 = 0x6474e552;
    pub const PT_LOSUNW: u32 = 0x6ffffffa;
    pub const PT_SUNWBSS: u32 = 0x6ffffffa;
    pub const PT_SUNWSTACK: u32 = 0x6ffffffb;
    pub const PT_HISUNW: u32 = 0x6fffffff;
    pub const PT_HIOS: u32 = 0x6fffffff;
    pub const PT_LOPROC: u32 = 0x70000000;
    pub const PT_HIPROC: u32 = 0x7fffffff;
}

// Struct SectionHdrType
pub struct SectionHdrType;

// Impl SectionHdrType
impl SectionHdrType {
    pub const SHT_NULL: u32 = 0x00;
    pub const SHT_PROGBITS: u32 = 0x01;
    pub const SHT_SYMTAB: u32 = 0x02;
    pub const SHT_STRTAB: u32 = 0x03;
    pub const SHT_RELA: u32 = 0x04;
    pub const SHT_HASH: u32 = 0x05;
    pub const SHT_DYNAMIC: u32 = 0x06;
    pub const SHT_NOTE: u32 = 0x07;
    pub const SHT_NOBITS: u32 = 0x08;
    pub const SHT_REL: u32 = 0x09;
    pub const SHT_SHLIB: u32 = 0x0a;
    pub const SHT_DYNSYM: u32 = 0x0b;
    pub const SHT_INIT_ARRAY: u32 = 0x0e;
    pub const SHT_FINI_ARRAY: u32 = 0x0f;
    pub const SHT_PREINIT_ARRAY: u32 = 0x10;
    pub const SHT_GROUP: u32 = 0x11;
    pub const SHT_SYMTAB_SHNDX: u32 = 0x12;
    pub const SHT_NUM: u32 = 0x13;
    pub const SHT_LOOS: u32 = 0x60000000;
}

// Struct SymbolType
pub struct SymbolType;

// Impl SymbolType
impl SymbolType {
    pub const STT_NOTYPE: u8 = 0;
    pub const STT_OBJECT: u8 = 1;
    pub const STT_FUNC: u8 = 2;
    pub const STT_SECTION: u8 = 3;
    pub const STT_FILE: u8 = 4;
    pub const STT_LOPROC: u8 = 13;
    pub const STT_HIPROC: u8 = 15;
}

// Struct SymbolBind
pub struct SymbolBind;

// Impl SymbolBind
impl SymbolBind {
    pub const STB_LOCAL: u8 = 0;
    pub const STB_GLOBAL: u8 = 1;
    pub const STB_WEAK: u8 = 2;
    pub const STB_LOPROC: u8 = 13;
    pub const STB_HIPROC: u8 = 15;
}

// Struct DynamicType
pub struct DynamicType;

// Impl DynamicType
impl DynamicType {
    pub const DT_NULL: u32 = 0;
    pub const DT_NEEDED: u32 = 1;
    pub const DT_PLTRELSZ: u32 = 2;
    pub const DT_PLTGOT: u32 = 3;
    pub const DT_HASH: u32 = 4;
    pub const DT_STRTAB: u32 = 5;
    pub const DT_SYMTAB: u32 = 6;
    pub const DT_RELA: u32 = 7;
    pub const DT_RELASZ: u32 = 8;
    pub const DT_RELAENT: u32 = 9;
    pub const DT_STRSZ: u32 = 10;
    pub const DT_SYMENT: u32 = 11;
    pub const DT_INIT: u32 = 12;
    pub const DT_FINI: u32 = 13;
    pub const DT_SONAME: u32 = 14;
    pub const DT_RPATH: u32 = 15;
    pub const DT_SYMBOLIC: u32 = 16;
    pub const DT_REL: u32 = 17;
    pub const DT_RELSZ: u32 = 18;
    pub const DT_RELENT: u32 = 19;
    pub const DT_PLTREL: u32 = 20;
    pub const DT_DEBUG: u32 = 21;
    pub const DT_TEXTREL: u32 = 22;
    pub const DT_JMPREL: u32 = 23;
    pub const DT_ENCODING: u32 = 32;
    pub const OLD_DT_LOOS: u32 = 0x60000000;
    pub const DT_LOOS: u32 = 0x6000000d;
    pub const DT_HIOS: u32 = 0x6ffff000;
    pub const DT_VALRNGLO: u32 = 0x6ffffd00;
    pub const DT_VALRNGHI: u32 = 0x6ffffdff;
    pub const DT_ADDRRNGLO: u32 = 0x6ffffe00;
    pub const DT_ADDRRNGHI: u32 = 0x6ffffeff;
    pub const DT_VERSYM: u32 = 0x6ffffff0;
    pub const DT_RELACOUNT: u32 = 0x6ffffff9;
    pub const DT_RELCOUNT: u32 = 0x6ffffffa;
    pub const DT_FLAGS_1: u32 = 0x6ffffffb;
    pub const DT_VERDEF: u32 = 0x6ffffffc;
    pub const DT_VERDEFNUM: u32 = 0x6ffffffd;
    pub const DT_VERNEED: u32 = 0x6ffffffe;
    pub const DT_VERNEEDNUM: u32 = 0x6fffffff;
    pub const OLD_DT_HIOS: u32 = 0x6fffffff;
    pub const DT_LOPROC: u32 = 0x70000000;
    pub const DT_HIPROC: u32 = 0x7fffffff;
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
    // Size
    pub const SIZE: usize = 16;

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

        if bytes.len() < Self::SIZE {
            return symtab;
        }

        symtab.name = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        symtab.value = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        symtab.size = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        symtab.type_bind = bytes[12];
        symtab.other = bytes[13];
        symtab.shndx = u16::from_le_bytes(bytes[14..16].try_into().unwrap());

        symtab
    }
}

// Struct RelocateEntry
pub struct RelocateEntry {
    pub offset: u32,
    pub typ: u8,
    pub symbol: u8,
    pub reversed: u16,
}

// Impl RelocateEntry
impl RelocateEntry {
    // Size
    pub const SIZE: usize = 8;

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

        if bytes.len() < Self::SIZE {
            return reltab;
        }

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
    // Size
    pub const SIZE: usize = 52;

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

        if bytes.len() < Self::SIZE {
            return hdr;
        }

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

// Struct SectionHeader
pub struct SectionHeader {
    pub name: u32,
    pub typ: u32,
    pub flags: u32,
    pub addr: u32,
    pub offset: u32,
    pub size: u32,
    pub link: u32,
    pub info: u32,
    pub addr_align: u32,
    pub entire_size: u32,
}

// Impl SectionHeader
impl SectionHeader {
    // Size
    pub const SIZE: usize = 40;

    // New
    pub const fn new() -> Self {
        Self {
            name: 0,
            typ: 0,
            flags: 0,
            addr: 0,
            offset: 0,
            size: 0,
            link: 0,
            info: 0,
            addr_align: 0,
            entire_size: 0,
        }
    }

    // From
    pub fn from(bytes: &[u8]) -> Self {
        let mut shdr = Self::new();

        if bytes.len() < Self::SIZE {
            return shdr;
        }

        shdr.name = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        shdr.typ = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        shdr.flags = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        shdr.addr = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        shdr.offset = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        shdr.size = u32::from_le_bytes(bytes[20..24].try_into().unwrap());
        shdr.link = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
        shdr.info = u32::from_le_bytes(bytes[28..32].try_into().unwrap());
        shdr.addr_align = u32::from_le_bytes(bytes[32..36].try_into().unwrap());
        shdr.entire_size = u32::from_le_bytes(bytes[36..40].try_into().unwrap());

        shdr
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
    // Size
    pub const SIZE: usize = 32;

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

        if bytes.len() < Self::SIZE {
            return phdr;
        }

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
    // Size
    pub const SIZE: usize = 8;

    // New
    pub const fn new() -> Self {
        Self { tag: 0, val: 0 }
    }

    // From
    pub fn from(bytes: &[u8]) -> Self {
        let mut dynamic = Self::new();

        if bytes.len() < Self::SIZE {
            return dynamic;
        }

        dynamic.tag = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        dynamic.val = u32::from_le_bytes(bytes[4..8].try_into().unwrap());

        dynamic
    }
}
