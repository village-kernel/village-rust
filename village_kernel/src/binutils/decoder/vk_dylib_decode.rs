//###########################################################################
// vk_dylib_decode.rs
// The specific implementation of functions related to dylib decode
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::decoder::vk_elf_defines::{ELFHeader, ProgramHeader, ProgHdrType};
use crate::binutils::decoder::vk_elf_decode::ElfDecoder;
use crate::traits::vk_builder::LibDecoder;
use alloc::vec::Vec;

// Struct DylibDecoder
pub struct DylibDecoder {
    base: u32,
    dynamic: u32,

    hdr: ELFHeader,
    elf: ElfDecoder,
}

// Impl DylibDecoder
impl DylibDecoder {
    // New
    pub const fn new() -> Self {
        Self {
            base: 0,
            dynamic: 0,

            hdr: ELFHeader::new(),
            elf: ElfDecoder::new(),
        }
    }

    // Get base address
    pub fn base(&mut self) -> u32 {
        self.base
    }
}

// Impl DylibDecoder
impl DylibDecoder {
    // Decode
    fn decode(&mut self, data: &mut Vec<u8>) -> bool {
        // Set data
        self.base = data.as_ptr() as u32;

        // Set elf header
        self.hdr = ELFHeader::from(&data[0..ELFHeader::SIZE]);

        // Get dynmaic offset
        for i in 0..self.hdr.prog_hdr_num as usize {
            let prog_start = self.hdr.prog_hdr_off as usize + i * self.hdr.prog_hdr_size as usize;
            let prog_end = prog_start + self.hdr.prog_hdr_size as usize;
            let phdr = ProgramHeader::from(&data[prog_start..prog_end]);

            if phdr.typ == ProgHdrType::PT_DYNAMIC {
                self.dynamic = phdr.vaddr;
                break;
            }
        }

        true
    }
}

// Impl LibDecoder for DylibDecoder
impl LibDecoder for DylibDecoder {
    // Init
    fn init(&mut self, path: &str, mut data: Vec<u8>) -> bool {
        if !self.decode(&mut data) {
            return false;
        }
        if !self.elf.init(path, data, self.dynamic) {
            return false;
        }
        true
    }

    // Get
    fn get(&mut self, symbol: &str) -> usize {
        self.elf.get(symbol)
    }
    
    // Exit
    fn exit(&mut self) -> bool {
        self.elf.exit()
    }
}
