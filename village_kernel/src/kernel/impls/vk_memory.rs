use crate::kernel::traits::vk_kernel::Memory;

pub struct ConcreteMemory;

impl Memory for ConcreteMemory {
    fn heap_alloc(&self, size: u32) -> u32 {
        0
    }
    
    fn stack_alloc(&self, size: u32) -> u32 {
        0
    }

    fn free(&self, address: u32, size: u32) {
        
    }

    fn get_size(&self) -> u32 {
        0
    }

    fn get_used(&self) -> u32 {
        0
    }

    fn get_curr_addr(&self) -> u32 {
        0
    }
}
