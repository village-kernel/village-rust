use crate::kernel::traits::vk_kernel::FileSystem;

pub struct ConcreteFileSystem;

impl FileSystem for ConcreteFileSystem {
    fn mount_hard_drive(&self, disk: &str) -> bool {
        false
    }

    fn unmount_hard_drive(&self, disk: &str) -> bool {
        false
    }

    fn register_fs(&self, name: &str) {

    }

    fn unregister_fs(&self, name: &str) {

    }

    fn get_volume(&self, name: &str) {
        
    }
}
