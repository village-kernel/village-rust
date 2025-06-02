use crate::kernel::traits::vk_kernel::Device;

pub struct ConcreteDevice;

impl Device for ConcreteDevice {
    fn register_block_device(&self) {

    }

    fn unregister_block_device(&self) {

    }
}
