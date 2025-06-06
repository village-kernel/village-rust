//###########################################################################
// village.rs
// The specific implementation of functions related to village kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use spin::Once;
use core::cell::UnsafeCell;
use crate::traits::vk_kernel::Kernel;
use crate::kernel::vk_village::Village;

/// Village kernel
pub struct VillageKernel {
    inner: UnsafeCell<Option<Village>>,
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
        self.initialized.call_once(|| {
            unsafe { *self.inner.get() = Some(Village::new()); }
        });
        unsafe {
            (*self.inner.get()).as_mut().unwrap() as &'static mut Village
        }
    }
}

// Impl sync for village kernel
unsafe impl Sync for VillageKernel{}

// Static village kernel
static VILLAGE_KERNEL: VillageKernel = VillageKernel::new();

// Get village kernel
pub fn kernel() -> &'static mut dyn Kernel {
    VILLAGE_KERNEL.get()
}
