//###########################################################################
// vk_console.rs
// The specific implementation of functions related to console
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use crate::village::kernel;
use super::vk_cmdmsg::CmdMsg;
use super::vk_cmdmsg::CmdMsgMgr;

// Console welcome string
const VK_WELCOME: &[&str] = &[
    "\r\n",
    r"        _ ____                    __                        __ ",
    r" _   __(_) / /___ _____ ____     / /_____  _________  ___  / / ",
    r"| | / / / / / __ `/ __ `/ _ \   / //_/ _ \/ ___/ __ \/ _ \/ /  ",
    r"| |/ / / / / /_/ / /_/ /  __/  / ,< /  __/ /  / / / /  __/ /   ",
    r"|___/_/_/_/\__,_/\__, /\___/  /_/|_|\___/_/  /_/ /_/\___/_/    ",
    r"                /____/                                         ",
    "\r\n",
];

// Struct console
pub struct Console {
    msg_mgr: CmdMsgMgr,
    user: String,
    mach: String,
    path: String,
}

// Impl console
impl Console {
    // New
    pub const fn new() -> Self {
        Self {
            msg_mgr: CmdMsgMgr::new(),
            user: String::new(),
            mach: String::new(),
            path: String::new(),
        }
    }

    // Setup
    pub fn setup(&mut self, driver: &str) {
        // Set default user
        self.user = "root".to_string();

        // Set default machine
        self.mach = "village".to_string();

        // Set default path
        self.path = "/".to_string();
        
        // Set msg mgr
        self.msg_mgr.setup(driver);

        // Output welcome message
        self.show_welcome_msg();

        // Output console symbol
        self.show_user_and_path();
    }

    // Exit
    pub fn exit(&mut self) {
        // Exit msg mgr
        self.msg_mgr.exit();
    }
}

// Impl console
impl Console {
    // Execute
    pub fn execute(&mut self) {
        loop {
            if self.msg_mgr.execute() {
                let msg = self.msg_mgr.read();
                self.execute_cmd(msg);
            }
        }
    }

    // Execute cmd
    fn execute_cmd(&mut self, msg: CmdMsg) {
        self.msg_mgr.write("\r\n");

        if let Some(cmd) = kernel().terminal().get_cmd(&msg.cmd) {
            cmd.setup(self);
            cmd.execute(msg.args.split(' ').collect());
            cmd.exit();
            self.show_user_and_path();
            return;
        }

        self.msg_mgr.write(&format!("{}: command not found\r\n", msg.cmd));
        self.show_user_and_path();
    }

    // Show welcome msg
    fn show_welcome_msg(&mut self) {
        // Disable irq
        kernel().system().disable_irq();

        // Output welcome message
        for line in VK_WELCOME.iter() {
            self.msg_mgr.write(&format!("{}\r\n",line));
        }

        // Enable irq
        kernel().system().enable_irq();
    }

    // Show user and path
    fn show_user_and_path(&mut self) {
        self.msg_mgr.write(&format!("{}@{} {} # ", self.user, self.mach, self.path));
    }
}

// Impl console
impl Console {
    // Set path
    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }

    // Get path
    pub fn get_path(&mut self) -> &str {
        &self.path
    }

    // Absolute path
    pub fn absolute_path(&mut self, path: &str) -> String {
        let mut abs_path = String::new();

        if !path.starts_with('/') {
            abs_path.push_str(&self.path);
            
            if !abs_path.ends_with('/') && !abs_path.is_empty() {
                abs_path.push('/');
            }
        }

        abs_path.push_str(path);

        abs_path
    }
}

// Impl Console
impl Console {
    // Log
    pub fn log(&mut self, log: &str) {
        self.msg_mgr.write(&format!("Log: {} \r\n", log));
    }

    // Info
    pub fn info(&mut self, info: &str) {
        self.msg_mgr.write(&format!("\x1b[36m[Info] {} \r\n\x1b[39m", info));
    }

    // Error
    pub fn error(&mut self, error: &str) {
        self.msg_mgr.write(&format!("\x1b[31m[Error] {} \r\n\x1b[39m", error));
    }

    // Warn
    pub fn warn(&mut self, warn: &str) {
        self.msg_mgr.write(&format!("\x1b[33m[Warning] {} \r\n\x1b[39m", warn));
    }

    // print
    pub fn print(&mut self, msg: &str) {
        self.msg_mgr.write(&format!("{}", msg));
    }

    // println
    pub fn println(&mut self, msg: &str) {
        self.msg_mgr.write(&format!("{}\r\n", msg));
    }
}
