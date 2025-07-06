//###########################################################################
// village.rs
// The specific implementation of functions related to village kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::kernel::vk_village::Village;
use crate::traits::vk_kernel::Kernel;
use alloc::boxed::Box;
use core::cell::UnsafeCell;
use spin::Once;

/// Village kernel
pub struct VillageKernel {
    inner: UnsafeCell<Option<Box<Village>>>,
    initialized: Once<()>,
}

// Impl village kernel
impl VillageKernel {
    // new village kernel
    const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
            initialized: Once::new(),
        }
    }

    // get village kernel
    pub fn get(&'static self) -> &'static mut dyn Kernel {
        self.initialized.call_once(|| unsafe {
            *self.inner.get() = Some(Box::new(Village::new()));
        });
        unsafe { (*self.inner.get()).as_mut().unwrap().as_mut() as &'static mut dyn Kernel }
    }
}

// Impl sync for village kernel
unsafe impl Sync for VillageKernel {}

// Static village kernel
static VILLAGE_KERNEL: VillageKernel = VillageKernel::new();

// Get village kernel
pub fn kernel() -> &'static mut dyn Kernel {
    VILLAGE_KERNEL.get()
}
