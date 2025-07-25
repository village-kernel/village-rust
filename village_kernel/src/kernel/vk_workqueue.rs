//###########################################################################
// vk_workqueue.rs
// The specific implementation of functions related to workqueue
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_callback::Callback;
use crate::traits::vk_kernel::{Work, WorkQueue, WorkState};
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;

// Struct village work queue
pub struct VillageWorkQueue {
    works: LinkedList<Work>,
    id_cnt: i32,
}

// Impl village work queue
impl VillageWorkQueue {
    pub const fn new() -> Self {
        Self {
            works: LinkedList::new(),
            id_cnt: 0,
        }
    }
}

// Impl village work queue
impl VillageWorkQueue {
    // Setup
    pub fn setup(&mut self) {
        // Create work queue task
        let execute_cb = Callback::new(Self::execute as u32).with_instance(self);
        kernel().thread().create_task("WorkQueue::execute", execute_cb);

        //output debug info
        kernel().debug().info("Work queue setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        self.works.clear();
    }
}

// Impl village work queue
impl VillageWorkQueue {
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

// Impl work queue for village work queue
impl WorkQueue for VillageWorkQueue {
    // Create
    fn create(&mut self, callback: Callback, ticks: u32) -> i32 {
        let id = self.id_cnt;
        self.id_cnt += 1;
        let work = Work::new(id, ticks, callback);
        self.works.push(work);
        id
    }

    // Delete
    fn delete(&mut self, work_id: i32) {
        self.works.retain_mut(|w|
            !(w.id == work_id && w.state == WorkState::Terminated)
        );
    }

    // Sched
    fn sched(&mut self, work_id: i32) -> bool {
        if let Some(work) = self.works.iter_mut().find(|t| t.id == work_id) {
            work.state = WorkState::Ready;
            return true;
        }
        false
    }
}
