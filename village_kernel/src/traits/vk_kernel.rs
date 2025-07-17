//###########################################################################
// vK_kernel.rs
// The interfaces of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_callback::Callback;
use super::vk_command::CmdWrapper;
use super::vk_driver::{DriverWrapper, PlatDevWrapper, PlatDrvWrapper};
use super::vk_executor::{BaseRunner, ExecutorWrapper};
use super::vk_filesys::{FileSysWrapper, FileVol};
use super::vk_linkedlist::LinkedList;
use super::vk_module::ModuleWrapper;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// System
pub trait System {
    fn get_ticks(&mut self) -> u32;
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
    fn alloc(&mut self, size: u32) -> u32;
    fn dealloc(&mut self, address: u32, size: u32);

    // Info Methods
    fn get_size(&mut self) -> u32;
    fn get_used(&mut self) -> u32;
    fn get_curr_addr(&mut self) -> u32;
}

// Debug level
#[derive(PartialEq, PartialOrd)]
pub enum DebugLevel {
    Lv0 = 0,
    Lv1,
    Lv2,
    Lv3,
    Lv4,
    Lv5,
}

impl DebugLevel {
    // as_str
    pub fn as_str(&self) -> &'static str {
        match self {
            DebugLevel::Lv0 => "Lv0",
            DebugLevel::Lv1 => "Lv1",
            DebugLevel::Lv2 => "Lv2",
            DebugLevel::Lv3 => "Lv3",
            DebugLevel::Lv4 => "Lv4",
            DebugLevel::Lv5 => "Lv5",
        }
    }

    // from_str
    pub fn from_str(s: &str) -> Option<DebugLevel> {
        match s {
            "lv0" => Some(DebugLevel::Lv0),
            "lv1" => Some(DebugLevel::Lv1),
            "lv2" => Some(DebugLevel::Lv2),
            "lv3" => Some(DebugLevel::Lv3),
            "lv4" => Some(DebugLevel::Lv4),
            "lv5" => Some(DebugLevel::Lv5),
            _ => None,
        }
    }
}

// Debug
pub trait Debug {
    fn log(&mut self, log: &str);
    fn info(&mut self, info: &str);
    fn error(&mut self, error: &str);
    fn warning(&mut self, warning: &str);
    fn output(&mut self, level: DebugLevel, msg: &str);
    fn set_debug_level(&mut self, level: DebugLevel);
}

// Interrupt
pub trait Interrupt {
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

// Impl thread state
impl ThreadState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThreadState::New => "NEW",
            ThreadState::Ready => "READY",
            ThreadState::Running => "RUNNING",
            ThreadState::Blocked => "BLOCKED",
            ThreadState::Terminated => "TERMINATED",
        }
    }
}

// Thread task
pub struct ThreadTask {
    pub name: String,
    pub id: i32,
    pub psp: u32,
    pub ticks: u32,
    pub stack_start: u32,
    pub stack_ended: u32,
    pub state: ThreadState,
}

// ThreadTask
impl ThreadTask {
    // default
    pub fn default() -> Self {
        ThreadTask {
            name: "None".to_string(),
            id: -1,
            psp: 0,
            ticks: 0,
            stack_start: 0,
            stack_ended: 0,
            state: ThreadState::New,
        }
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
    fn get_tasks(&mut self) -> &mut LinkedList<ThreadTask>;

    // State Methods
    fn get_task_id(&mut self) -> i32;
    fn sleep(&mut self, ticks: u32);
    fn blocked(&mut self);
    fn terminated(&mut self);

    // Scheduler Methods
    fn save_task_psp(&mut self, psp: u32);
    fn get_task_psp(&mut self) -> u32;
    fn select_next_task(&mut self);
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
    fn register_driver(&mut self, driver: DriverWrapper);
    fn unregister_driver(&mut self, name: &str);

    // Platform driver methods
    fn register_plat_driver(&mut self, driver: PlatDrvWrapper);
    fn unregister_plat_driver(&mut self, name: &str);

    // Platform device methods
    fn register_plat_device(&mut self, device: PlatDevWrapper);
    fn unregister_plat_device(&mut self, name: &str);

    // Data methods
    fn get_driver(&mut self, name: &str) -> Option<&mut DriverWrapper>;
    fn get_drivers(&mut self) -> &mut LinkedList<DriverWrapper>;
}

// Feature
pub trait Feature {
    // Register methods
    fn register_module(&mut self, module: ModuleWrapper);
    fn unregister_module(&mut self, name: &str);

    // Data methods
    fn get_module(&mut self, name: &str) -> Option<&mut ModuleWrapper>;
}

// FileSystem
pub trait FileSystem {
    // Hard drive methods
    fn mount_hard_drive(&mut self, disk: &str) -> bool;
    fn unmount_hard_drive(&mut self, disk: &str) -> bool;

    // Register methods
    fn register_fs(&mut self, fs: FileSysWrapper);
    fn unregister_fs(&mut self, name: &str);

    // Volume methods
    fn get_volume(&mut self, name: &str) -> Option<&mut Box<dyn FileVol>>;
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

// WorkQueue
pub trait WorkQueue {
    // Create Methods
    fn create(&mut self, callback: Callback, ticks: u32) -> Option<&mut Work>;

