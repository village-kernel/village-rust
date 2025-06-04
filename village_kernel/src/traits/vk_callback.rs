//###########################################################################
// vk_class.rs
// The specific implementation of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::boxed::Box;
use core::any::Any;

// callback trait
pub trait Callback: Any { }

// typedef callback instance, function and method
pub type Instance = Box<dyn Any>;
pub type Function = Box<dyn for<'a> FnMut(&'a mut dyn Any, *mut ())>;
pub type Method<T> = fn(&mut T, *mut ());

// erase user to instance
pub fn erase_user_instance<T: 'static + Callback>(instance: T) -> Instance {
    Box::new(instance)
}

// erase method to callback
pub fn erase_method_callback<T: 'static + Callback>(method: Method<T>) -> Function {
    Box::new(move |obj: &mut dyn Any, data_ptr| {
        if let Some(concrete_obj) = obj.downcast_mut::<T>() {
            (method)(concrete_obj, data_ptr);
        }
    })
}

// invoke callback
pub fn invoke_callback(callback: &mut Option<Function>, instance: &mut Option<Instance>, data: *mut ())
{
    if let (Some(ref mut callback), Some(ref mut instance)) =
        (callback, instance)
    {
        callback(instance.as_mut(), data);
    }
}
