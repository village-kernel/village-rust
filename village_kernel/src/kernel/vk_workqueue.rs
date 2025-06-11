//###########################################################################
// vk_workqueue.rs
// The specific implementation of functions related to workqueue
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::village::kernel;
use crate::traits::vk_callback::Callback;
use crate::traits::vk_linkedlist::LinkedList;
use crate::traits::vk_kernel::{WorkState, Work, WorkQueue};

// Struct concrete work queue
pub struct ConcreteWorkQueue {
    works: LinkedList<Work>,
    id_cnt: u32,
}

// Impl concrete work queue
impl ConcreteWorkQueue {
    pub const fn new() -> Self {
        Self { 
            works: LinkedList::new(),
            id_cnt: 0,
        }
    }
}

// Impl concrete work queue
impl ConcreteWorkQueue {
    // Setup
    pub fn setup(&mut self) {
        // Create work queue task
        let execute_cb = Callback::new(Self::execute as u32).with_instance(self);
        kernel().thread().create_task("WorkQueue::Execute", execute_cb);

        //output debug info
        kernel().debug().info("Work queue setup done!");
    }

    // Exit
    pub fn exit(&mut self) {
        self.works.clear();
    }
}

// Impl concrete work queue
impl ConcreteWorkQueue {
    // Execute
    fn execute(&mut self) {
        loop {
            for work in self.works.iter_mut() {
                if work.state == WorkState::Ready {
                    work.state = WorkState::Running;

                    if work.ticks > 0 {
                        kernel().thread().sleep(work.ticks);
                    }
                    
                    work.callback.call();
                    work.state = WorkState::Terminated;
                }
            }
            
            kernel().thread().sleep(1);
        }
    }
}

// Impl work queue for concrete work queue
impl WorkQueue for ConcreteWorkQueue {
    // Create
    fn create(&mut self, callback: Callback, ticks: u32) -> Option<&mut Work> {
        let id = self.id_cnt;
        self.id_cnt += 1;
        let work = Work::new(id, ticks, callback);
        self.works.push(work);
        self.works.end();
        self.works.item()
    }

    // Delete
    fn delete(&mut self, work: &mut Work) -> bool{
        if work.state == WorkState::Terminated {
            self.works.delete(work);
            return true;
        }
        false
    }

    // Sched
    fn sched(&mut self, work: &mut Work) -> bool {
        if let Some(work) = self.works.iter_mut().find(|t| t.id == work.id) {
            work.state = WorkState::Ready;
            return true;
        }
        false
    }
}
