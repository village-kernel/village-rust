//###########################################################################
// vK_filesys.rs
// The interfaces of functions related to filesys
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::string::{String, ToString};

// Enum FileMode
pub enum FileMode
{
    OpenExisting  = 0x00,
    Read          = 0x01,
    Write         = 0x02,
    ReadWrite     = 0x03,
    CreateNew     = 0x04,
    CreateAlways  = 0x10,
    OpenAppend    = 0x30,
}

// Enum FileType
pub enum FileType
{
    Unknown  = 0x00,
    File     = 0x01,
    Diretory = 0x02,
    Volume   = 0x04,
}

// Enum FileAttr
pub enum FileAttr
{
    Visible = 0x04,
    Hidden  = 0x08,
    System  = 0x10,
}

// Struct FileDir
pub struct FileDir {
    pub path: String,
    pub name: String,
    pub attr: FileAttr,
     pub typeid: FileType,
}

// Trait FileVol
pub trait FileVol {
    // Methods
    fn setup(&mut self, disk: &str, starting_lba: u32) -> bool;
    fn exit(&mut self);

    // Volume methods
    fn set_mount_path(&mut self, path: &str);
    fn get_mount_path(&mut self) -> &str;
    fn set_name(&mut self, name: &str);
    fn get_name(&mut self) -> &str;

    // File methods
    fn open(&mut self, name: &str) -> usize;
    fn write(&mut self, fd: usize, data: &[u8], size: usize, offset: usize) -> usize;
    fn read(&mut self, fd: usize, data: &mut [u8], size: usize, offset: usize) -> usize;
    fn size(&mut self, fd: usize);
    fn flush(&mut self, fd: usize);
    fn close(&mut self, fd: usize);

    // Dir methods
    fn opendir(&mut self, name: &str);
    fn readdir(&mut self, fd: usize, dirs: &mut [FileDir], size: usize, offset: usize);
    fn sizedir(&mut self, fd: usize);
    fn closedir(&mut self, fd: usize);

    // Opt methods
    fn get_file_type(&mut self, name: &str) -> FileType;
    fn is_exist(&mut self, name: &str, typeid: FileType) -> bool;
    fn remove(&mut self, name: &str);
}

// Struct filesys info
pub struct FileSysInfo {
    name: String,
}

// Impl filesys data
impl FileSysInfo {
    // New
    pub const fn new() -> Self {
        Self {
            name: String::new(),
        }
    }

    // Set name
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // Get name
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// FileSys
pub trait FileSys {
    fn info(&mut self) -> &mut FileSysInfo;
    fn get_system_id(&mut self) -> usize;
    fn create_volume(&mut self) -> Box<dyn FileVol>;
}

// Register filesys macro
#[macro_export]
macro_rules! register_filesys {
    ($filsys:expr, $name:ident) => {
        paste::paste! {
            #[used]
            #[link_section = ".init_array"]
            static [<INIT_ $name:upper>]: fn() = [<$name _init>];

            #[used]
            #[link_section = ".fini_array"]
            static [<EXIT_ $name:upper>]: fn() = [<$name _exit>];

            fn [<$name _init>]() {
                let mut filesys = Box::new($filsys);
                filesys.info().set_name(stringify!($name));
                kernel().filesys().register_fs(filesys);
            }

            fn [<$name _exit>]() {
                kernel().filesys().unregister_fs(stringify!($name));
            }
        }
    };
}
