#![no_std]

// import arch
pub mod arch {
    pub mod ia32 {
        pub mod legacy {
            pub mod vk_system;
            pub mod vk_exception;
            pub mod vk_scheduler;
        }
    }
}

// import filesys
pub mod filesys {
    pub mod impls {
        pub mod vk_filesystem;
    }
}

// import kernel 
pub mod kernel {
    pub mod traits {
        pub mod vk_kernel;
    }
    pub mod impls {
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
}

// import protocol
pub mod protocol {
    pub mod impls {
        pub mod vk_protocol;
    }
}

// import terminal
pub mod terminal {
    pub mod impls {
        pub mod vk_terminal;
    }
}

// import vendor
pub mod vendor {
    pub mod ia32legacy {
        pub mod crt0 {
            pub mod crt0_kernel;
        }
    }
}

// import vklibs
pub mod vklibs {
    pub mod libc{
        pub mod string;
    }
}
