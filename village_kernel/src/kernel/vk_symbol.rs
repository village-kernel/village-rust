use crate::traits::vk_linkedlist::LinkedList;
//###########################################################################
// vk_symbol.rs
// The specific implementation of functions related to symbol
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use crate::village::kernel;
use crate::traits::vk_kernel::Symbol;

// Struct entry
struct Entry {
    name: *const str,
    addr: u32,
}

// Impl entry
impl Entry {
    // New
    const fn new(addr: u32, name: &str) -> Self {
        Self {
            name,
            addr,
        }
    }
}

// Impl partia eq for entry
impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

// Struct concrete symbol
pub struct ConcreteSymbol {
    entrys: LinkedList<Entry>
}

// Impl concrete symbol
impl ConcreteSymbol {
    pub const fn new() -> Self {
        Self {
            entrys: LinkedList::new()
        }
    }
}

// Impl concrete symbol
impl ConcreteSymbol {
    // Setup
    pub fn setup(&mut self) {
        // Output debug info
        kernel().debug().info("Symbol setup done!");
    }

    // Exit
    pub fn exit(&mut self) {

    }
}

// Impl symbol for concrete symbol
impl Symbol for ConcreteSymbol {
    // Export
    fn export(&mut self, sym_addr: u32, name: &str) {
        let entry = Entry::new(sym_addr, name);
        self.entrys.push(entry);
    }

    // Unexport
    fn unexport(&mut self, sym_addr: u32, name: &str) {
        let entry = Entry::new(sym_addr, name);
        self.entrys.delete(&entry);
    }
    
    // Search
    fn search(&mut self, name: &str) -> u32 {
        if let Some(entry) = self.entrys.iter_mut().find(|t| t.name == name) {
            return entry.addr;
        }
        0
    }
}
