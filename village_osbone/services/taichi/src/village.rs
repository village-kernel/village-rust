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

// Village instance
pub struct VillageInstance {
    inner: UnsafeCell<Option<DynKernel>>,
}

// Impl village instance
impl VillageInstance {
    // new village instance
    const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    // Erase a function pointer to a start entry
    fn dyn_kernel(src: *const c_void) -> DynKernel {
        unsafe { core::mem::transmute::<*const c_void, DynKernel>( src ) }
    }

    // set village instance
    pub fn set(&'static self, village: *const c_void) {
        unsafe { *self.inner.get() = Some(Self::dyn_kernel(village)); }
    }

    // get village instance
    pub fn get(&'static self) -> DynKernel {
        unsafe {
            *(*self.inner.get()).as_mut().unwrap()
        }
    }
}

// Impl sync for village instance
unsafe impl Sync for VillageInstance{}

// Static village instance
static VILLAGE_INSTANCE: VillageInstance = VillageInstance::new();

// Set village instance
#[unsafe(no_mangle)]
pub fn set_kernel(village: *const c_void) {
    VILLAGE_INSTANCE.set(village);
}

// Get village instance
pub fn kernel() -> &'static mut dyn Kernel {
    (VILLAGE_INSTANCE.get())()
}
