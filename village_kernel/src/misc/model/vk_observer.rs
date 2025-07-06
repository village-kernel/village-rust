//###########################################################################
// vk_observer.rs
// The specific implementation of functions related to observer
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_callback::Callback;
use crate::traits::vk_linkedlist::LinkedList;

// Struct ObserverModel
pub struct ObserverModel {
    observers: LinkedList<Callback>,
}

// Impl ObserverModel
impl ObserverModel {
    // New
    pub const fn new() -> Self {
        Self {
            observers: LinkedList::new(),
        }
    }

    // Attach
    pub fn attach(&mut self, callback: Callback) {
        self.observers.push(callback);
    }

    // Detach
    pub fn detach(&mut self, callback: Callback) {
        self.observers.retain_mut(|cb| {
            !(cb.instance == callback.instance
                && core::ptr::fn_addr_eq(cb.callback, callback.callback))
        });
    }

    // Notify
    pub fn notify<T>(&mut self, argv: &mut T) {
        for observer in self.observers.iter_mut() {
            observer.userdata = argv as *mut T as *mut ();
            observer.call();
        }
    }

    // Clear
    pub fn clear(&mut self) {
        self.observers.clear();
    }
}
