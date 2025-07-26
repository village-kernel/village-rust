//###########################################################################
// vK_command.rs
// The interfaces of functions related to command
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;

// Trait Console
pub trait Console {
    // Path methods
    fn set_path(&mut self, path: &str);
    fn get_path(&mut self) -> &str;
    fn real_path(&mut self, path: &str) -> String;

    // Msg methods
    fn log(&mut self, log: &str);
    fn info(&mut self, info: &str);
    fn error(&mut self, error: &str);
    fn warn(&mut self, warn: &str);
    fn print(&mut self, msg: &str);
    fn println(&mut self, msg: &str);
}

// Trait Cmd
pub trait Cmd {
    fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>);
    fn help(&mut self, console: &mut dyn Console);
}

// Struct CmdWrapper
pub struct CmdWrapper {
    name: &'static str,
    inner: Box<dyn Cmd>,
}

// Impl CmdWrapper
impl CmdWrapper {
    // New with name
    #[inline]
    pub fn new(inner: Box<dyn Cmd>, name: &'static str) -> Self {
        Self {
            name,
            inner,
        }
    }

    // Get name
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    // Execute
    #[inline]
    pub fn exec(&mut self, console: &mut dyn Console, argv: Vec<&str>) {
        self.inner.exec(console, argv);
    }

    // Help
    #[inline]
    pub fn help(&mut self, console: &mut dyn Console) {
        self.inner.help(console);
    }
}
