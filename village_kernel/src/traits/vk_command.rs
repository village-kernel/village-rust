//###########################################################################
// vK_command.rs
// The interfaces of functions related to command
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::{string::{String, ToString}, vec::Vec};
use crate::terminal::vk_console::Console;

// Struct cmd base
pub struct CmdBase {
    name: String,
    console: Option<*mut Console>,
}

// Impl cmd base
impl CmdBase {
    // New
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            console: None,
        }
    }

    // Set name
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // Get name
    pub fn get_name(&mut self) -> &str {
        &self.name
    }

    // Set console
    pub fn set_console(&mut self, console: &mut Console) {
        self.console = Some(console as *mut _);
    }

    // Get console
    pub fn get_console(&mut self) -> Option<&mut Console> {
        unsafe { self.console.map(|ptr| &mut *ptr) }
    }
}

// Cmd
pub trait Cmd {
    // Base
    fn base(&mut self) -> &mut CmdBase;
    
    // Setup
    fn setup(&mut self, console: &mut Console) {
        self.base().set_console(console);
    }

    // Exit
    fn exit(&mut self) {}

    // Methods
    fn execute(&mut self, argv: Vec<&str>);
    fn help(&mut self);
}

// Register cmd macro
#[macro_export]
macro_rules! register_cmd {
    ($cmd:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let mut command = Box::new($cmd);
                command.base().set_name(stringify!($name));
                kernel().terminal().register_cmd(command);
            }

            fn [<$name _exit>]() {
                kernel().terminal().unregister_cmd(stringify!($name));
            }
        }
    };
}
