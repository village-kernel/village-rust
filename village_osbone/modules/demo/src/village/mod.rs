//###########################################################################
// village.rs
// The specific implementation of functions related to village mod
//
// $Copyright: Copyright (C) village
//###########################################################################
// import alloc module
extern crate alloc;

// import village module
pub use stdlib::village::kernel;

// import libstd modules
pub mod stdlib {
    pub mod crt0;
    pub mod panic;
    pub mod string;
    pub mod stdlib;
    pub mod village;
}

// import traits modules
pub mod traits {
    pub mod vk_callback;
    pub mod vk_command;
    pub mod vk_driver;
    pub mod vk_event_codes;
    pub mod vk_executor;
    pub mod vk_filesys;
    pub mod vk_kernel;
    pub mod vk_linkedlist;
    pub mod vk_extension;
}

// import misc modules
pub mod misc {
    pub mod fopts {
        pub mod vk_dev_fopt;
        pub mod vk_dir_fopt;
        pub mod vk_file_fopt;
        pub mod vk_filesys_fopt;
    }
    pub mod lock {
        pub mod vk_mutex;
        pub mod vk_spinlock;
    }
    pub mod model {
        pub mod vk_observer;
    }
    pub mod parser {
        pub mod vk_args_parser;
        pub mod vk_rc_parser;
    }
}
