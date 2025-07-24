//###########################################################################
// vk_exec_decode.rs
// The specific implementation of functions related to exec decode
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

// Type aliases for start entry
type StartEntry = fn(DynKernel, &[&str]);

// Struct ExecDecoder
pub struct ExecDecoder {
    dynamic: u32,
    entry: u32,

    base: u32,
    exec: u32,

    elf: ElfDecoder,
}

// Impl ExecDecoder
impl ExecDecoder {
    // New
    pub const fn new() -> Self {
        Self {
            dynamic: 0,
            entry: 0,

            base: 0,
            exec: 0,

            elf: ElfDecoder::new(),
        }
    }

    // Get base address
    pub fn base(&mut self) -> u32 {
        self.base
    }
}

// Impl ExecDecoder
impl ExecDecoder {
    // decode
    fn decode(&mut self, data: &mut Vec<u8>) -> bool {
        if data.len() < 12 {
            return false;
        }

        self.dynamic = u32::from_le_bytes(data[0..4].try_into().unwrap());
        self.entry = u32::from_le_bytes(data[4..8].try_into().unwrap());

        self.base = data.as_ptr() as u32;
        self.exec = self.base + self.entry;

        true
    }

    // Erase a function pointer to a start entry
    fn start_exec(exec: u32) -> StartEntry {
        unsafe { core::mem::transmute::<u32, StartEntry>(exec) }
    }
}

// Impl ProgDecoder for ExecDecoder
impl ProgDecoder for ExecDecoder {
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

    // Execute
    fn exec(&mut self, argv: Vec<&str>) -> bool {
        if self.exec != 0 {
            (Self::start_exec(self.exec))(kernel, argv.as_slice());
            return true;
        }
        false
    }

    // Exit
    fn exit(&mut self) -> bool {
        self.elf.exit()
    }
}
