//###########################################################################
// vk_memory.rs
// The specific implementation of functions related to memory
//
// $Copyright: Copyright (C) village
//###########################################################################
use spin::Mutex;
use core::ptr;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, Ordering};
use crate::village::kernel;
use crate::traits::vk_kernel::Memory;

// Constant Members
const ALIGN: u32 = 4;

// Struct map
#[repr(C, align(4))]
struct Map {
    addr: u32,
    size: u32,
}

// Impl map
impl Map {
    const fn new(addr: u32, size: u32) -> Self {
        Self { addr, size }
    }
}

// Struct map node
#[repr(C, align(4))]
struct MapNode {
    map: Map,
    prev: AtomicPtr<MapNode>,
    next: AtomicPtr<MapNode>,
}

// Impl map node
impl MapNode {
    const fn new(map: Map) -> Self {
        Self {
            map,
            prev: AtomicPtr::new(core::ptr::null_mut()),
            next: AtomicPtr::new(core::ptr::null_mut()),
        }
    }
}

// Struct concrete memory
struct MemoryAllocator {
    sram_start: AtomicU32,
    sram_ended: AtomicU32,
    sram_used: AtomicU32,

    head: AtomicPtr<MapNode>,
    tail: AtomicPtr<MapNode>,
    curr: AtomicPtr<MapNode>,

    initialized: AtomicBool,
}

// Impl sync for conrete memory
unsafe impl Sync for MemoryAllocator {}

// Impl conrete memory
impl MemoryAllocator {
    // New
    pub const fn new() -> Self {
        Self {
            sram_start: AtomicU32::new(0),
            sram_ended: AtomicU32::new(0),
            sram_used: AtomicU32::new(0),

            head: AtomicPtr::new(core::ptr::null_mut()),
            tail: AtomicPtr::new(core::ptr::null_mut()),
            curr: AtomicPtr::new(core::ptr::null_mut()),

            initialized: AtomicBool::new(false),
        }
    }

    // Align up
    fn align_up(value: u32, align: u32) -> u32 {
        (value + align - 1) & !(align - 1)
    }

    // Align down
    fn align_down(value: u32, align: u32) -> u32 {
        value & !(align - 1)
    }
}

// Impl concrete memory
impl MemoryAllocator {
    // Initiate
    pub fn initiate(&mut self) {
        // Return when initialized
        if self.initialized.load(Ordering::Acquire) == true {
            return;
        }

        // Initialize heap end at first call
        if self.sram_start.load(Ordering::Relaxed) == 0 {
            // Symbol defined in the linker script
            unsafe extern "C" {
                unsafe static _ebss: u32;
                unsafe static _estack: u32;
                unsafe static _rsvd_heap: u32;
                unsafe static _rsvd_stack: u32;
            }

            // Gets the ebss, estack, rsvd_heap and rsvd_stack value
            let ebss = unsafe { &_ebss as *const u32 as u32 };
            let estack = unsafe { &_estack as *const u32 as u32 };
            let rsvd_heap = unsafe { &_rsvd_heap as *const u32 as u32 };
            let rsvd_stack = unsafe { &_rsvd_stack as *const u32 as u32 };

            // Calculate sram start and end address
            let sram_start = ebss + rsvd_heap;
            let sram_ended = estack - rsvd_stack;

            // Aligning sram_start and sram_ended by align byte
            let sram_start = Self::align_up(sram_start, ALIGN);
            let sram_ended = Self::align_down(sram_ended, ALIGN);
            
            // Store value
            self.sram_start.store(sram_start, Ordering::Relaxed);
            self.sram_ended.store(sram_ended, Ordering::Relaxed);
        }

        // Initialize list, align 4 bytes
        if self.head.load(Ordering::Relaxed).is_null() ||
           self.head.load(Ordering::Relaxed).is_null()
        {
            let size_of_node = core::mem::size_of::<MapNode>() as u32;
            let sram_start = self.sram_start.load(Ordering::Relaxed);
            let sram_ended = self.sram_ended.load(Ordering::Relaxed);

            // Create head and tail
            let head =  sram_start as *mut MapNode;
            let tail = (sram_start + size_of_node) as *mut MapNode;
            
            // Initialize head and tail node
            unsafe {
                ptr::write(head, MapNode::new(
                    Map::new(sram_start + size_of_node, size_of_node))
                );

                ptr::write(tail, MapNode::new(
                    Map::new(sram_ended - size_of_node, size_of_node))
                );

                (*head).next.store(tail, Ordering::Relaxed);
                (*tail).prev.store(head, Ordering::Relaxed);
            }
            
            // Store value
            self.head.store(head, Ordering::Relaxed);
            self.tail.store(tail, Ordering::Relaxed);
            self.curr.store(head, Ordering::Relaxed);
        }
        
        // Set initialized flag
        self.initialized.store(true, Ordering::Release);
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear initialized flag
        self.initialized.store(false, Ordering::Relaxed);
    }
}

