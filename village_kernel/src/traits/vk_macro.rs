//###########################################################################
// vK_macro.rs
// The interfaces of functions related to builder
//
// $Copyright: Copyright (C) village
//###########################################################################

// Println macro
#[macro_export]
macro_rules! println {
    () => {
        crate::village::kernel().debug().println("");
    };
    
    ($fmt:expr) => {
        crate::village::kernel().debug().println($fmt);
    };
    
    ($fmt:expr, $($arg:tt)*) => {
        let formatted = alloc::format!($fmt, $($arg)*);
        crate::village::kernel().debug().println(&formatted);
    };
}

// Debug log macro
#[macro_export]
macro_rules! debug_log {
    () => {
        crate::village::kernel().debug().log("");
    };
    
    ($fmt:expr) => {
        crate::village::kernel().debug().log($fmt);
    };
    
    ($fmt:expr, $($arg:tt)*) => {
        let formatted = alloc::format!($fmt, $($arg)*);
        crate::village::kernel().debug().log(&formatted);
    };
}

// Debug info macro
#[macro_export]
macro_rules! debug_info {
    () => {
        crate::village::kernel().debug().info("");
    };
    
    ($fmt:expr) => {
        crate::village::kernel().debug().info($fmt);
    };
    
    ($fmt:expr, $($arg:tt)*) => {
        let formatted = alloc::format!($fmt, $($arg)*);
        crate::village::kernel().debug().info(&formatted);
    };
}

// Debug error macro
#[macro_export]
macro_rules! debug_error {
    () => {
        crate::village::kernel().debug().error("");
    };
    
    ($fmt:expr) => {
        crate::village::kernel().debug().error($fmt);
    };
    
    ($fmt:expr, $($arg:tt)*) => {
        let formatted = alloc::format!($fmt, $($arg)*);
        crate::village::kernel().debug().error(&formatted);
    };
}

// Debug warning macro
#[macro_export]
macro_rules! debug_warning {
    () => {
        crate::village::kernel().debug().warning("");
    };
    
    ($fmt:expr) => {
        crate::village::kernel().debug().warning($fmt);
    };
    
    ($fmt:expr, $($arg:tt)*) => {
        let formatted = alloc::format!($fmt, $($arg)*);
        crate::village::kernel().debug().warning(&formatted);
    };
}

// Debug output macro
#[macro_export]
macro_rules! debug_output {
    ($lv:expr) => {
        crate::village::kernel().debug().output($lv, "");
    };
    
    ($lv:expr, $fmt:expr) => {
        crate::village::kernel().debug().output($lv, $fmt);
    };
    
    ($lv:expr, $fmt:expr, $($arg:tt)*) => {
        let formatted = alloc::format!($fmt, $($arg)*);
        crate::village::kernel().debug().output($lv, &formatted);
    };
}

// Register lib builder macro
#[macro_export]
macro_rules! register_lib_builder {
    ($lib:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let builder = crate::traits::vk_builder::LibBuilderWrapper::new(
                    Box::new($lib), stringify!($name)
                );
                crate::village::kernel().director().register_lib_builder(builder);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().director().unregister_lib_builder(stringify!($name));
            }
        }
    };
}

// Register prog builder macro
#[macro_export]
macro_rules! register_prog_builder {
    ($prog:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let builder = crate::traits::vk_builder::ProgBuilderWrapper::new(
                    Box::new($prog), stringify!($name)
                );
                crate::village::kernel().director().register_prog_builder(builder);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().director().unregister_prog_builder(stringify!($name));
            }
        }
    };
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
                let command = crate::traits::vk_command::CmdWrapper::new(
                    Box::new($cmd), stringify!($name)
                );
                crate::village::kernel().terminal().register_cmd(command);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().terminal().unregister_cmd(stringify!($name));
            }
        }
    };
}

// Register driver macro
#[macro_export]
macro_rules! register_driver {
    ($drv:expr, $id:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[link_section = ".init_array"]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[link_section = ".fini_array"]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let driver = crate::traits::vk_driver::DriverWrapper::with(
                    Box::new($drv), $id, stringify!($name)
                );
                crate::village::kernel().device().register_driver(driver);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().device().unregister_driver(stringify!($name));
            }
        }
    };
}

// Register plat driver macro
#[macro_export]
macro_rules! register_plat_driver {
    ($drv:expr, $name:ident, $fn_name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $fn_name:upper>]: fn() = [<$fn_name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $fn_name:upper>]: fn() = [<$fn_name _exit>];

            fn [<$fn_name _init>]() {
                let driver = crate::traits::vk_driver::PlatDrvWrapper::new(
                    Box::new($drv), stringify!($name)
                );
                crate::village::kernel().device().register_plat_driver(driver);
            }

            fn [<$fn_name _exit>]() {
                crate::village::kernel().device().unregister_plat_driver(stringify!($name));
            }
        }
    };
}

// Register plat device macro
#[macro_export]
macro_rules! register_plat_device {
    ($drv:expr, $name:ident, $fn_name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $fn_name:upper>]: fn() = [<$fn_name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $fn_name:upper>]: fn() = [<$fn_name _exit>];

            fn [<$fn_name _init>]() {
                let device = crate::traits::vk_driver::PlatDevWrapper::new(
                    Box::new($drv), stringify!($name)
                );
                crate::village::kernel().device().register_plat_device(device);
            }

            fn [<$fn_name _exit>]() {
                crate::village::kernel().device().unregister_plat_device(stringify!($name));
            }
        }
    };
}

// Register module macro
#[macro_export]
macro_rules! register_extension {
    ($mod:expr, $id:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let module = crate::traits::vk_extension::ExtensionWrapper::new(
                    alloc::boxed::Box::new($mod), $id, stringify!($name)
                );
                crate::village::kernel().extender().register_extension(module);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().extender().unregister_extension(stringify!($name));
            }
        }
    };
}

// Register filesys macro
#[macro_export]
macro_rules! register_filesys {
    ($filsys:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[unsafe(link_section = ".init_array")]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[unsafe(link_section = ".fini_array")]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let filesys = crate::traits::vk_filesys::FileSysWrapper::new(
                    Box::new($filsys), stringify!($name)
                );
                crate::village::kernel().filesys().register_fs(filesys);
            }

            fn [<$name _exit>]() {
                crate::village::kernel().filesys().unregister_fs(stringify!($name));
            }
        }
    };
}
