//###########################################################################
// vK_kernel.rs
// The interfaces of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_callback::{FnCallback, MethodCb};

// System
pub trait System {
    fn systick_counter(&mut self);
    fn get_sysclk_counts(&mut self) -> u32;
    fn delay_ms(&mut self, millis: u32);

    fn enable_irq(&mut self);
    fn disable_irq(&mut self);

    fn sleep(&mut self);
    fn standby(&mut self);
    fn shutdown(&mut self);
    fn reboot(&mut self);
}

// Memory
pub trait Memory {
    // Alloc Methods
    fn heap_alloc(&mut self, size: u32) -> u32;
    fn stack_alloc(&mut self, size: u32) -> u32;
    fn free(&mut self, address: u32, size: u32);

    // Info Methods
    fn get_size(&mut self) -> u32;
    fn get_used(&mut self) -> u32;
    fn get_curr_addr(&mut self) -> u32;
}

// Debug
pub trait Debug {
    fn log(&mut self, log: &str);
    fn info(&mut self, log: &str);
    fn error(&mut self, error: &str);
    fn warn(&mut self, warn: &str);
    fn output(&mut self, level: i32, msg: &str);
}

// Interrupt
pub trait Interrupt  {
    // Set ISR Methods
    fn set_isr_fn_cb(&mut self, irq: isize, func: FnCallback, args: *mut());
    fn set_isr_meth_cb(&mut self, irq: isize, method: MethodCb, data: *mut ());
    
    // Append ISR Methods
    fn add_isr_fn_cb(&mut self, irq: isize, func: FnCallback, args: *mut());
    fn add_isr_meth_cb(&mut self, irq: isize, method: MethodCb, data: *mut ());
    
    // Remove ISR Methods
    fn del_isr_fn_cb(&mut self, irq: isize, func: FnCallback, args: *mut());
    fn del_isr_meth_cb(&mut self, irq: isize, method: MethodCb, data: *mut ());    
    
    // Clear ISR Methods
    fn clear_isr_cb(&mut self, irq: isize);
    
    // Replace Methods
    fn replace(&mut self, handler: usize);
    
    // Feature Methods
    fn handler(&mut self, irq: isize);
}

// Scheduler
pub trait Scheduler {
    fn start(&mut self);
    fn sched(&mut self);
}

// Thread
pub trait Thread {
    fn create_task(&mut self) -> i32;
    
    fn get_task_id(&mut self) -> i32;
    fn start_task(&mut self, tid: i32) -> bool;
    fn stop_task(&mut self, tid: i32) -> bool;
    fn wait_for_task(&mut self, tid: i32) -> bool;
    fn exit_blocked(&mut self, tid: i32) -> bool;
    fn delete_task(&mut self, tid: i32) -> bool;
    fn is_task_alive(&mut self, tid: i32) -> bool;
    fn get_tasks(&mut self);

    fn change_state(&mut self);
    fn sleep(&mut self);
    fn blocked(&mut self);
    fn task_exit(&mut self);

    fn save_task_psp(&mut self, psp: u32);
    fn get_task_psp(&mut self) -> u32;
    fn select_next_task(&mut self);
    fn idle_task(&mut self);
}

// Symbol
pub trait Symbol {
    fn export(&mut self, sym_addr: u32, name: &str);
    fn unexport(&mut self, name: &str);
    fn search(&mut self, name: &str);
}

// Device
pub trait Device {
    fn register_block_device(&mut self);
    fn unregister_block_device(&mut self);
}

// Feature
pub trait Feature {
    fn register_module(&mut self);
    fn unregister_module(&mut self);
    fn get_module(&mut self, name: &str);
}

// FileSystem
pub trait FileSystem {
    fn mount_hard_drive(&mut self, disk: &str) -> bool;
    fn unmount_hard_drive(&mut self, disk: &str) -> bool;

    fn register_fs(&mut self, name: &str);
    fn unregister_fs(&mut self, name: &str);

    fn get_volume(&mut self, name: &str);
}

// WorkQueue
pub trait WorkQueue {
    fn create(&mut self);

    fn delete(&mut self);
    fn sched(&mut self);
}

// Event
pub trait Event {
    fn init_input_device(&mut self);
    fn exit_input_device(&mut self);
}

// Loader
pub trait Loader {
    fn load(&mut self);
    fn unload(&mut self);
}

// Process
pub trait Process {
    fn register_executor(&mut self);
    fn unregister_executor(&mut self);
}

// Timer
pub trait Timer {
    fn create(&mut self);
    fn modify(&mut self);
    fn delete(&mut self);
}

// Terminal
pub trait Terminal {
    fn register_cmd(&mut self);
    fn unregister_cmd(&mut self);
}

// Signal
pub trait Signal {
    fn raising(&mut self, signal: i32);
}

// Protocol
pub trait Protocol {
    fn register_stack(&mut self);
    fn unregister_stack(&mut self);
}

// Kernel
pub trait Kernel {
    fn setup(&mut self);
    fn start(&mut self);
    fn exit(&mut self);

    fn sleep(&mut self);
    fn standby(&mut self);
    fn shutdown(&mut self);
    fn reboot(&mut self);

    fn get_build_date(&mut self);
    fn get_build_time(&mut self);
    fn get_build_version(&mut self);
    fn get_build_git_sha(&mut self);

    fn system(&mut self) -> &mut dyn System;
    fn memory(&mut self) -> &mut dyn Memory;
    fn debug(&mut self) -> &mut dyn Debug;
    fn interrupt(&mut self) -> &mut dyn Interrupt;
    fn scheduler(&mut self) -> &mut dyn Scheduler;
    fn thread(&mut self) -> &mut dyn Thread;
    fn workqueue(&mut self) -> &mut dyn WorkQueue;
    fn event(&mut self) -> &mut dyn Event;
    fn symbol(&mut self) -> &mut dyn Symbol;
    fn device(&mut self) -> &mut dyn Device;
    fn feature(&mut self) -> &mut dyn Feature;
    fn filesys(&mut self) -> &mut dyn FileSystem;
    fn loader(&mut self) -> &mut dyn Loader;
    fn process(&mut self) -> &mut dyn Process;
    fn timer(&mut self) -> &mut dyn Timer;
    fn terminal(&mut self) -> &mut dyn Terminal;
    fn signal(&mut self) -> &mut dyn Signal;
    fn protocol(&mut self) -> &mut dyn Protocol;
}
