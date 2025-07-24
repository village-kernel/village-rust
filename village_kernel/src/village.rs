//###########################################################################
// village.rs
// The specific implementation of functions related to village instance
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use core::cell::UnsafeCell;
use crate::traits::vk_kernel::Kernel;
use crate::kernel::vk_village::VillageKernel;

// Village instance
pub struct VillageInstance {
    inner: UnsafeCell<Option<Box<VillageKernel>>>,
}

// Impl village instance
impl VillageInstance {
    // new village instance
    const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    // set village instance
    pub fn set(&'static self) {
        unsafe {
            *self.inner.get() = Some(Box::new(VillageKernel::new()));
        }
    }

    // get village instance
    pub fn get(&'static self) -> &'static mut dyn Kernel {
        unsafe {
            (*self.inner.get())
                .as_mut()
                .unwrap()
                .as_mut() as &'static mut dyn Kernel 
        }
    }
}

// Impl sync for village instance
unsafe impl Sync for VillageInstance {}

// Static village instance
static VILLAGE_INSTANCE: VillageInstance = VillageInstance::new();

// Set village instance
#[unsafe(no_mangle)]
pub fn init_kernel() {
    VILLAGE_INSTANCE.set();
}

// Get village instance
pub fn kernel() -> &'static mut dyn Kernel {
    VILLAGE_INSTANCE.get()
}
