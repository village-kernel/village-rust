//###########################################################################
// vk_kernel.rs
// The specific implementation of functions related to village kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::ffi::c_void;
use core::cell::UnsafeCell;
use crate::vk_kernel::Kernel;

// Type aliases for start entry
type DynKernel = fn() -> &'static mut dyn Kernel;

// Village kernel
pub struct VillageKernel {
    inner: UnsafeCell<Option<DynKernel>>,
}

// Impl village kernel
impl VillageKernel {
    // new village kernel
    const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    // Erase a function pointer to a start entry
    fn dyn_kernel(src: *const c_void) -> DynKernel {
        unsafe { core::mem::transmute::<*const c_void, DynKernel>( src ) }
    }

    // set village kernel
    pub fn set(&'static self, village: *const c_void) {
        unsafe { *self.inner.get() = Some(Self::dyn_kernel(village)); }
    }

    // get village kernel
    pub fn get(&'static self) -> DynKernel {
        unsafe {
            *(*self.inner.get()).as_mut().unwrap()
        }
    }
}

// Impl sync for village kernel
unsafe impl Sync for VillageKernel{}

// Static village kernel
static VILLAGE_KERNEL: VillageKernel = VillageKernel::new();

// Set village kernel
#[unsafe(no_mangle)]
pub fn set_kernel(village: *const c_void) {
    VILLAGE_KERNEL.set(village);
}

// Get village kernel
pub fn kernel() -> &'static mut dyn Kernel {
    (VILLAGE_KERNEL.get())()
}
