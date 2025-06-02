use crate::kernel::traits::vk_kernel::WorkQueue;

pub struct ConcreteWorkQueue;

impl WorkQueue for ConcreteWorkQueue {
    fn create(&self) {

    }

    fn delete(&self) {

    }

    fn sched(&self) {
        
    }
}
