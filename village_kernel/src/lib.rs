//###########################################################################
// lib.rs
// The specific implementation of functions related to lib
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]
#![feature(linkage)]

// import arch modules
pub mod arch {
    pub mod ia32 {
        pub mod legacy {
            pub mod vk_system;
            pub mod vk_exception;
            pub mod vk_scheduler;
        }
    }
}

// import filesys modules
pub mod filesys {
    pub mod vk_filesystem;
}

// import kernel modules
pub mod kernel {
    pub mod vk_debug;
    pub mod vk_device;
    pub mod vk_event;
    pub mod vk_feature;
    pub mod vk_interrupt;
    pub mod vk_loader;
    pub mod vk_memory;
    pub mod vk_process;
    pub mod vk_signal;
    pub mod vk_symbol;
    pub mod vk_thread;
    pub mod vk_timer;
    pub mod vk_village;
    pub mod vk_workqueue;
}

// import misc modules
pub mod misc {
    pub mod lock {
        pub mod vk_spinlock;
    }
}

// import protocol modules
pub mod protocol {
    pub mod vk_protocol;
}

// import terminal modules
pub mod terminal {
    pub mod vk_terminal;
}

// import traits modules
pub mod traits {
    pub mod vk_callback;
    pub mod vk_kernel;
}

// import vendor modules
pub mod vendor {
    pub mod ia32legacy {
        pub mod crt0 {
            pub mod crt0_kernel;
        }
    }
}

// import vklibs modules
pub mod vklibs {
    pub mod libc{
        pub mod stdlib;
        pub mod string;
    }
}
