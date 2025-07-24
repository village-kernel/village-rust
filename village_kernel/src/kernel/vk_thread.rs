//###########################################################################
// vk_thread.rs
// The specific implementation of functions related to thread
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::arch::ia32::legacy::vk_registers::TaskContext;
use crate::traits::vk_callback::{Callback, FnCallback};
use crate::traits::vk_kernel::{Thread, ThreadState, ThreadTask};
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;
use alloc::string::ToString;
use core::ptr;

// Static constants
const TASK_STACK_SIZE: u32 = 8192;
const PSP_FRAME_SIZE: u32 = core::mem::size_of::<TaskContext>() as u32;

// Struct village thread
pub struct VillageThread {
    tasks: LinkedList<ThreadTask>,
    id_cnt: i32,
}

// Impl village thread
impl VillageThread {
    // New
    pub const fn new() -> Self {
        VillageThread {
            tasks: LinkedList::new(),
            id_cnt: 0,
        }
    }

    // Setup
    pub fn setup(&mut self) {
        // Create idle task
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
        // Set all task state to ready
        for task in &mut self.tasks.iter_mut() {
            task.state = ThreadState::Ready;
        }
    }

    // Exit
    pub fn exit(&mut self) {
        for task in &mut self.tasks.iter_mut() {
            if task.stack_start != 0 {
                kernel().memory().dealloc(task.stack_start, 0);
            }
        }
        self.tasks.clear();
    }

    // Task function handler
    fn task_handler(&mut self, callback: FnCallback, instance: *mut (), userdata: *mut ()) {
        callback(instance, userdata);
        self.terminated();
        loop {}
    }

    // Idle task
    fn idle_task(&mut self) {
        loop {
            if let Some(task) = self.tasks.item() {
                task.state = ThreadState::Ready;
                kernel().scheduler().sched();
                while task.state == ThreadState::Ready {}
            }
        }
    }

    // Monitor task
    fn monitor(&mut self) {
        loop {
            self.tasks.retain_mut(|task| {
                if task.state == ThreadState::Terminated {
                    kernel().memory().dealloc(task.stack_start, 0);
                    false
                } else {
                    true
                }
            });
            self.sleep(10);
        }
    }
}

// Impl thread for village thread
impl Thread for VillageThread {
    // Create task fn
    fn create_task(&mut self, name: &str, callback: Callback) -> i32 {
        // Create a new task and allocate stack space
        let stack_start = kernel().memory().alloc(TASK_STACK_SIZE);
        let stack_ended = stack_start + TASK_STACK_SIZE;
        let psp = stack_ended - PSP_FRAME_SIZE;

        // Fill the stack content
        let context = TaskContext::new(
            Self::task_handler as u32,
            self as *mut _ as u32,
            callback.callback as u32,
            callback.instance as u32,
            callback.userdata as u32,
        );
        unsafe {
            ptr::write(psp as *mut TaskContext, context);
        }

        // Create an new task with unique TID
        let tid = self.id_cnt;
        self.id_cnt += 1;

        let task = ThreadTask {
            name: name.to_string(),
            id: tid,
            psp,
            ticks: 0,
            stack_start,
            stack_ended,
            state: ThreadState::New,
        };

        // Add task into tasks list
        self.tasks.push(task);
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
                kernel().memory().dealloc(task.stack_start, 0);
                false
            } else {
                true
            }
        });
    }

    // Thread check task is alive
    fn is_task_alive(&mut self, tid: i32) -> bool {
        self.tasks
            .iter_mut()
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

    // Thread Sleep
    fn sleep(&mut self, ticks: u32) {
        if let Some(task) = self.tasks.item() {
            task.state = ThreadState::Blocked;
            task.ticks = kernel().system().get_ticks() + ticks;
            kernel().scheduler().sched();
            while task.state == ThreadState::Blocked {}
        }
    }

    // Thread Blocked
    fn blocked(&mut self) {
        if let Some(task) = self.tasks.item() {
            task.state = ThreadState::Blocked;
            task.ticks = 0;
            kernel().scheduler().sched();
            while task.state == ThreadState::Blocked {}
        }
    }

    // Thread Terminated
    fn terminated(&mut self) {
        if let Some(task) = self.tasks.item() {
            task.state = ThreadState::Terminated;
            kernel().scheduler().sched();
        }
    }

    // Save task PSP, the first item must be None
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

    // Select next task
    fn select_next_task(&mut self) {
        loop {
            // Get next task
            if let Some(task) = self.tasks.cycle() {
                // Task state is Blocked
                if task.state == ThreadState::Blocked {
                    if task.ticks != 0 && task.ticks <= kernel().system().get_ticks() {
                        task.ticks = 0;
                        task.state = ThreadState::Running;
                        break;
                    }
                }
                
                // Task state is ready
                else if task.state == ThreadState::Ready {
                    task.state = ThreadState::Running;
                    break;
                }

                // Task state is running
                else if task.state == ThreadState::Running {
                    break;
                }
            }
        }
    }
}
