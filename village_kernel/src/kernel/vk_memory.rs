//###########################################################################
// vk_memory.rs
// The specific implementation of functions related to memory
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Memory;
use core::ptr;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, Ordering};

const ALIGN: u32 = 4;
const KERNEL_RSVD_HEAP: u32 = 1024;
const KERNEL_RSVD_STACK: u32 = 1024;

// struct map
#[repr(C, align(4))]
struct Map {
    addr: u32,
    size: u32,
}

// impl map
impl Map {
    const fn new(addr: u32, size: u32) -> Self {
        Self { addr, size }
    }
}

// struct map node
#[repr(C, align(4))]
struct MapNode {
    map: Map,
    prev: AtomicPtr<MapNode>,
    next: AtomicPtr<MapNode>,
}

// impl map node
impl MapNode {
    const fn new(map: Map) -> Self {
        Self {
            map,
            prev: AtomicPtr::new(core::ptr::null_mut()),
            next: AtomicPtr::new(core::ptr::null_mut()),
        }
    }
}

// struct concrete memory
pub struct ConcreteMemory {
    sram_start: AtomicU32,
    sram_ended: AtomicU32,
    sram_used: AtomicU32,
    sbrk_heap: AtomicU32,

    head: AtomicPtr<MapNode>,
    tail: AtomicPtr<MapNode>,
    curr: AtomicPtr<MapNode>,
    
    initialized: AtomicBool,
}

// impl sync for conrete memory
unsafe impl Sync for ConcreteMemory {}

// impl conrete memory
impl ConcreteMemory {
    // new
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
        }
    }
}

// impl concrete memory
impl ConcreteMemory {
    // setup
    pub fn setup(&self) {
        // return when initialized
        if self.initialized.load(Ordering::Acquire) != false {
            return;
        }

        // initialize heap end at first call
        if self.sbrk_heap.load(Ordering::Relaxed) == 0 {
            // symbol defined in the linker script
            extern "C" {
                static _ebss: u32;
                static _estack: u32;
            }

            // calculate sram start and end address
            let sram_start = unsafe { &_ebss as *const u32 as u32 } + KERNEL_RSVD_HEAP;
            let sram_ended = unsafe { &_estack as *const u32 as u32 } + KERNEL_RSVD_STACK;

            // aligning sram_start and sram_ended by align byte
            let sram_start = align_up(sram_start, ALIGN);
            let sram_ended = align_down(sram_ended, ALIGN);
            
            // calculate sbrk stack address
            let sbrk_heap = unsafe { &_ebss as *const _ as u32 };
            
            // store value
            self.sram_start.store(sram_start, Ordering::Relaxed);
            self.sram_ended.store(sram_ended, Ordering::Relaxed);
            self.sbrk_heap.store(sbrk_heap, Ordering::Relaxed);
        }

        // initialize list, align 4 byts
        if self.head.load(Ordering::Relaxed).is_null() ||
           self.head.load(Ordering::Relaxed).is_null()
        {
            let size_of_node = core::mem::size_of::<MapNode>() as u32;
            let sram_start = self.sram_start.load(Ordering::Relaxed);
            let sram_ended = self.sram_ended.load(Ordering::Relaxed);

            // create head and tail
            let head =  sram_start as *mut MapNode;
            let tail = (sram_start + size_of_node) as *mut MapNode;
            
            // initialize head and tail node
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
            
            // store value
            self.head.store(head, Ordering::Relaxed);
            self.tail.store(tail, Ordering::Relaxed);
            self.curr.store(head, Ordering::Relaxed);
        }
        
        // set initialized flag
        self.initialized.store(true, Ordering::Release);
        
        // output debug info
        kernel().debug().info("Memory setup done!");
    }

    // exit
    pub fn exit(&self) {
        // clear initialized flag
        self.initialized.store(false, Ordering::Relaxed);
    }
}

// impl memory for concrete memory
impl Memory for ConcreteMemory {
    // heap alloc
    fn heap_alloc(&self, size: u32) -> u32 {
        let size_of_node = core::mem::size_of::<MapNode>() as u32;
        let mut curr_node = self.curr.load(Ordering::Acquire);
        let mut alloc_addr = 0;

        unsafe {
            // find free space
            while !curr_node.is_null() {
                let next_node = (*curr_node).next.load(Ordering::Relaxed);

                if !next_node.is_null() {
                    // calculate the next map size
                    let next_map_size = size_of_node + size;

                    // calculate the next map
                    let next_map_addr = (*curr_node).map.addr + (*curr_node).map.size;

                    // align memory by aligning allocation sizes
                    let next_map_size = align_up(next_map_size, ALIGN);

                    // align memory by aligning allocation addr
                    let next_map_addr = align_up(next_map_addr, ALIGN);

                    // calculate the end addr
                    let next_end_addr = next_map_addr + next_map_size;

                    // there is free space between the current node and the next node
                    if next_end_addr <= (*next_node).map.addr {
                        // update the used size of sram
                        self.sram_used.fetch_add(next_map_size, Ordering::SeqCst);

                        // add new node into list
                        let new_node = next_map_addr as *mut MapNode;
                        ptr::write(new_node, MapNode{
                            map: Map::new(next_map_addr, next_map_size),
                            prev: curr_node.into(),
                            next: next_node.into(),
                        });

                        // update list
                        (*curr_node).next.store(new_node, Ordering::Relaxed);
                        (*next_node).prev.store(new_node, Ordering::Relaxed);
                        
                        // update curr node
                        self.curr.store(new_node, Ordering::Release);

                        // calculate the alloc address
                        alloc_addr = next_map_addr + size_of_node;
                        break;
                    }
                }

                curr_node = next_node;
            }
        }

        // out of memory
        if alloc_addr == 0 {
            kernel().debug().error("out of memory.");
            loop {}
        }

        alloc_addr
    }
    
