//###########################################################################
// vk_callback.rs
// The specific implementation of functions related to callback
//
// $Copyright: Copyright (C) village
//###########################################################################
// Type aliases for function callback
pub type FnCallback = extern "C" fn(*mut (), *mut ());

// Erase a function pointer to a callback function
fn convert_to_cb(fn_addr: u32) -> FnCallback {
    unsafe { core::mem::transmute::<u32, FnCallback>(fn_addr) }
}

// Structure to hold callback function, instance, and userdata
pub struct Callback {
    pub callback: FnCallback,
    pub instance: *mut (),
    pub userdata: *mut (),
}

impl Callback {
    // Create a new callback
    pub fn new(fn_addr: u32) -> Self {
        Self {
            callback: convert_to_cb(fn_addr),
            instance: core::ptr::null_mut(),
            userdata: core::ptr::null_mut(),
        }
    }

    // Set the instance
    pub fn with_instance<T>(mut self, instance: &mut T) -> Self {
        self.instance = instance as *mut T as *mut ();
        self
    }

    // Set the userdata
    pub fn with_userdata<T>(mut self, userdata: &mut T) -> Self {
        self.userdata = userdata as *mut T as *mut ();
        self
    }

    // Call the registered callback
    pub fn call(&mut self) {
        (self.callback)(self.instance, self.userdata);
    }
}

// Impl partialeq for callback
impl PartialEq for Callback {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::fn_addr_eq(self.callback, other.callback)
            && self.instance == other.instance
            && self.userdata == other.userdata
    }
}