// Impl memory for concrete memory
impl MemoryAllocator {
    // Heap alloc
    // |--------------|---------------|
    // | size of node | size of alloc |
    // |--------------|---------------|
    // |    MapNode   |  heap memory  |
    // |--------------|---------------|
    fn heap_alloc(&mut self, size: u32) -> u32 {
        // Check is initialized
        if !self.initialized.load(Ordering::Acquire) {
            self.initiate();
        }

        let size_of_node = core::mem::size_of::<MapNode>() as u32;
        let mut curr_node = self.curr.load(Ordering::Acquire);
        let mut alloc_addr = 0;
        let mut ena_retry = true;

        unsafe {
            // Find free space
            while !curr_node.is_null() {
                // Get the next node
                let next_node = (*curr_node).next.load(Ordering::Relaxed);

                // Retry or break when next node is null
                if next_node.is_null() { 
                    if ena_retry {
                        curr_node = self.head.load(Ordering::Acquire);
                        ena_retry = false;
                        continue;
                    } else {
                        break;
                    }
                }

                // Calculate and align the new node map size
                let mut new_map_size = size_of_node + size;
                new_map_size = Self::align_up(new_map_size, ALIGN);

                // Calculate and align the new node map addr
                let mut new_map_addr = (*curr_node).map.addr + (*curr_node).map.size;
                new_map_addr = Self::align_up(new_map_addr, ALIGN);

                // Calculate the end addr
                let new_end_addr = new_map_addr + new_map_size;

                // There is free space between the current node and the next node
                if new_end_addr <= (*next_node).map.addr {
                    // Update the used size of sram
                    self.sram_used.fetch_add(new_map_size, Ordering::SeqCst);

                    // Create an new node
                    let new_node = new_map_addr as *mut MapNode;
                    ptr::write(new_node, MapNode{
                        map: Map::new(new_map_addr, new_map_size),
                        prev: curr_node.into(),
                        next: next_node.into(),
                    });

                    // Memory barrier: Ensure that the pointer update of the new node is visible to other threads.
                    core::sync::atomic::fence(Ordering::Release);

                    // Update list
                    (*curr_node).next.store(new_node, Ordering::Release);
                    (*next_node).prev.store(new_node, Ordering::Release);
                    
                    // Update curr node
                    self.curr.store(new_node, Ordering::Relaxed);

                    // Calculate the alloc address
                    alloc_addr = new_map_addr + size_of_node;
                    break;
                }

                curr_node = next_node;
            }
        }

        alloc_addr
    }
    
    // Stack alloc
    // |-------------------| |---------------|
    // |  MapNode in heap  | | size of alloc |
    // |                   | |---------------|
    // |  memory in stack  | | stack memory  |
    // |-------------------| |---------------|
    fn stack_alloc(&mut self, size: u32) -> u32 {
        // Check is initialized
        if !self.initialized.load(Ordering::Acquire) {
            self.initiate();
        }

        // Create an new node by heap alloc
        let size_of_node = core::mem::size_of::<MapNode>() as u32;
        let new_node = self.heap_alloc(size_of_node) as *mut MapNode;

        let mut curr_node = self.tail.load(Ordering::Acquire);
        let mut alloc_addr = 0;

        unsafe {
            // Find free space
            while !curr_node.is_null() {
                // Get the prev node
                let prev_node = (*curr_node).prev.load(Ordering::Acquire);

                // Break loop when prev node is null
                if prev_node.is_null() { break; }

                // Calculate and align the new map size
                let mut new_map_size = size;
                new_map_size = Self::align_up(new_map_size, ALIGN);
                
                // Calculate and align the new map
                let mut new_map_addr = (*curr_node).map.addr - new_map_size;
                new_map_addr = Self::align_up(new_map_addr, ALIGN);

                // There is free space between the current node and the prev node
                if  new_map_addr >= ((*prev_node).map.addr + (*prev_node).map.size) {
                    // Update the used size of sram
                    self.sram_used.fetch_add(new_map_size, Ordering::SeqCst);

                    // Create an new node
                    ptr::write(new_node, MapNode{
                        map: Map::new(new_map_addr, new_map_size),
                        prev: prev_node.into(),
                        next: curr_node.into(),
                    });

                    // Memory barrier: Ensure that the pointer update of the new node is visible to other threads.
                    core::sync::atomic::fence(Ordering::Release);

                    // Update list
                    (*prev_node).next.store(new_node, Ordering::Release);
                    (*curr_node).prev.store(new_node, Ordering::Release);

                    // Update current node
                    self.curr.store(new_node, Ordering::Relaxed);
 
                    // Calculate the alloc address
                    alloc_addr = new_map_addr;
                    break;
                }

                curr_node = prev_node;
            }
        }

        alloc_addr
    }
    
