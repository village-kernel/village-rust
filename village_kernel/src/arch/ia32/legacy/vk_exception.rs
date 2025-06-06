//###########################################################################
// vk_exception.rs
// The specific implementation of functions related to exception
//
// $Copyright: Copyright (C) village
//###########################################################################

// struct concrete exception
pub struct ConcreteException;

pub const ISR_NUM: usize = 256;
pub const RSVD_ISR_SIZE: usize = 0;

// impl concrete exception
impl ConcreteException {
    pub const fn new() -> Self {
        Self { }
    }
}

// impl concrete exception
impl ConcreteException {
    // setup
    pub fn setup(&mut self) {

    }

    // exit
    pub fn exit(&mut self) {

    }

    // install
    pub fn install(irq: u32, handler: u32) {
        let _ = irq;
        let _ = handler;
    }
}
