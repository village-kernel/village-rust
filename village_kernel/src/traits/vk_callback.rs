//###########################################################################
// vk_class.rs
// The specific implementation of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::boxed::Box;
use core::any::Any;

// Callback trait that all callable types must implement
pub trait Callback: Any { }

// Type aliases for callback instance and function
pub type CbInstance = Box<dyn Any>;
pub type CbFunction = Box<dyn for<'a> FnMut(&'a mut dyn Any, *mut ())>;

// Type aliases for function callback and method callback
pub type FnCallback = fn(*mut ());
pub type MethodCb<T> = fn(&mut T, *mut ());

// Erase a function pointer to a callback function
pub fn convert_fn_to_cb(func: FnCallback) -> CbFunction {
    Box::new(move |_: &mut dyn Any, data_ptr| {
        (func)(data_ptr);
    })
}

// Erase a method pointer to a callback function
pub fn convert_method_to_cb<T: 'static + Callback>(method: MethodCb<T>) -> CbFunction {
    Box::new(move |obj: &mut dyn Any, data_ptr| {
        if let Some(concrete_obj) = obj.downcast_mut::<T>() {
            (method)(concrete_obj, data_ptr);
        }
    })
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
    pub fn register_fn(&mut self, func: FnCallback, data: *mut ()) {
        self.callback = Some(convert_fn_to_cb(func));
        self.instance = Some(Box::new(()));
        self.userdata = data;
    }

    // Register a class method-based callback
    pub fn register_method<T: 'static + Callback>(&mut self, instance: T, method: MethodCb<T>, data: *mut ()) {
        self.callback = Some(convert_method_to_cb(method));
        self.instance = Some(Box::new(instance));
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