    // Free
    fn free(&mut self, memory: u32, size: u32) {
        if memory == 0 { return; }

        let size_of_node = core::mem::size_of::<MapNode>() as u32;
        let mut curr_node = self.curr.load(Ordering::Acquire);

        unsafe {
            while !curr_node.is_null() {
            
                let curr_start_addr = (*curr_node).map.addr;
                let curr_ended_addr = (*curr_node).map.addr + (*curr_node).map.size;

                // Break when the memory is between the end of the current node 
                // and the beginning of the next node, because it has been released
                let next_node = (*curr_node).next.load(Ordering::Acquire);
                if !next_node.is_null() {
                    let next_start_addr = (*next_node).map.addr;
                    if memory > curr_ended_addr && memory < next_start_addr {
                        break;
                    }
                }

                // Release memory node
                if memory >= curr_start_addr && memory < curr_ended_addr {
                    let curr_map_size = (*curr_node).map.size;

                    if size == 0 || size_of_node == (curr_map_size - size) {
                        let prev_node = (*curr_node).prev.load(Ordering::Acquire);
                        let next_node = (*curr_node).next.load(Ordering::Acquire);
                        
                        // Remove map node from list
                        if !prev_node.is_null() {
                            (*prev_node).next = next_node.into();
                        }
                        if !next_node.is_null() {
                            (*next_node).prev = prev_node.into();
                        }
                    } else {
                        // Reduce space
                        (*curr_node).map.size = curr_map_size - size;
                    }

                    // Update current node
                    let new_curr = (*curr_node).prev.load(Ordering::Acquire);
                    if !new_curr.is_null() {
                        self.curr.store(new_curr, Ordering::Relaxed);
                    } else {
                        self.curr.store(self.head.load(Ordering::Relaxed), Ordering::Relaxed);
                    };
                    
                    // Update the used size of sram
                    self.sram_used.fetch_sub((*curr_node).map.size, Ordering::SeqCst);
                    break;
                } else {
                    if memory < (*curr_node).map.addr {
                        curr_node = (*curr_node).prev.load(Ordering::Acquire);
                    } else {
                        curr_node = (*curr_node).next.load(Ordering::Acquire);
                    }
                }
            }
        }
    }
    
    // Get size
    fn get_size(&mut self) -> u32 {
        let sram_start = self.sram_start.load(Ordering::Relaxed);
        let sram_ended = self.sram_ended.load(Ordering::Relaxed);
        sram_ended - sram_start
    }
    
    // Get used
    fn get_used(&mut self) -> u32 {
        let sram_used = self.sram_used.load(Ordering::Relaxed);
        sram_used
    }
    
    // Get curr addr
    fn get_curr_addr(&mut self) -> u32 {
        let curr_ptr = self.curr.load(Ordering::Relaxed);
        unsafe { (*curr_ptr).map.addr }
    }
}

// Struct ConcreteMemory
pub struct ConcreteMemory;

// Impl ConcreteMemory
impl ConcreteMemory {
    // New
    pub const fn new() -> Self {
        Self {}
    }

    // Setup
    pub fn setup(&mut self) {
        ALLOCATOR.memory.lock().initiate();

        // Output debug info
        kernel().debug().info("Memory setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        ALLOCATOR.memory.lock().exit();
    }
}

// Impl Memory for ConcreteMemory
impl Memory for ConcreteMemory {
    // Heap alloc
    fn heap_alloc(&mut self, size: u32) -> u32 {
        let alloc_addr = ALLOCATOR.memory.lock().heap_alloc(size);

        // Out of memory
        if alloc_addr == 0 {
            kernel().debug().error("out of memory.");
            loop {}
        }

        alloc_addr
    }
    
    // Stack alloc
    fn stack_alloc(&mut self, size: u32) -> u32 {
        let alloc_addr = ALLOCATOR.memory.lock().stack_alloc(size);

        // Out of memory
        if alloc_addr == 0 {
            kernel().debug().error("out of memory.");
            loop {}
        }

        alloc_addr
    }
    
    // Free
    fn free(&mut self, address: u32, size: u32) {
        ALLOCATOR.memory.lock().free(address, size);
    }
    
    // Get size
    fn get_size(&mut self) -> u32 {
        ALLOCATOR.memory.lock().get_size()
    }
    
    // Get used
    fn get_used(&mut self) -> u32 {
        ALLOCATOR.memory.lock().get_used()
    }
    
    // Get curr addr
    fn get_curr_addr(&mut self) -> u32 {
        ALLOCATOR.memory.lock().get_curr_addr()
    }
}

// Struct GlobalAllocator
struct GlobalAllocator {
    memory: Mutex<MemoryAllocator>,
}

// Set global allocator
#[global_allocator]
static ALLOCATOR: GlobalAllocator = GlobalAllocator {
    memory: Mutex::new(MemoryAllocator::new())
};

// Impl global alloc for global allocator
unsafe impl GlobalAlloc for GlobalAllocator {
    // Alloc
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_addr = self.memory.lock().heap_alloc(layout.size() as u32);
        
        if alloc_addr == 0 {
            panic!("out of memory.");
        }

        alloc_addr as *mut u8
    }

    // Dealloc
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            panic!("invalid memory ptr.");
        }

        self.memory.lock().free(ptr as u32, layout.size() as u32);
    }
}
