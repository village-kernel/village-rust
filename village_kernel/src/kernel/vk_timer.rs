//###########################################################################
// vk_timer.rs
// The specific implementation of functions related to timer
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_callback::Callback;
use crate::traits::vk_kernel::{Job, JobState, Timer};
use crate::traits::vk_linkedlist::LinkedList;
use crate::vendor::ia32legacy::core::i686::SYSTICK_IRQN;
use crate::village::kernel;

// Struct village timer
pub struct VillageTimer {
    jobs: LinkedList<Job>,
    id_cnt: u32,
}

// Impl village timer
impl VillageTimer {
    pub const fn new() -> Self {
        Self {
            jobs: LinkedList::new(),
            id_cnt: 0,
        }
    }
}

// Impl village timer
impl VillageTimer {
    // Setup
    pub fn setup(&mut self) {
        // Add the systick interrupt handler
        let exec_cb = Callback::new(Self::execute as u32).with_instance(self);
        kernel().interrupt().add_isr_cb(SYSTICK_IRQN, exec_cb);

        //output debug info
        kernel().debug().info("Timer setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear jobs
        self.jobs.clear();

        // Del the systick interrupt handler
        let exec_cb = Callback::new(Self::execute as u32).with_instance(self);
        kernel().interrupt().del_isr_cb(SYSTICK_IRQN, exec_cb);
    }
}

// Impl concreta timer
impl VillageTimer {
    // Execute
    fn execute(&mut self) {
        for job in self.jobs.iter_mut() {
            if job.state == JobState::Ready {
                if kernel().system().get_ticks() >= job.ticks {
                    job.callback.call();
                    job.state = JobState::Terminated;
                }
            }
        }
    }
}

// Impl timer for village timer
impl Timer for VillageTimer {
    // Create
    fn create(&mut self, callback: Callback) -> Option<&mut Job> {
        let id = self.id_cnt;
        self.id_cnt += 1;
        let job = Job::new(id, callback);
        self.jobs.push(job);
        self.jobs.iter_mut().rev().next()
    }

    // Modify
    fn modify(&mut self, job: &mut Job, ticks: u32) {
        if let Some(job) = self.jobs.iter_mut().find(|t| t.id == job.id) {
            job.ticks = ticks;
            job.state = JobState::Ready;
        }
    }

    // Delete
    fn delete(&mut self, job: &mut Job) {
        self.jobs
            .retain_mut(|j| !(j.id == job.id && j.callback == job.callback));
    }
}
