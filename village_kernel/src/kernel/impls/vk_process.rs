use crate::kernel::traits::vk_kernel::Process;

pub struct ConcreteProcess;

impl Process for ConcreteProcess {
    fn register_executor(&self) {

    }
    
    fn unregister_executor(&self) {

    }
}
