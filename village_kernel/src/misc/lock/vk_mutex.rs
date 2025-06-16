//###########################################################################
// vk_mutex.rs
// The specific implementation of functions related to mutex
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::kernel;

// struct mutex
pub struct Mutex {
    lock: AtomicBool,
    ticks: AtomicU32,
}

// impl mutex
impl Mutex {
    // new
    pub const fn new() -> Self {
        Self {
            lock: AtomicBool::new(false),
            ticks: AtomicU32::new(0),
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
            let ticks = self.ticks.load(Ordering::Acquire);
            kernel().thread().sleep(ticks);
        }
    }

    // unlock
    pub fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}
