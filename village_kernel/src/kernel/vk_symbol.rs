//###########################################################################
// vk_symbol.rs
// The specific implementation of functions related to symbol
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::Symbol;
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;

// Struct entry
struct Entry {
    name: *const str,
    addr: u32,
}

// Impl entry
impl Entry {
    // New
    const fn new(addr: u32, name: &str) -> Self {
        Self { name, addr }
    }
}

// Struct village symbol
pub struct VillageSymbol {
    entrys: LinkedList<Entry>,
}

// Impl village symbol
impl VillageSymbol {
    pub const fn new() -> Self {
        Self {
            entrys: LinkedList::new(),
        }
    }
}

// Impl village symbol
impl VillageSymbol {
    // Setup
    pub fn setup(&mut self) {
        // Output debug info
        kernel().debug().info("Symbol setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {}
}

// Impl symbol for village symbol
impl Symbol for VillageSymbol {
    // Export
    fn export(&mut self, sym_addr: u32, name: &str) {
        let entry = Entry::new(sym_addr, name);
        self.entrys.push(entry);
    }

    // Unexport
    fn unexport(&mut self, sym_addr: u32, name: &str) {
        self.entrys
            .retain_mut(|entry| !(entry.addr == sym_addr && entry.name == name));
    }

    // Search
    fn search(&mut self, name: &str) -> u32 {
        if let Some(entry) = self.entrys.iter_mut().find(|t| t.name == name) {
            return entry.addr;
        }
        0
    }
}