    // stack alloc
    fn stack_alloc(&self, size: u32) -> u32 {
        let mut curr_node = self.tail.load(Ordering::Acquire);
        let mut alloc_addr = 0;

        unsafe {
            // find free space
            while !curr_node.is_null() {
                let prev_node = (*curr_node).prev.load(Ordering::Acquire);

                if !prev_node.is_null() {
                    // calculate the prev map size
                    let prev_map_size = size;
                    
                    // calculate the prev map
                    let prev_map_addr = (*curr_node).map.addr - (*curr_node).map.size;

                    // align memory by aligning allocation sizes
                    let prev_map_size = align_up(prev_map_size, ALIGN);

                    // align memory by aligning allocation addr
                    let prev_map_addr = align_up(prev_map_addr, ALIGN);

                    // calculate the end addr
                    let prev_end_addr = prev_map_addr - prev_map_size;

                    // there is free space between the current node and the prev node
                    if prev_end_addr >= (*prev_node).map.addr {
                        // update the used size of sram
                        self.sram_used.fetch_add(prev_map_size, Ordering::SeqCst);

                        // add new node into list
                        let new_node = prev_map_addr as *mut MapNode;
                        ptr::write(new_node, MapNode{
                            map: Map::new(prev_map_addr, prev_map_size),
                            prev: prev_node.into(),
                            next: curr_node.into(),
                        });

                        // update list
                        (*prev_node).next.store(new_node, Ordering::Relaxed);
                        (*curr_node).prev.store(new_node, Ordering::Relaxed);

                        // calculate the alloc address
                        alloc_addr = prev_map_addr;
                        break;
                    }
                }

                curr_node = prev_node;
            }
        }

        // out of memory
        if alloc_addr == 0 {
            kernel().debug().error("out of memory.");
            loop {}
        }

        alloc_addr
    }
    
    // free
    fn free(&self, memory: u32, size: u32) {
        if memory == 0 { return; }

        let size_of_node = core::mem::size_of::<MapNode>() as u32;
        let mut curr_node = self.curr.load(Ordering::Acquire);

        unsafe {
            while !curr_node.is_null() {
            
                let curr_start_addr = (*curr_node).map.addr;
                let curr_ended_addr = (*curr_node).map.addr + (*curr_node).map.size;

                // break when the memory is between the end of the current node 
                // and the beginning of the next node, because it has been released
                let next_node = (*curr_node).next.load(Ordering::Acquire);
                if !next_node.is_null() {
                    let next_start_addr = (*next_node).map.addr;
                    if memory > curr_ended_addr && memory < next_start_addr {
                        break;
                    }
                }

                // release memory node
                if memory >= curr_start_addr && memory < curr_ended_addr {
                    let curr_map_size = (*curr_node).map.size;

                    if size == 0 || size_of_node == (curr_map_size - size) {
                        let prev_node = (*curr_node).prev.load(Ordering::Acquire);
                        let next_node = (*curr_node).next.load(Ordering::Acquire);
                        
                        // remove map node from list
                        if !prev_node.is_null() {
                            (*prev_node).next = next_node.into();
                        }
                        if !next_node.is_null() {
                            (*next_node).prev = prev_node.into();
                        }
                    } else {
                        // reduce space
                        (*curr_node).map.size = curr_map_size - size;
                    }

                    // select new current node
                    let prev_node = (*curr_node).prev.load(Ordering::Acquire);
                    let new_curr = if !prev_node.is_null() {
                        prev_node
                    } else {
                        self.head.load(Ordering::Relaxed)
                    };
                    
                    // update current node
                    if (*self.curr.load(Ordering::Relaxed)).map.addr > (*new_curr).map.addr {
                        self.curr.store(new_curr, Ordering::Release);
                    }
                    
                    // update the used size of sram
                    self.sram_used.fetch_sub((*curr_node).map.size, Ordering::SeqCst);
                    break;
                } else {
                    curr_node = if memory <  (*curr_node).map.addr {
                        (*curr_node).prev.load(Ordering::Acquire)
                    } else {
                        (*curr_node).next.load(Ordering::Acquire)
                    };
                }   
            }
        }
    }
    
    // get size
    fn get_size(&self) -> u32 {
        let sram_start = self.sram_start.load(Ordering::Relaxed);
        let sram_ended = self.sram_ended.load(Ordering::Relaxed);
        sram_ended - sram_start
    }
    
    // get used
    fn get_used(&self) -> u32 {
        let sram_used = self.sram_used.load(Ordering::Relaxed);
        sram_used
    }
    
    // get curr addr
    fn get_curr_addr(&self) -> u32 {
        let curr_ptr = self.curr.load(Ordering::Relaxed);
        unsafe { (*curr_ptr).map.addr }
    }
}

// align up
fn align_up(value: u32, align: u32) -> u32 {
    (value + align - 1) & !(align - 1)
}

// align down
fn align_down(value: u32, align: u32) -> u32 {
    value & !(align - 1)
}

// struct memory allocator
struct MemoryAllocator;

// set global allocator
#[global_allocator]
static ALLOCATOR: MemoryAllocator = MemoryAllocator;

// impl global alloc for memory allocator
unsafe impl GlobalAlloc for MemoryAllocator {
    // alloc
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
       kernel().memory().heap_alloc(layout.size() as u32) as *mut u8
    }

    // dealloc
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        kernel().memory().free(ptr as u32, layout.size() as u32);
    }
}
