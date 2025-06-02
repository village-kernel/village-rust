use crate::kernel::traits::vk_kernel::Event;

pub struct ConcreteEvent;

impl Event for ConcreteEvent {
    fn init_input_device(&self) {

    }

    fn exit_input_device(&self) {

    }
}
