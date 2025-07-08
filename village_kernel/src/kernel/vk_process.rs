//###########################################################################
// vk_process.rs
// The specific implementation of functions related to process
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_callback::Callback;
use crate::traits::vk_executor::{BaseExecutor, ExecutorWrapper};
use crate::traits::vk_kernel::{Process, ProcessBehavior, ProcessData};
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;

// Struct village process
pub struct VillageProcess {
    pid_cnt: i32,
    processes: LinkedList<Box<ProcessData>>,
    executors: LinkedList<Box<ExecutorWrapper>>,
}

// Impl village process
impl VillageProcess {
    pub const fn new() -> Self {
        Self {
            pid_cnt: 0,
            processes: LinkedList::new(),
            executors: LinkedList::new(),
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
        kernel()
            .thread()
            .create_task("Process::monitor", monitor_cb);

        // Output debug info
        kernel().debug().info("Process setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear datum
        self.processes.clear();

        // Clear factories
        self.executors.clear();
    }
}

// Impl village process
impl VillageProcess {
    // Taichi
    fn taichi(&mut self) {
        let taichi = "/services/taichi.exec";

        if self.run_with_args(ProcessBehavior::Background, taichi) < 0 {
            kernel()
                .debug()
                .error(&format!("{} execute failed!", taichi));
        }
    }

    // Monitor
    fn monitor(&mut self) {
        loop {
            self.processes
                .retain_mut(|data| kernel().thread().is_task_alive(data.tid));
            kernel().thread().sleep(10);
        }
    }
}

// Impl village process
impl VillageProcess {
    // Create executor
    fn create_executor(&mut self, path: &str) -> Option<Box<dyn BaseExecutor>> {
        let suffix = match path.rfind('.') {
            Some(pos) => &path[pos..],
            None => return None,
        };

        for executor in self.executors.iter_mut() {
            let suffixes = executor.get_suffixes();

            for supported_suffix in suffixes {
                if suffix == supported_suffix {
                    return Some(executor.create());
                }
            }
        }

        kernel()
            .debug()
            .error(&format!("file type: \"*{}\" executor no found!", suffix));
        None
    }
}

// Impl process for village process
impl Process for VillageProcess {
    // Register executor
    fn register_executor(&mut self, executor: Box<ExecutorWrapper>) {
        self.executors.push(executor);
    }

    // Unregister executor
    fn unregister_executor(&mut self, name: &str) {
        self.executors
            .retain_mut(|executor| !(executor.get_name() == name));
    }

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
        let mut process = Box::new(ProcessData::new());

        // Set the path
        process.path = path.to_string();

        // Create executor
        process.exec = self.create_executor(path);
        if process.exec.is_none() {
            return -1;
        }

        // Run executor with argv
        process.tid = process.exec.as_mut().unwrap().run(path, argv);
        if process.tid < 0 {
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
            if let Some(process) = self
                .processes
                .iter_mut()
                .find(|p| p.pid == (self.pid_cnt - 1))
            {
                if let Some(executor) = &mut process.exec {
                    executor.wait();
                }
            }
        }

        pid
    }

    // Kill by path
    fn kill_by_path(&mut self, path: &str) {
        if let Some(data) = self.processes.iter_mut().find(|d| d.path == path) {
            if let Some(executor) = &mut data.exec {
                executor.kill();
            }
        }
    }

    // Kill by pid
    fn kill_by_pid(&mut self, pid: i32) {
        if let Some(data) = self.processes.iter_mut().find(|d| d.pid == pid) {
            if let Some(executor) = &mut data.exec {
                executor.kill();
            }
        }
    }

    // Is exist by path
    fn is_exist_by_path(&mut self, path: &str) -> bool {
        if let Some(_) = self.processes.iter_mut().find(|d| d.path == path) {
            return true;
        }
        false
    }

    // Is exist by pid
    fn is_exist_by_pid(&mut self, pid: i32) -> bool {
        if let Some(_) = self.processes.iter_mut().find(|d| d.pid == pid) {
            return true;
        }
        false
    }

    // Get processes
    fn get_processes(&mut self) -> &mut LinkedList<Box<ProcessData>> {
        &mut self.processes
    }
}