    // Feature Methods
    fn delete(&mut self, work: &mut Work) -> bool;
    fn sched(&mut self, work: &mut Work) -> bool;
}

// Enum EventType
pub enum EventType {
    InputKey = 0,
    InputAxis,
    OutputText,
    OutputAxis,
    AllSizes,
}

// Enum EventOutFormat
#[derive(Clone)]
pub enum EventOutFormat {
    Noraml = 0,
    Terminal,
}

// Struct EventInputKey
pub struct EventInputKey {
    pub code: isize,
    pub status: isize,
}

// Impl EventInputKey
impl EventInputKey {
    pub const fn new() -> Self {
        Self { code: 0, status: 0 }
    }
}

// Struct EventInputAxis
pub struct EventInputAxis {
    pub axis_x: isize,
    pub axis_y: isize,
    pub axis_z: isize,
}

// Impl EventInputAxis
impl EventInputAxis {
    pub const fn new() -> Self {
        Self {
            axis_x: 0,
            axis_y: 0,
            axis_z: 0,
        }
    }
}

// Struct EventOutputText
pub struct EventOutputText {
    pub data: String,
}

// Impl EventOutputText
impl EventOutputText {
    pub const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

// Struct EventOutputAxis
pub struct EventOutputAxis {
    pub axis_x: isize,
    pub axis_y: isize,
    pub axis_z: isize,
}

// Impl EventOutputAxis
impl EventOutputAxis {
    pub const fn new() -> Self {
        Self {
            axis_x: 0,
            axis_y: 0,
            axis_z: 0,
        }
    }
}

// Event
pub trait Event {
    // Device Methods
    fn init_input_device(&mut self, input: &str);
    fn exit_input_device(&mut self, input: &str);

    // Attach Methods
    fn attach(&mut self, etype: EventType, callback: Callback);
    fn detach(&mut self, etype: EventType, callback: Callback);

    // Input Methods
    fn report_key(&mut self, code: isize, status: isize);
    fn report_axis(&mut self, axis_x: isize, axis_y: isize, axis_z: isize);

    // Output Methods
    fn push_char(&mut self, ch: char);
    fn push_str(&mut self, str: &str);
    fn push_axis(&mut self, axis_x: isize, axis_y: isize, axis_z: isize);
    fn set_out_format(&mut self, format: EventOutFormat);
    fn get_out_format(&mut self) -> EventOutFormat;
}

// Loader
pub trait Loader {
    // Library Methods
    fn install_lib(&mut self, name: &str) -> bool;
    fn uninstall_lib(&mut self, name: &str) -> bool;
    fn search_symbol(&mut self, symbol: &str) -> usize;

    // Module Methods
    fn install_mod(&mut self, name: &str) -> bool;
    fn uninstall_mod(&mut self, name: &str) -> bool;

    // Data Methods
    fn get_libraries(&mut self) -> Vec<&str>;
    fn get_modules(&mut self) -> Vec<&str>;
}

// Process behavior
#[derive(PartialEq)]
pub enum ProcessBehavior {
    Foreground = 0,
    Background,
}

// Process data
pub struct ProcessData {
    pub path: String,
    pub pid: i32,
    pub tid: i32,
    pub runner: Option<Box<dyn BaseRunner>>,
}

// Process data
impl ProcessData {
    // New
    pub fn new() -> Self {
        ProcessData {
            path: "None".to_string(),
            pid: -1,
            tid: -1,
            runner: None,
        }
    }
}

// Executer
pub trait Executer {
    // Register Methods
    fn register_executor(&mut self, executor: ExecutorWrapper);
    fn unregister_executor(&mut self, name: &str);

    // Data Methods
    fn create_runner(&mut self, path: &str) -> Option<Box<dyn BaseRunner>>;
}

// Process
pub trait Process {
    // Run Methods
    fn run_with_args(&mut self, behavior: ProcessBehavior, args: &str) -> i32;
    fn run_with_argv(&mut self, behavior: ProcessBehavior, path: &str, argv: Vec<&str>) -> i32;

    // Kill Methods
    fn kill_by_path(&mut self, path: &str);
    fn kill_by_pid(&mut self, pid: i32);

    // Check Methods
    fn is_exist_by_path(&mut self, path: &str) -> bool;
    fn is_exist_by_pid(&mut self, pid: i32) -> bool;

    // Data Methods
    fn get_processes(&mut self) -> &mut LinkedList<ProcessData>;
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
    // Cmd Methods
    fn register_cmd(&mut self, cmd: CmdWrapper);
    fn unregister_cmd(&mut self, name: &str);
    fn get_cmd(&mut self, name: &str) -> Option<&mut CmdWrapper>;
    fn get_cmds(&mut self) -> &mut LinkedList<CmdWrapper>;

    // Console Methods
    fn create_console(&mut self, driver: &str) -> i32;
    fn destroy_console(&mut self, driver: &str);
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

// Struct BuildInfo
pub struct BuildInfo {
    pub year: &'static str,
    pub date: &'static str,
    pub time: &'static str,
    pub version: &'static str,
    pub git_sha: &'static str,
}

// Kernel
pub trait Kernel {
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
    fn executer(&mut self) -> &mut dyn Executer;
    fn feature(&mut self) -> &mut dyn Feature;
    fn filesys(&mut self) -> &mut dyn FileSystem;
    fn loader(&mut self) -> &mut dyn Loader;
    fn process(&mut self) -> &mut dyn Process;
    fn timer(&mut self) -> &mut dyn Timer;
    fn terminal(&mut self) -> &mut dyn Terminal;
    fn signal(&mut self) -> &mut dyn Signal;
    fn protocol(&mut self) -> &mut dyn Protocol;
    fn build_info(&self) -> &BuildInfo;

    fn setup(&mut self);
    fn start(&mut self);
    fn exit(&mut self);
}
