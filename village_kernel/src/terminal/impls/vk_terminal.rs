use crate::kernel::traits::vk_kernel::Terminal;

pub struct ConcreteTerminal;

impl Terminal for ConcreteTerminal {
    fn register_cmd(&self) {

    }

    fn unregister_cmd(&self) {
        
    }
}
