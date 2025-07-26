//###########################################################################
// vk_process.rs
// The specific implementation of functions related to process
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_callback::Callback;
use crate::traits::vk_kernel::{Process, ProcessBehavior, ProcessData};
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;
use crate::debug_error;
use crate::debug_info;
use alloc::string::ToString;
use alloc::vec::Vec;

// Struct village process
pub struct VillageProcess {
    pid_cnt: i32,
    processes: LinkedList<ProcessData>,
}

// Impl village process
impl VillageProcess {
    pub const fn new() -> Self {
        Self {
            pid_cnt: 0,
            processes: LinkedList::new(),
        }
    }
}

// Impl village process
impl VillageProcess {
    // Setup
    pub fn setup(&mut self) {
        // Create a running taichi application task
        let taichi_cb = Callback::new(Self::taichi as u32).with_instance(self);
        kernel().thread().create_task("Process::taichi", taichi_cb);

        // Create a monitor thread alive task
        let monitor_cb = Callback::new(Self::monitor as u32).with_instance(self);
        kernel().thread().create_task("Process::monitor", monitor_cb);

        // Output debug info
        debug_info!("Process setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear processes
        self.processes.clear();
    }
}

// Impl village process
impl VillageProcess {
    // Taichi
    fn taichi(&mut self) {
        let taichi = "/services/taichi/taichi.exec";

        if self.run_with_args(ProcessBehavior::Background, taichi) < 0 {
            debug_error!("{} execute failed!", taichi);
        }
    }

    // Monitor
    fn monitor(&mut self) {
        loop {
            self.processes.retain_mut(|data| 
                kernel().thread().is_task_alive(data.tid)
            );
            kernel().thread().sleep(10);
        }
    }
}

// Impl process for village process
impl Process for VillageProcess {
    // Run with args
    fn run_with_args(&mut self, behavior: ProcessBehavior, args: &str) -> i32 {
        // Split args
        let argv: Vec<&str> = args.split_whitespace().collect();

        // Run with argv
        self.run_with_argv(behavior, argv[0], argv)
    }

    // Run with argv
    fn run_with_argv(&mut self, behavior: ProcessBehavior, path: &str, argv: Vec<&str>) -> i32 {
        // New data object
        let mut process = ProcessData::new();

        // Set the path
        process.path = path.to_string();

        // Create runner
        process.container = kernel().director().create_prog_container(path);
        if process.container.is_none() {
            debug_error!("{} unsupported file type!", path);
            return -1;
        }

        // Run with argv
        process.tid = process.container.as_mut().unwrap().run(path, argv);
        if process.tid < 0 {
            debug_error!("{} create task failed!", path);
            return -1;
        }

        // Get process id
        let pid: i32 = self.pid_cnt;
        self.pid_cnt += 1;

        // Set process id
        process.pid = pid;
        
        // Add into list
        self.processes.push(process);

        // Wait for task done
        if behavior == ProcessBehavior::Foreground {
            if let Some(process) = self.processes
                .iter_mut().find(|p| p.pid == (self.pid_cnt - 1))
            {
                if let Some(runner) = &mut process.container {
                    runner.wait();
                }
            }
        }

        pid
    }

    // Kill by path
    fn kill_by_path(&mut self, path: &str) {
        if let Some(process) = self.processes
            .iter_mut().find(|p| p.path == path)
        {
            if let Some(runner) = &mut process.container {
                runner.kill();
            }
        }
    }

    // Kill by pid
    fn kill_by_pid(&mut self, pid: i32) {
        if let Some(process) = self.processes
            .iter_mut().find(|p| p.pid == pid)
        {
            if let Some(runner) = &mut process.container {
                runner.kill();
            }
        }
    }

    // Is exist by path
    fn is_exist_by_path(&mut self, path: &str) -> bool {
        if let Some(_) = self.processes.iter_mut().find(|p| p.path == path) {
            return true;
        }
        false
    }

    // Is exist by pid
    fn is_exist_by_pid(&mut self, pid: i32) -> bool {
        if let Some(_) = self.processes.iter_mut().find(|p| p.pid == pid) {
            return true;
        }
        false
    }

    // Get processes
    fn get_processes(&mut self) -> &mut LinkedList<ProcessData> {
        &mut self.processes
    }
}
