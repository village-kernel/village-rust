//###########################################################################
// vk_elf_loader.rs
// The specific implementation of functions related to elf loader
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::decoder::vk_defs_elf::{ELFClass, ELFVersion, ELFMachine, ELFType};
use crate::binutils::decoder::vk_defs_elf::{ELFHeader, ProgramHeader, ProgHdrType};
use crate::misc::fopts::vk_file_fopt::FileFopt;
use crate::traits::vk_builder::ProgLoader;
use crate::traits::vk_filesys::FileMode;
use crate::traits::vk_kernel::DebugLevel;
use crate::village::kernel;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// Struct ElfLoader
pub struct ElfLoader {
    elf: Vec<u8>,
    hdr: ELFHeader,
    filename: String,
}

// Impl ElfLoader
impl ElfLoader {
    // New
    pub const fn new() -> Self {
        Self {
            elf: Vec::new(),
            hdr: ELFHeader::new(),
            filename: String::new(),
        }
    }

    // Load elf
    fn load_elf(&mut self) -> bool {
        let mut file = FileFopt::new();
        let mut result = false;

        if file.open(&self.filename, FileMode::READ) {
            let size = file.size();
            self.elf.resize(size, 0);
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

    // Load parogram
    fn load_program(&mut self, data: &mut Vec<u8>) -> bool {
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
        data.resize(prog_size as usize, 0);

        // Load the program from the ELF file
        for phdr in phdrs.iter_mut() {
            for i in 0..phdr.mem_size {
                let vaddr = (phdr.vaddr + i) as usize;
                let offset = (phdr.offset + i) as usize;
                data[vaddr] = self.elf[offset];
            }
        }

        true
    }
}

// Impl ProgLoader for ELFLoader
impl ProgLoader for ElfLoader {
    // Init
    fn init(&mut self, filename: &str, data: &mut Vec<u8>) -> bool {
        // Save filename in local
        self.filename = filename.to_string();

        // Load and mapping
        if !self.load_elf() {
            return false;
        }
        if !self.check_elf() {
            return false;
        }
        if !self.load_program(data) {
            return false;
        }

        true
    }
    
    // Exit
    fn exit(&mut self) -> bool {
        self.elf.clear();
        true
    }
}
