//###########################################################################
// vk_mod_decode.rs
// The specific implementation of functions related to mod decode
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::binutils::decoder::vk_elf_decode::ElfDecoder;
use crate::traits::vk_builder::ProgDecoder;
use crate::traits::vk_kernel::Kernel;
use crate::village::kernel;
use alloc::vec::Vec;

// Type aliases for start entry
type DynKernel = fn() -> &'static mut dyn Kernel;

// Type aliases for func entry
type FuncEntry = fn(DynKernel);

// Struct ModDecoder
pub struct ModDecoder {
    dynamic: u32,
    init_entry: u32,
    exit_entry: u32,

    base: u32,
    init_exec: u32,
    exit_exec: u32,

    elf: ElfDecoder,
}

// Impl ModDecoder
impl ModDecoder {
    // New
    pub const fn new() -> Self {
        Self {
            dynamic: 0,
            init_entry: 0,
            exit_entry: 0,

            base: 0,
            init_exec: 0,
            exit_exec: 0,

            elf: ElfDecoder::new(),
        }
    }

    // Get base address
    pub fn base(&mut self) -> u32 {
        self.base
    }
}

// Impl ModDecoder
impl ModDecoder {
    // decode
    fn decode(&mut self, data: &mut Vec<u8>) -> bool {
        if data.len() < 12 {
            return false;
        }

        self.dynamic = u32::from_le_bytes(data[0..4].try_into().unwrap());
        self.init_entry = u32::from_le_bytes(data[4..8].try_into().unwrap());
        self.exit_entry = u32::from_le_bytes(data[8..12].try_into().unwrap());

        self.base = data.as_ptr() as u32;
        self.init_exec = self.base + self.init_entry;
        self.exit_exec = self.base + self.exit_entry;

        true
    }

    // Erase a function pointer to a func entry
    fn func_exec(exec: u32) -> FuncEntry {
        unsafe { core::mem::transmute::<u32, FuncEntry>(exec) }
    }
}

// Impl ProgDecpder for ModDecoder
impl ProgDecoder for ModDecoder {
    // Init
    fn init(&mut self, path: &str, mut data: Vec<u8>) -> bool {
        if !self.decode(&mut data) {
            return false;
        }
        if !self.elf.init(path, data, self.dynamic) {
            return false;
        }
        if self.init_exec != 0 {
            (Self::func_exec(self.init_exec))(kernel);
            return true;
        }
        false
    }

    // Execute
    fn exec(&mut self, _argv: Vec<&str>) -> bool {
        true
    }

    // Exit
    fn exit(&mut self) -> bool {
        if self.exit_exec != 0 {
            (Self::func_exec(self.exit_exec))(kernel);
        }
        self.elf.exit()
    }
}
