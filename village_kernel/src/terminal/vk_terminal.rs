//###########################################################################
// vk_terminal.rs
// The specific implementation of functions related to terminal
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use crate::village::kernel;
use crate::traits::vk_kernel::Terminal;
use crate::traits::vk_command::Cmd;
use crate::traits::vk_callback::Callback;
use crate::traits::vk_linkedlist::LinkedList;
use crate::terminal::vk_console::Console;

// Struct sandbox
pub struct Sandbox {
    pub cid: i32,
    pub tid: i32,
    pub driver: String,
    pub console: Option<Box<Console>>,
}

// Impl sandbox
impl Sandbox {
    // New
    pub const fn new() -> Self {
        Self {
            cid: -1,
            tid: -1,
            driver: String::new(),
            console: None,
        }
    }
}

// Struct concrete terminal
pub struct ConcreteTerminal {
    cid_cnt: i32,
    commands: LinkedList<Box<dyn Cmd>>,
    sandboxes: LinkedList<Box<Sandbox>>,
}

// Impl concrete terminal
impl ConcreteTerminal {
    // New
    pub const fn new() -> Self {
        Self {
            cid_cnt: 0,
            commands: LinkedList::new(),
            sandboxes: LinkedList::new(),
        }
    }
}

// Impl concrete terminal
impl ConcreteTerminal {
    // Setup
    pub fn setup(&mut self) {
        // Create terminal execute
        let execute_cb = Callback::new(Self::execute as u32).with_instance(self);
        kernel().thread().create_task("Terminal::execute", execute_cb);

        // Output debug info
        kernel().debug().info("Terminal setup completed!");
    }

    // Execute
    fn execute(&mut self) {
        // Create the default console
        self.create_console("serial0");
    }

    // Exit
    pub fn exit(&mut self) {
        // Clear cmds
        self.commands.clear();

        // Clear sandboxs
        self.sandboxes.clear();
    }
}

// Impl terminal for concrete terminal
impl Terminal for ConcreteTerminal {
    // Register cmd
    fn register_cmd(&mut self, cmd: Box<dyn Cmd>) {
        self.commands.add(cmd);
    }

    // Unregister cmd
    fn unregister_cmd(&mut self, name: &str) {
        self.commands.retain_mut(|cmd|
            !(cmd.base().get_name() == name)
        );
    }

    // Get cmd
    fn get_cmd(&mut self, name: &str) -> Option<&mut Box<dyn Cmd>> {
        for cmd in self.commands.iter_mut() {
            if cmd.base().get_name() == name {
                return Some(cmd);
            }
        }
        None
    }

    // Get cmds
    fn get_cmds(&mut self) -> &mut LinkedList<Box<dyn Cmd>> {
        &mut self.commands
    }

    // Create console
    fn create_console(&mut self, driver: &str) -> i32 {
        // Create sandbox object
        let mut sandbox = Box::new(Sandbox::new());

        // Set the sandbox driver
        sandbox.driver = driver.to_string();

        // Create console object
        sandbox.console = Some(Box::new(Console::new()));

        // Set sandbox cid
        sandbox.cid = self.cid_cnt;
        self.cid_cnt += 1;

        // Create thread task
        let sandbox_na = format!("Console::{}", driver);
        let sandbox_cb = Callback::new(Self::console_sandbox as u32)
            .with_instance(self)
            .with_userdata(&mut sandbox.cid);
        let tid = kernel().thread().create_task(&sandbox_na, sandbox_cb);
        
        // Set sandbox tid
        sandbox.tid = tid;

        // Add to sandboxes list
        self.sandboxes.add(sandbox);

        // Start console task
        kernel().thread().start_task(tid);

        // return cid
        self.cid_cnt - 1
    }

    // Destroy console
    fn destroy_console(&mut self, driver: &str) {
        self.sandboxes.retain_mut(|sandbox| {
            if sandbox.driver == driver {
                kernel().thread().stop_task(sandbox.tid);
                false
            } else {
                true
            }
        });
    }
}

impl ConcreteTerminal {
    // Get sandbox
    fn get_sandbox(&mut self, cid: i32) -> Option<&mut Box<Sandbox>> {
        for sandbox in self.sandboxes.iter_mut() {
            if sandbox.cid == cid {
                return Some(sandbox);
            }
        }
        None
    }

    // Console sandbox
    fn console_sandbox(&mut self, userdata: *mut()) {
        // get cid form userdata
        let cid = unsafe { *(userdata as *const i32) };

        // Get sandbox
        if let Some(sandbox) = self.get_sandbox(cid) {
            // Running console
            if let Some(console) = &mut sandbox.console {
                console.setup(&sandbox.driver);
                console.execute();
                console.exit();
            }
        }

        // Delete console
        self.sandboxes.retain_mut(|sandbox| {
            !(sandbox.cid == cid)
        });
    }
}
