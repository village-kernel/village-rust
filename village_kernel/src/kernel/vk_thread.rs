//###########################################################################
// vk_thread.rs
// The specific implementation of functions related to thread
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::kernel;
use crate::traits::vk_kernel::Thread;

/// struct concrete thread
pub struct ConcreteThread;

/// impl concrete thread
impl ConcreteThread {
    /// setup
    pub fn setup(&self) {
        //output debug info
        kernel().debug().info("Thread setup done!");
    }

    /// start
    pub fn start(&self) {

    }

    /// exit
    pub fn exit(&self) {

    }
}

/// impl thread for concrete thread
impl Thread for ConcreteThread {
    /// create task
    fn create_task(&self) -> i32 {
        0
    }
    
    /// get task id
    fn get_task_id(&self) -> i32 {
        0
    }

    /// start task
    fn start_task(&self, tid: i32) -> bool {
        false
    }

    /// stop task
    fn stop_task(&self, tid: i32) -> bool {
        false
    }

    /// wait for task
    fn wait_for_task(&self, tid: i32) -> bool {
        false
    }

    /// exit blocked
    fn exit_blocked(&self, tid: i32) -> bool {
        false
    }

    /// delete task
    fn delete_task(&self, tid: i32) -> bool {
        false
    }

    /// is task alive
    fn is_task_alive(&self, tid: i32) -> bool {
        false
    }

    /// get tasks
    fn get_tasks(&self) {

    }

    /// change state
    fn change_state(&self) {

    }

    /// sleep
    fn sleep(&self) {

    }

    /// blocked
    fn blocked(&self) {

    }
    
    /// task exit
    fn task_exit(&self) {

    }

    /// save task psp
    fn save_task_psp(&self, psp: u32) {

    }

    /// get task psp
    fn get_task_psp(&self) -> u32 {
        0
    }
    
    /// select next task
    fn select_next_task(&self) {

    }

    /// idle task
    fn idle_task(&self) {

    }
}
