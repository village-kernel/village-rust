//###########################################################################
// vk_prog_decode.rs
// The specific implementation of functions related to prog decode
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_elf_defines::{DynamicHeader, DynamicType, RelocationCode, RelocationEntry};
use crate::traits::vk_executor::BaseDecoder;
use crate::traits::vk_kernel::Kernel;
use crate::village::kernel;
use alloc::vec::Vec;

// Type aliases for start entry
type DynKernel = fn() -> &'static mut dyn Kernel;

// Type aliases for start entry
type StartEntry = fn(DynKernel, &[&str]);

// Struct ProgDecoder
pub struct ProgDecoder {
    data: Vec<u8>,

    load: u32,
    base: u32,
    exec: u32,

    offset: u32,
    dynamic: u32,
    entry: u32,
}

// Impl ProgDecoder
impl ProgDecoder {
    // New
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),

            load: 0,
            base: 0,
            exec: 0,

            offset: 0,
            dynamic: 0,
            entry: 0,
        }
    }

    // Get base address
    pub fn base(&mut self) -> u32 {
        self.base
    }
}

// Impl ProgDecoder
impl ProgDecoder {
    // decode
    fn decode(&mut self, data: Vec<u8>) -> bool {
        if data.len() < 12 {
            return false;
        }

        self.data = data;
        self.load = self.data.as_ptr() as u32;
        self.offset = u32::from_le_bytes(self.data[0..4].try_into().unwrap());
        self.dynamic = u32::from_le_bytes(self.data[4..8].try_into().unwrap());
        self.entry = u32::from_le_bytes(self.data[8..12].try_into().unwrap());

        self.base = self.load - self.offset;
        self.exec = self.base + self.entry;

        true
    }

    // Relocate
    fn relocate(&mut self) -> bool {
        let mut relcount: u32 = 0;
        let mut relocate: Option<u32> = None;

        // Calc dynamic section offset in bin data
        let dynamic_start = (self.dynamic - self.offset) as usize;
        if dynamic_start + 8 > self.data.len() {
            return false;
        }

        // Gets dynamic bytes from bin data
        let dynamic_bytes = &self.data[dynamic_start..];

        // Gets the relocate section address and the relcount
        let mut i = 0;
        loop {
            // Calc dynamic offset
            let dynamic_offset = i * 8;
            if dynamic_offset + 8 > dynamic_bytes.len() {
                break;
            }

            // Convert bytes into dynamic header
            let dynamic = DynamicHeader::from(&dynamic_bytes[dynamic_offset..dynamic_offset + 8]);

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
        if relocate.is_none() && relcount == 0 {
            return true;
        }
        if relocate.is_none() || relcount == 0 {
            return false;
        }

        // Calc relocate start offset
        let relocate_start = (relocate.unwrap() - self.offset) as usize;

        // Relocate the value of relative type
        for i in 0..relcount {
            let relocate_offset = relocate_start + (i * 8) as usize;
            if relocate_offset + 8 > self.data.len() {
                continue;
            }

            let relocate_entry =
                RelocationEntry::from(&self.data[relocate_offset..relocate_offset + 8]);

            if relocate_entry.typ == RelocationCode::TYPE_RELATIVE {
                let rel_addr_offset = (relocate_entry.offset - self.offset) as usize;
                if rel_addr_offset + 4 > self.data.len() {
                    continue;
                }

                // Read original relative value
                let original_relative = u32::from_le_bytes(
                    self.data[rel_addr_offset..rel_addr_offset + 4]
                        .try_into()
                        .unwrap(),
                );

                // Calc relocated value, absolute address
                let absolute_addr = self.base + original_relative;

                // Write relocated value back
                let absolute_bytes = absolute_addr.to_le_bytes();
                self.data[rel_addr_offset..rel_addr_offset + 4].copy_from_slice(&absolute_bytes);
            }
        }

        true
    }

    // Erase a function pointer to a start entry
    fn start_exec(exec: u32) -> StartEntry {
        unsafe { core::mem::transmute::<u32, StartEntry>(exec) }
    }
}

// Impl ProgDecoder
impl BaseDecoder for ProgDecoder {
    // Init
    fn init(&mut self, data: Vec<u8>) -> bool {
        if !self.decode(data) {
            return false;
        }
        if !self.relocate() {
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
        self.data.clear();
        self.data.shrink_to_fit();
        true
    }
}
