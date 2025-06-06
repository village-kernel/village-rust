//###########################################################################
// vk_thread.rs
// The specific implementation of functions related to thread
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_kernel::Thread;

// struct concrete thread
pub struct ConcreteThread;

// impl concrete thread
impl ConcreteThread {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete thread
impl ConcreteThread {
    // setup
    pub fn setup(&mut self) {
        //output debug info
        kernel().debug().info("Thread setup done!");
    }

    // start
    pub fn start(&mut self) {

    }

    // exit
    pub fn exit(&mut self) {

    }
}

// impl thread for concrete thread
impl Thread for ConcreteThread {
    // create task
    fn create_task(&mut self) -> i32 {
        0
    }
    
    // get task id
    fn get_task_id(&mut self) -> i32 {
        0
    }

    // start task
    fn start_task(&mut self, tid: i32) -> bool {
        let _ = tid;
        false
    }

    // stop task
    fn stop_task(&mut self, tid: i32) -> bool {
        let _ = tid;
        false
    }

    // wait for task
    fn wait_for_task(&mut self, tid: i32) -> bool {
        let _ = tid;
        false
    }

    // exit blocked
    fn exit_blocked(&mut self, tid: i32) -> bool {
        let _ = tid;
        false
    }

    // delete task
    fn delete_task(&mut self, tid: i32) -> bool {
        let _ = tid;
        false
    }

    // is task alive
    fn is_task_alive(&mut self, tid: i32) -> bool {
        let _ = tid;
        false
    }

    // get tasks
    fn get_tasks(&mut self) {

    }

    // change state
    fn change_state(&mut self) {

    }

    // sleep
    fn sleep(&mut self) {

    }

    // blocked
    fn blocked(&mut self) {

    }
    
    // task exit
    fn task_exit(&mut self) {

    }

    // save task psp
    fn save_task_psp(&mut self, psp: u32) {
        let _ = psp;
    }

    // get task psp
    fn get_task_psp(&mut self) -> u32 {
        0
    }
    
    // select next task
    fn select_next_task(&mut self) {

    }

    // idle task
    fn idle_task(&mut self) {

    }
}
