//###########################################################################
// lib.rs
// The specific implementation of functions related to lib
//
// $Copyright: Copyright (C) village
//###########################################################################
#![no_std]
#![no_main]
#![feature(linkage)]

// import alloc module
extern crate alloc;

// import village module
pub mod village;

// import arch modules
pub mod arch {
    pub mod ia32 {
        pub mod legacy {
            pub mod vk_exception;
            pub mod vk_registers;
            pub mod vk_scheduler;
            pub mod vk_system;
        }
    }
}

// import binutils modules
pub mod binutils {
    pub mod executor {
        pub mod vk_bin_executor;
        pub mod vk_elf_executor;
        pub mod vk_hex_executor;
        pub mod vk_prog_executor;
    }
    pub mod loader {
        pub mod vk_bin_loader;
        pub mod vk_elf_defines;
        pub mod vk_elf_loader;
        pub mod vk_hex_loader;
        pub mod vk_lib_loader;
        pub mod vk_mod_loader;
        pub mod vk_prog_decode;
    }
    pub mod tool {
        pub mod vk_library_tool;
        pub mod vk_module_tool;
    }
}

// import board modules
pub mod board {
    pub mod vk_ia32legacy_board;
}

// import drivers modules
pub mod drivers {
    pub mod platdrv {
        pub mod block {
            pub mod vk_ata_lba_disk;
        }
        pub mod serial {
            pub mod vk_pic32_uart;
        }
    }
}

// import filesys modules
pub mod filesys {
    pub mod fs {
        pub mod fat {
            pub mod vk_fat_diskio;
            pub mod vk_fat_entry;
            pub mod vk_fat_filedir;
            pub mod vk_fat_folder;
            pub mod vk_fat_object;
            pub mod vk_fat_system;
        }
    }
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

// import protocol modules
pub mod protocol {
    pub mod vk_protocol;
}

// import terminal modules
pub mod terminal {
    pub mod cmds {
        pub mod vk_cmd_about;
        pub mod vk_cmd_cat;
        pub mod vk_cmd_debug;
        pub mod vk_cmd_device;
        pub mod vk_cmd_echo;
        pub mod vk_cmd_filesys;
        pub mod vk_cmd_help;
        pub mod vk_cmd_kill;
        pub mod vk_cmd_memory;
        pub mod vk_cmd_mod;
        pub mod vk_cmd_null;
        pub mod vk_cmd_power;
        pub mod vk_cmd_process;
        pub mod vk_cmd_run;
        pub mod vk_cmd_tasker;
    }
    pub mod vk_cmdmsg;
    pub mod vk_console;
    pub mod vk_terminal;
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
    pub mod vk_module;
}

// import vendor modules
pub mod vendor {
    pub mod ia32legacy {
        pub mod core {
            pub mod i686;
        }

        pub mod crt0 {
            pub mod crt0_kernel;
        }
    }
}

// import vklibs modules
pub mod vklibs {
    pub mod libc {
        pub mod stdlib;
        pub mod string;
    }
}
