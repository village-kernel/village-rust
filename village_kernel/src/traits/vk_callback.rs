//###########################################################################
// vk_class.rs
// The specific implementation of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::boxed::Box;
use core::any::Any;

// Type aliases for callback instance and function
pub type CbInstance = Box<dyn Any>;
pub type CbFunction = Box<dyn for<'a> FnMut(&'a mut dyn Any, *mut ())>;

// Type aliases for function callback
pub type FnCallback = fn(*mut ());

// No user data
pub fn no_user_data<T>() -> *mut T {
    core::ptr::null_mut()
}

// Erase a function pointer to a callback function
pub fn convert_fn_to_cb(func: FnCallback) -> CbFunction {
    Box::new(move |_: &mut dyn Any, data_ptr| {
        (func)(data_ptr);
    })
}

// Type-erased callback wrapper struct that encapsulates 
// a callable instance and its invocation logic
pub struct MethodCb {
    instance: CbInstance,
    callback: CbFunction,
}

impl MethodCb {
    pub fn new<T: 'static + Callback>(instance: &mut T, method: fn(&mut T, *mut ())) -> Self {
        let instance_ptr = instance as *mut T;
        let instance_box = unsafe { Box::from_raw(instance_ptr) };

        MethodCb {
            instance: instance_box as CbInstance,
            callback: Box::new(move |obj: &mut dyn Any, data| {
                if let Some(concrete_obj) = obj.downcast_mut::<T>() {
                    (method)(concrete_obj, data);
                }
            }),
        }
    }
}

// Callback trait that all callable types must implement
pub trait Callback: Any {
    // fn to_cb template
    // fn to_cb(&mut self, method: fn(&mut Self, *mut ())) -> MethodCb {
    //     MethodCb::new(self, method)
    // }
    fn to_cb(&mut self, method: fn(&mut Self, *mut ())) -> MethodCb;
}

// Structure to hold callback function, instance, and user data
pub struct CbInvoker {
    callback: Option<CbFunction>,
    instance: Option<CbInstance>,
    userdata: *mut (),
}

impl CbInvoker {
    // Create a new empty callback invoker
    pub fn new() -> Self {
        Self {
            callback: None,
            instance: None,
            userdata: core::ptr::null_mut(),
        }
    }

    // Register a function pointer-based callback
    pub fn register_fn(&mut self, func: FnCallback) {
        self.callback = Some(convert_fn_to_cb(func));
        self.instance = Some(Box::new(()));
        self.userdata = no_user_data();
    }

    // Register a function pointer-based callback with data
    pub fn register_fn_with_data(&mut self, func: FnCallback, data: *mut ()) {
        self.callback = Some(convert_fn_to_cb(func));
        self.instance = Some(Box::new(()));
        self.userdata = data;
    }

    // Register a class method-based callback
    pub fn register_method(&mut self, method: MethodCb) {
        self.callback = Some(method.callback);
        self.instance = Some(method.instance);
        self.userdata = no_user_data();
    }

    // Register a class method-based callback with data
    pub fn register_method_with_data(&mut self, method: MethodCb, data: *mut ()) {
        self.callback = Some(method.callback);
        self.instance = Some(method.instance);
        self.userdata = data;
    }

    // Invoke the registered callback
    pub fn invoke(&mut self) {
        if let (Some(ref mut callback), Some(ref mut instance)) =
            (&mut self.callback, &mut self.instance)
        {
            callback(instance.as_mut(), self.userdata);
        }
    }
}
