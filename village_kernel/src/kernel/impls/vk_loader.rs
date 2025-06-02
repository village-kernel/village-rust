use crate::kernel::traits::vk_kernel::Loader;

pub struct ConcreteLoader;

impl Loader for ConcreteLoader {
    fn load(&self) {

    }

    fn unload(&self) {
        
    }
}
