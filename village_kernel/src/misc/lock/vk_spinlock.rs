//###########################################################################
// vk_spinlock.rs
// The specific implementation of functions related to spinlock
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::sync::atomic::{AtomicBool, Ordering};

// struct spin lock
pub struct SpinLock {
    lock: AtomicBool,
}

// impl spin lock
impl SpinLock {
    // new
    pub const fn new() -> Self {
        Self {
            lock: AtomicBool::new(false),
        }
    }

    // lock
    pub fn lock(&self) {
        while self
            .lock
            .compare_exchange(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_err()
        {
            core::hint::spin_loop();
        }
    }

    // unlock
    pub fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}
