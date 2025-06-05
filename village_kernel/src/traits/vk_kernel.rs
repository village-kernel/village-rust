//###########################################################################
// vK_kernel.rs
// The interfaces of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_callback::{FnCallback, MethodCb};

// System
pub trait System {
    fn systick_counter(&self);
    fn get_sysclk_counts(&self) -> u32;
    fn delay_ms(&self, millis: u32);

    fn enable_irq(&self);
    fn disable_irq(&self);

    fn sleep(&self);
    fn standby(&self);
    fn shutdown(&self);
    fn reboot(&self);
}

// Memory
pub trait Memory {
    // Alloc Methods
    fn heap_alloc(&self, size: u32) -> u32;
    fn stack_alloc(&self, size: u32) -> u32;
    fn free(&self, address: u32, size: u32);

    // Info Methods
    fn get_size(&self) -> u32;
    fn get_used(&self) -> u32;
    fn get_curr_addr(&self) -> u32;
}

// Debug
pub trait Debug {
    fn log(&self, log: &str);
    fn info(&self, log: &str);
    fn error(&self, error: &str);
    fn warn(&self, warn: &str);
    fn output(&self, level: i32, msg: &str);
}

// Interrupt
pub trait Interrupt  {
    // Set ISR Methods
    fn set_isr_fn_cb(&self, irq: isize, func: FnCallback, args: *mut());
    fn set_isr_meth_cb(&mut self, irq: isize, method: MethodCb, data: *mut ());
    
    // Append ISR Methods
    fn add_isr_fn_cb(&self, irq: isize, func: FnCallback, args: *mut());
    fn add_isr_meth_cb(&mut self, irq: isize, method: MethodCb, data: *mut ());
    
    // Remove ISR Methods
    fn del_isr_fn_cb(&self, irq: isize, func: FnCallback, args: *mut());
    fn del_isr_meth_cb(&mut self, irq: isize, method: MethodCb, data: *mut ());    
    
    // Clear ISR Methods
    fn clear_isr_cb(&self, irq: isize);
    
    // Replace Methods
    fn replace(&self, handler: usize);
    
    // Feature Methods
    fn handler(&self, irq: isize);
}

// Scheduler
pub trait Scheduler {
    fn start(&self);
    fn sched(&self);
}

// Thread
pub trait Thread {
    fn create_task(&self) -> i32;
    
    fn get_task_id(&self) -> i32;
    fn start_task(&self, tid: i32) -> bool;
    fn stop_task(&self, tid: i32) -> bool;
    fn wait_for_task(&self, tid: i32) -> bool;
    fn exit_blocked(&self, tid: i32) -> bool;
    fn delete_task(&self, tid: i32) -> bool;
    fn is_task_alive(&self, tid: i32) -> bool;
    fn get_tasks(&self);

    fn change_state(&self);
    fn sleep(&self);
    fn blocked(&self);
    fn task_exit(&self);

    fn save_task_psp(&self, psp: u32);
    fn get_task_psp(&self) -> u32;
    fn select_next_task(&self);
    fn idle_task(&self);
}

// Symbol
pub trait Symbol {
    fn export(&self, sym_addr: u32, name: &str);
    fn unexport(&self, name: &str);
    fn search(&self, name: &str);
}

// Device
pub trait Device {
    fn register_block_device(&self);
    fn unregister_block_device(&self);
}

// Feature
pub trait Feature {
    fn register_module(&self);
    fn unregister_module(&self);
    fn get_module(&self, name: &str);
}

// FileSystem
pub trait FileSystem {
    fn mount_hard_drive(&self, disk: &str) -> bool;
    fn unmount_hard_drive(&self, disk: &str) -> bool;

    fn register_fs(&self, name: &str);
    fn unregister_fs(&self, name: &str);

    fn get_volume(&self, name: &str);
}

// WorkQueue
pub trait WorkQueue {
    fn create(&self);

    fn delete(&self);
    fn sched(&self);
}

// Event
pub trait Event {
    fn init_input_device(&self);
    fn exit_input_device(&self);
}

// Loader
pub trait Loader {
    fn load(&self);
    fn unload(&self);
}

// Process
pub trait Process {
    fn register_executor(&self);
    fn unregister_executor(&self);
}

// Timer
pub trait Timer {
    fn create(&self);
    fn modify(&self);
    fn delete(&self);
}

// Terminal
pub trait Terminal {
    fn register_cmd(&self);
    fn unregister_cmd(&self);
}

// Signal
pub trait Signal {
    fn raising(&self, signal: i32);
}

// Protocol
pub trait Protocol {
    fn register_stack(&self);
    fn unregister_stack(&self);
}

// Kernel
pub trait Kernel {
    fn setup(&self);
    fn start(&self);
    fn exit(&self);

    fn sleep(&self);
    fn standby(&self);
    fn shutdown(&self);
    fn reboot(&self);

    fn get_build_date(&self);
    fn get_build_time(&self);
    fn get_build_version(&self);
    fn get_build_git_sha(&self);

    fn system(&self) -> &dyn System;
    fn memory(&self) -> &dyn Memory;
    fn debug(&self) -> &dyn Debug;
    fn interrupt(&self) -> &dyn Interrupt;
    fn scheduler(&self) -> &dyn Scheduler;
    fn thread(&self) -> &dyn Thread;
    fn workqueue(&self) -> &dyn WorkQueue;
    fn event(&self) -> &dyn Event;
    fn symbol(&self) -> &dyn Symbol;
    fn device(&self) -> &dyn Device;
    fn feature(&self) -> &dyn Feature;
    fn filesys(&self) -> &dyn FileSystem;
    fn loader(&self) -> &dyn Loader;
    fn process(&self) -> &dyn Process;
    fn timer(&self) -> &dyn Timer;
    fn terminal(&self) -> &dyn Terminal;
    fn signal(&self) -> &dyn Signal;
    fn protocol(&self) -> &dyn Protocol;
}

// Declarations kernel pointer
static mut KERNEL: Option<&'static dyn Kernel> = None;

// Init kernel
pub fn init_kernel(kernel: &'static dyn Kernel) {
    unsafe { KERNEL = Some(kernel); }
}

// Get Kernel
pub fn kernel() -> &'static dyn Kernel {
    unsafe { KERNEL.expect("Kernel not initialized") }
}
