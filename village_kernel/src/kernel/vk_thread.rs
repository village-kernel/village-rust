//###########################################################################
// vk_thread.rs
// The specific implementation of functions related to thread
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::ptr;
use core::arch::asm;
use alloc::string::ToString;
use crate::village::kernel;
use crate::traits::vk_kernel::{ThreadState, ThreadTask, Thread};
use crate::traits::vk_callback::{FnCallback, Callback};
use crate::traits::vk_linkedlist::LinkedList;
use crate::arch::ia32::legacy::vk_registers::TaskContext;

// Static constants
const TASK_STACK_SIZE: u32 = 8192;
const PSP_FRAME_SIZE: u32 = core::mem::size_of::<TaskContext>() as u32;

// ConcreteThread implementation
pub struct ConcreteThread {
    tasks: LinkedList<ThreadTask>,
    id_cnt: i32,
}

impl ConcreteThread {
    // New
    pub const fn new() -> Self {
        ConcreteThread {
            tasks: LinkedList::new(),
            id_cnt: 0,
        }
    }

    // Setup
    pub fn setup(&mut self) {
        // First task should be idle task and the tid is 0
        let idle_task_cb = Callback::new(Self::idle_task as u32).with_instance(self);
        self.create_task("Thread::idle", idle_task_cb);

        // Create a monitor thread alive task
        let monitor_cb = Callback::new(Self::monitor as u32).with_instance(self);
        self.create_task("Thread::monitor", monitor_cb);

        // Output debug info
        kernel().debug().info("Thread setup completed!");
    }

    // Start
    pub fn start(&mut self) {
        for task in &mut self.tasks.iter_mut() {
            task.state = ThreadState::Running;
        }
        self.tasks.begin();
    }

    // Exit
    pub fn exit(&mut self) {
        for task in &mut self.tasks.iter_mut() {
            if task.stack != 0 {
                kernel().memory().free(task.stack, 0);
            }
        }
        self.tasks.clear();
    }

    // Task function handler 
    fn task_handler(&mut self, callback: FnCallback, instance: *mut(), userdata: *mut()) {
        callback(instance, userdata);
        self.terminated();
        loop {}
    }

    // Monitor
    fn monitor(&mut self) {
        loop {
            self.tasks.retain_mut(|task| {
                if task.state == ThreadState::Terminated {
                    kernel().memory().free(task.stack, 0);
                    false
                } else {
                    true
                }
            });
            self.sleep(10);
        }
    }
}

// Impl thread for concrete thread
impl Thread for ConcreteThread {
    // Create task fn
    fn create_task(&mut self, name: &str, callback: Callback) -> i32 {
       // Create a new task and allocate stack space
        let stack = kernel().memory().stack_alloc(TASK_STACK_SIZE);
        let psp = stack - PSP_FRAME_SIZE;

        // Fill the stack content
        let context = TaskContext::new(
            Self::task_handler as u32,
            self as *mut _ as u32,
            callback.callback as u32,
            callback.instance as u32,
            callback.userdata as u32,
        );
        unsafe { ptr::write(psp as *mut TaskContext, context); }

        // Create an new task with unique TID
        let tid = self.id_cnt;
        self.id_cnt += 1;

        let task = ThreadTask {
            name: name.to_string(),
            id: tid,
            psp,
            ticks: 0,
            stack,
            state: ThreadState::New,
        };

        // Add task into tasks list
        self.tasks.add(task);
        tid
    }
    
    // Start task
    fn start_task(&mut self, tid: i32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == tid) {
            task.state = ThreadState::Ready;
        }
    }

    // Stop task
    fn stop_task(&mut self, tid: i32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == tid) {
            task.state = ThreadState::Terminated;
        }
    }

    // Thread wait for task
    fn wait_for_task(&mut self, tid: i32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == tid) {
            while task.state != ThreadState::Terminated {}
        }
    }

    // Exit task blocked state
    fn exit_blocked(&mut self, tid: i32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == tid) {
            if task.state == ThreadState::Blocked {
                task.state = ThreadState::Ready;
            }
        }
    }

    // Thread delete task
    fn delete_task(&mut self, tid: i32) {
        self.tasks.retain(|task| {
            if task.id == tid {
                kernel().memory().free(task.stack, 0);
                false
            } else {
                true
            }
        });
    }

    // Thread check task is alive
    fn is_task_alive(&mut self, tid: i32) -> bool {
        self.tasks.iter_mut()
            .find(|t| t.id == tid)
            .map(|t| t.state != ThreadState::Terminated)
            .unwrap_or(false)
    }

    // Get tasks
    fn get_tasks(&mut self) -> &mut LinkedList<ThreadTask> {
        &mut self.tasks
    }

    // Get current task id
    fn get_task_id(&mut self) -> i32 {
        if let Some(task) = self.tasks.item() {
            return task.id;
        }
        -1
    }

    // Set State
    fn set_state(&mut self, state: ThreadState) {
        if let Some(task) = self.tasks.item() {
            task.state = state;
            kernel().scheduler().sched();
        }
    }

    // Thread sleep
    fn sleep(&mut self, ticks: u32) {
        if let Some(task) = self.tasks.item() {
            if task.id > 0 {
                task.state = ThreadState::Ready;
                task.ticks = kernel().system().get_sysclk_counts() + ticks;
                kernel().scheduler().sched();
                while task.state == ThreadState::Ready {}
            }
        }
    }

    // Thread Blocked
    fn blocked(&mut self) {
        if let Some(task) = self.tasks.item() {
            if task.id > 0 {
                task.state = ThreadState::Blocked;
                kernel().scheduler().sched();
                while task.state == ThreadState::Blocked {}
            }
        }
    }

    // Thread Terminated
    fn terminated(&mut self) {
        if let Some(task) = self.tasks.item() {
            if task.id > 0 {
                task.state = ThreadState::Terminated;
                kernel().scheduler().sched();
            }
        }
    }

    // Save task PSP
    fn save_task_psp(&mut self, psp: u32) {
        if let Some(task) = self.tasks.item() {
            task.psp = psp;
        }
    }

    // Get current task psp
    fn get_task_psp(&mut self) -> u32 {
        if let Some(task) = self.tasks.item() {
            task.psp
        } else {
            0
        }
    }

    // Select next task, round-Robin scheduler
    fn select_next_task(&mut self) {
        loop {
            // Set next task as current task
            self.tasks.next(); if self.tasks.is_end() { self.tasks.begin(); }

            // Get current task
            if let Some(task) = self.tasks.item() {

                //Check current task state
                if task.state == ThreadState::Ready {
                    if kernel().system().get_sysclk_counts() >= task.ticks {
                        task.state = ThreadState::Running;
                        task.ticks = 0;
                    }
                }

                // If no ready task is found, switch to the idle task (assuming the ID is 0)
                if task.state == ThreadState::Running {
                    break;
                }
            }
        }
    }

    // Idle task
    fn idle_task(&mut self) {
        loop {
            unsafe { asm!("nop");}
        }
    }
}
