use crate::kernel::traits::vk_kernel::Protocol;

pub struct ConcreteProtocol;

impl Protocol for ConcreteProtocol {
    fn register_stack(&self) {

    }

    fn unregister_stack(&self) {
        
    }
}
