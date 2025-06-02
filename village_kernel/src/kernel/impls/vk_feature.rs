use crate::kernel::traits::vk_kernel::Feature;

pub struct ConcreteFeature;

impl Feature for ConcreteFeature {
    fn register_module(&self) {

    }

    fn unregister_module(&self) {

    }

    fn get_module(&self, name: &str) {
        
    }
}
