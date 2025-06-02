use crate::kernel::traits::vk_kernel::Signal;

pub struct ConcreteSignal;

impl Signal for ConcreteSignal {
    fn raising(&self, signal: i32) {
        
    }
}
