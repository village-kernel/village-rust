//###########################################################################
// village.rs
// The specific implementation of functions related to village kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel::vk_village::VillageKernel;
use crate::traits::vk_kernel::Kernel;
use alloc::boxed::Box;
use core::cell::UnsafeCell;
use spin::Once;

/// Village instance
pub struct VillageInstance {
    inner: UnsafeCell<Option<Box<VillageKernel>>>,
    initialized: Once<()>,
}

// Impl village instance
impl VillageInstance {
    // new village instance
    const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
            initialized: Once::new(),
        }
    }

    // get village instance
    pub fn get(&'static self) -> &'static mut dyn Kernel {
        self.initialized.call_once(|| unsafe {
            *self.inner.get() = Some(Box::new(VillageKernel::new()));
        });
        unsafe { (*self.inner.get()).as_mut().unwrap().as_mut() as &'static mut dyn Kernel }
    }
}

// Impl sync for village instance
unsafe impl Sync for VillageInstance {}

// Static village instance
static VILLAGE_INSTANCE: VillageInstance = VillageInstance::new();

// Get village instance
pub fn kernel() -> &'static mut dyn Kernel {
    VILLAGE_INSTANCE.get()
}
