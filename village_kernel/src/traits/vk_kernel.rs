//###########################################################################
// vK_kernel.rs
// The interfaces of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::boxed::Box;
use super::vk_callback::Callback;
use super::vk_linkedlist::LinkedList;
use super::vk_module::Module;
use super::vk_driver::Driver;
use super::vk_driver::DriverID;
use super::vk_driver::DrvInfo;
use super::vk_driver::DriverOpts;
use super::vk_driver::PlatDriver;
use super::vk_driver::PlatDevice;

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

// Debug level
#[derive(PartialEq, PartialOrd, Debug)]
pub enum DebugLevel {
    Lv0 = 0,
    Lv1,
    Lv2,
    Lv3,
    Lv4,
    Lv5
}

// Debug
pub trait Debug {
    fn log(&mut self, log: &str);
    fn info(&mut self, log: &str);
    fn error(&mut self, error: &str);
    fn warn(&mut self, warn: &str);
    fn output(&mut self, level: DebugLevel, msg: &str);
    fn set_debug_level(&mut self, level: DebugLevel);
}

// Interrupt
pub trait Interrupt  {
    // Set ISR Methods
    fn set_isr_cb(&mut self, irq: isize, callback: Callback);

    // Add ISR Methods
    fn add_isr_cb(&mut self, irq: isize, callback: Callback);

    // Del ISR Methods
    fn del_isr_cb(&mut self, irq: isize, callback: Callback);
    
    // Clear ISR Methods
    fn clear_isr_cb(&mut self, irq: isize);
    
    // Replace Methods
    fn replace(&mut self, irq: isize, handler: usize);
    
    // Feature Methods
    fn handler(&mut self, irq: isize);
}

// Scheduler
pub trait Scheduler {
    fn start(&mut self);
    fn sched(&mut self);
}

// Thread state
#[derive(PartialEq)]
pub enum ThreadState {
    New = 0,
    Ready,
    Running,
    Blocked,
    Terminated,
}

// Thread task
pub struct ThreadTask {
    pub name: * const str,
    pub id:  i32,
    pub psp: u32,
    pub ticks: u32,
    pub stack: u32,
    pub state: ThreadState,
}

// ThreadTask
impl ThreadTask {
    // default
    pub fn default() -> Self {
        ThreadTask {
            name: "None",
            id:   -1,
            psp:   0,
            ticks: 0,
            stack: 0,
            state: ThreadState::New,
        }
    }
}

// Impl partial eq for thread task
impl PartialEq for ThreadTask {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Thread
pub trait Thread {
    // Create Methods
    fn create_task(&mut self, name: &str, callback: Callback) -> i32;
    
    // Task Methods
    fn start_task(&mut self, tid: i32);
    fn stop_task(&mut self, tid: i32);
    fn wait_for_task(&mut self, tid: i32);
    fn exit_blocked(&mut self, tid: i32);
    fn delete_task(&mut self, tid: i32);
    fn is_task_alive(&mut self, tid: i32) -> bool;
    fn get_tasks(&mut self) -> &LinkedList<ThreadTask>;

    // State Methods
    fn get_task_id(&mut self) -> i32;
    fn set_state(&mut self, state: ThreadState);
    fn sleep(&mut self, ticks: u32);
    fn blocked(&mut self);
    fn terminated(&mut self);

    // Scheduler Methods
    fn save_task_psp(&mut self, psp: u32);
    fn get_task_psp(&mut self) -> u32;
    fn select_next_task(&mut self);
    fn idle_task(&mut self);
}

// Symbol
pub trait Symbol {
    fn export(&mut self, sym_addr: u32, name: &str);
    fn unexport(&mut self, sym_addr: u32, name: &str);
    fn search(&mut self, name: &str) -> u32;
}

// Device
pub trait Device {
    // Register driver methods
    fn register_driver(&mut self, driver: Box<dyn Driver>);
    fn unregister_driver(&mut self, name: &str);

    // Platform driver methods
    fn register_plat_driver(&mut self, driver: Box<dyn PlatDriver>);
    fn unregister_plat_driver(&mut self, name: &str);

    // Platform device methods
    fn register_plat_device(&mut self, device: Box<dyn PlatDevice>);
    fn unregister_plat_device(&mut self, name: &str);

    // Data methods
    fn get_driver_fopts(&mut self, name: &str) -> Option<&mut dyn DriverOpts>;
    fn get_drivers(&mut self, id: DriverID) -> LinkedList<&mut DrvInfo>;
}

// Feature
pub trait Feature {
    // Register methods
    fn register_module(&mut self, module: Box<dyn Module>);
    fn unregister_module(&mut self, name: &str);

    // Data methods
    fn get_module(&mut self, name: &str) -> Option<&mut Box<dyn Module>>;
}

// FileSystem
pub trait FileSystem {
    fn mount_hard_drive(&mut self, disk: &str) -> bool;
    fn unmount_hard_drive(&mut self, disk: &str) -> bool;

    fn register_fs(&mut self, name: &str);
    fn unregister_fs(&mut self, name: &str);

    fn get_volume(&mut self, name: &str);
}

// Work state
#[derive(PartialEq)]
pub enum WorkState {
    New = 0,
    Ready,
    Running,
    Terminated,
}

// Struct work
pub struct Work {
    pub id: u32,
    pub ticks: u32,
    pub state: WorkState,
    pub callback: Callback,
}

// Impl work
impl Work {
    pub const fn new(id: u32, ticks: u32, callback: Callback) -> Self {
        Self {
            id,
            ticks,
            state: WorkState::New,
            callback,
        }
    }
}

// Impl partia eq for work
impl PartialEq for Work {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// WorkQueue
pub trait WorkQueue {
    fn create(&mut self, callback: Callback, ticks: u32) -> Option<&mut Work>;
    fn delete(&mut self, work: &mut Work) -> bool;
    fn sched(&mut self, work: &mut Work) -> bool;
}

// Event
pub trait Event {
    fn init_input_device(&mut self, input: &str);
    fn exit_input_device(&mut self, input: &str);
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

// Timer state
#[derive(PartialEq)]
pub enum JobState {
    New,
    Ready,
    Terminated,
}

// Struct timer job
pub struct Job {
    pub id: u32,
    pub ticks: u32,
    pub state: JobState,
    pub callback: Callback,
}

// Impl timer job
impl Job {
    // New
    pub const fn new(id: u32, callback: Callback) -> Self {
        Job {
            id,
            ticks: 0,
            state: JobState::New,
            callback,
        }
    }
}

// Impl partia eq for timer job
impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Timer
pub trait Timer {
    // Create Methods
    fn create(&mut self, callback: Callback) -> Option<&mut Job>;

    // Feature Methods
    fn modify(&mut self, job: &mut Job, ticks: u32);
    fn delete(&mut self, job: &mut Job);
}

// Terminal
pub trait Terminal {
    fn register_cmd(&mut self);
    fn unregister_cmd(&mut self);
}


// Signals
#[derive(PartialEq)]
pub enum Signals {
    None = 0,
    
    Sleep,
    Standby,
    Shutdown,
    Reboot,

    Kill,
}

// Signal
pub trait Signal {
    // Feature methods
    fn raising(&mut self, signal: Signals);
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
