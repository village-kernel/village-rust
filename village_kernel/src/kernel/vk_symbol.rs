//###########################################################################
// vk_symbol.rs
// The specific implementation of functions related to symbol
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::Symbol;
use crate::traits::vk_linkedlist::LinkedList;
use crate::debug_info;

// Struct entry
struct Entry {
    name: *const str,
    addr: usize,
}

// Impl entry
impl Entry {
    // New
    const fn new(addr: usize, name: &str) -> Self {
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
        debug_info!("Symbol setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {}
}

// Impl symbol for village symbol
impl Symbol for VillageSymbol {
    // Export
    fn export(&mut self, sym_addr: usize, name: &str) {
        let entry = Entry::new(sym_addr, name);
        self.entrys.push(entry);
    }

    // Unexport
    fn unexport(&mut self, sym_addr: usize, name: &str) {
        self.entrys
            .retain_mut(|entry| !(entry.addr == sym_addr && entry.name == name));
    }

    // Search
    fn search(&mut self, symbol: &str) -> usize {
        if let Some(entry) = self.entrys.iter_mut().find(|t| t.name == symbol) {
            return entry.addr;
        }
        0
    }
}
