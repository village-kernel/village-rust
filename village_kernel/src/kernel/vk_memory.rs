//###########################################################################
// vk_memory.rs
// The specific implementation of functions related to memory
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::ptr;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, Ordering};
use crate::village::kernel;
use crate::traits::vk_kernel::Memory;
use crate::misc::lock::vk_spinlock::SpinLock;

const ALIGN: u32 = 4;
const KERNEL_RSVD_HEAP: u32 = 1024;
const KERNEL_RSVD_STACK: u32 = 4096;

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
pub struct ConcreteMemory {
    sram_start: AtomicU32,
    sram_ended: AtomicU32,
    sram_used: AtomicU32,
    sbrk_heap: AtomicU32,

    head: AtomicPtr<MapNode>,
    tail: AtomicPtr<MapNode>,
    curr: AtomicPtr<MapNode>,

    initialized: AtomicBool,

    lock: SpinLock,
}

// Impl sync for conrete memory
unsafe impl Sync for ConcreteMemory {}

// Impl conrete memory
impl ConcreteMemory {
    // New
    pub const fn new() -> Self {
        Self {
            sram_start: AtomicU32::new(0),
            sram_ended: AtomicU32::new(0),
            sram_used: AtomicU32::new(0),
            sbrk_heap: AtomicU32::new(0),

            head: AtomicPtr::new(core::ptr::null_mut()),
            tail: AtomicPtr::new(core::ptr::null_mut()),
            curr: AtomicPtr::new(core::ptr::null_mut()),

            initialized: AtomicBool::new(false),

            lock: SpinLock::new(),
        }
    }
}

// Impl concrete memory
impl ConcreteMemory {
    // Setup
    pub fn setup(&mut self) {
        // Return when initialized
        if self.initialized.load(Ordering::Acquire) == true {
            return;
        }

        // Initialize heap end at first call
        if self.sbrk_heap.load(Ordering::Relaxed) == 0 {
            // Symbol defined in the linker script
            extern "C" {
                static _ebss: u32;
                static _estack: u32;
            }

            // Calculate sram start and end address
            let sram_start = unsafe { &_ebss as *const u32 as u32 } + KERNEL_RSVD_HEAP;
            let sram_ended = unsafe { &_estack as *const u32 as u32 } - KERNEL_RSVD_STACK;

            // Aligning sram_start and sram_ended by align byte
            let sram_start = align_up(sram_start, ALIGN);
            let sram_ended = align_down(sram_ended, ALIGN);
            
            // Calculate sbrk stack address
            let sbrk_heap = unsafe { &_ebss as *const _ as u32 };
            
            // Store value
            self.sram_start.store(sram_start, Ordering::Relaxed);
            self.sram_ended.store(sram_ended, Ordering::Relaxed);
            self.sbrk_heap.store(sbrk_heap, Ordering::Relaxed);
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
        
        // Output debug info
        kernel().debug().info("Memory setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear initialized flag
        self.initialized.store(false, Ordering::Relaxed);
    }
}

// Impl memory for concrete memory
impl Memory for ConcreteMemory {
    // Heap alloc
    fn heap_alloc(&mut self, size: u32) -> u32 {
        // Check is initialized
        if !self.initialized.load(Ordering::Acquire) {
            self.setup();
        }

        self.lock.lock();

        let size_of_node = core::mem::size_of::<MapNode>() as u32;
        let mut curr_node = self.curr.load(Ordering::Acquire);
        let mut alloc_addr = 0;

        unsafe {
            // Find free space
            while !curr_node.is_null() {
                let next_node = (*curr_node).next.load(Ordering::Relaxed);

                if !next_node.is_null() {
                    // Calculate the new node map size
                    let new_map_size = size_of_node + size;

                    // Calculate the new node map
                    let new_map_addr = (*curr_node).map.addr + (*curr_node).map.size;

                    // Align memory by aligning allocation sizes
                    let new_map_size = align_up(new_map_size, ALIGN);

                    // Align memory by aligning allocation addr
                    let new_map_addr = align_up(new_map_addr, ALIGN);

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
                }

                curr_node = next_node;
            }
        }

        self.lock.unlock();

        // Out of memory
        if alloc_addr == 0 {
            kernel().debug().error("out of memory.");
            loop {}
        }

        alloc_addr
    }
    
    // Stack alloc
    fn stack_alloc(&mut self, size: u32) -> u32 {
        // Check is initialized
        if !self.initialized.load(Ordering::Acquire) {
            self.setup();
        }

        // Create an new node by heap alloc
        let size_of_node = core::mem::size_of::<MapNode>() as u32;
        let new_node = self.heap_alloc(size_of_node) as *mut MapNode;

        self.lock.lock();

        let mut curr_node = self.tail.load(Ordering::Acquire);
        let mut alloc_addr = 0;

        unsafe {
            // Find free space
            while !curr_node.is_null() {
                let prev_node = (*curr_node).prev.load(Ordering::Acquire);

                if !prev_node.is_null() {
                    // Calculate the new map size
                    let new_map_size = size;
                    
                    // Calculate the new map
                    let new_map_addr = (*curr_node).map.addr - (*curr_node).map.size;

                    // Align memory by aligning allocation sizes
                    let new_map_size = align_up(new_map_size, ALIGN);

                    // Align memory by aligning allocation addr
                    let new_map_addr = align_up(new_map_addr, ALIGN);

                    // Calculate the end addr
                    let new_end_addr = new_map_addr - new_map_size;

                    // There is free space between the current node and the prev node
                    if new_end_addr >= (*prev_node).map.addr {
                        // Update the used size of sram
                        self.sram_used.fetch_add(new_map_size, Ordering::SeqCst);

                        // Create an new node
                        //let new_node = self.heap_alloc(core::mem::size_of::<MapNode>() as u32) as *mut MapNode;
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
                        let new_curr = (*new_node).prev.load(Ordering::Acquire);
                        if !new_curr.is_null() {
                            self.curr.store(new_curr, Ordering::Relaxed);
                        } else {
                            self.curr.store(self.head.load(Ordering::Relaxed), Ordering::Relaxed);
                        }

                        // Calculate the alloc address
                        alloc_addr = new_map_addr;
                        break;
                    }
                }

                curr_node = prev_node;
            }
        }

        self.lock.unlock();

        // Out of memory
        if alloc_addr == 0 {
            kernel().debug().error("out of memory.");
            loop {}
        }

        alloc_addr
    }
    
    // Free
    fn free(&mut self, memory: u32, size: u32) {
        if memory == 0 { return; }

        self.lock.lock();

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

        self.lock.unlock();
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

// Align up
fn align_up(value: u32, align: u32) -> u32 {
    (value + align - 1) & !(align - 1)
}

// Align down
fn align_down(value: u32, align: u32) -> u32 {
    value & !(align - 1)
}

// Struct memory allocator
struct MemoryAllocator;

// Set global allocator
#[global_allocator]
static ALLOCATOR: MemoryAllocator = MemoryAllocator;

// Impl global alloc for memory allocator
unsafe impl GlobalAlloc for MemoryAllocator {
    // Alloc
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
       kernel().memory().heap_alloc(layout.size() as u32) as *mut u8
    }

    // Dealloc
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        kernel().memory().free(ptr as u32, layout.size() as u32);
    }
}
