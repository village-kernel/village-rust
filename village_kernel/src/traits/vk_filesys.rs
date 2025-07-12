//###########################################################################
// vK_filesys.rs
// The interfaces of functions related to filesys
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use alloc::string::String;

// struct FileMode
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FileMode(u32);

// Impl FileMode
impl FileMode {
    pub const OPEN_EXISTING: Self = FileMode(0x00);
    pub const READ: Self = FileMode(0x01);
    pub const WRITE: Self = FileMode(0x02);
    pub const READ_WRITE: Self = FileMode(0x03);
    pub const CREATE_NEW: Self = FileMode(0x04);
    pub const CREATE_ALWAYS: Self = FileMode(0x10);
    pub const OPEN_APPEND: Self = FileMode(0x30);

    // Contains
    pub fn contains(self, flag: Self) -> bool {
        (self.0 & flag.0) != 0
    }

    // Insert
    pub fn insert(&mut self, flag: Self) {
        self.0 |= flag.0
    }
}

// Enum FileType
#[derive(PartialEq, Clone)]
pub enum FileType {
    Unknown = 0x00,
    File = 0x01,
    Directory = 0x02,
    Volume = 0x04,
}

// Enum FileAttr
#[derive(PartialEq, Clone)]
pub enum FileAttr {
    None = 0x00,
    Visible = 0x04,
    Hidden = 0x08,
    System = 0x10,
}

// Struct FileDir
#[derive(Clone)]
pub struct FileDir {
    pub path: String,
    pub name: String,
    pub attr: FileAttr,
    pub typid: FileType,
}

// Impl FileDir
impl FileDir {
    pub const fn new() -> Self {
        Self {
            path: String::new(),
            name: String::new(),
            attr: FileAttr::Hidden,
            typid: FileType::Unknown,
        }
    }
}

// Trait FileVol
pub trait FileVol {
    // Methods
    fn setup(&mut self, disk: &str, starting_lba: u32) -> bool;
    fn exit(&mut self);

    // Volume methods
    fn set_mount_path(&mut self, path: &str);
    fn get_mount_path(&mut self) -> &str;
    fn set_name(&mut self, name: &str) -> bool;
    fn get_name(&mut self) -> String;

    // File methods
    fn open(&mut self, name: &str, mode: FileMode) -> usize;
    fn write(&mut self, fd: usize, data: &[u8], size: usize, offset: usize) -> usize;
    fn read(&mut self, fd: usize, data: &mut [u8], size: usize, offset: usize) -> usize;
    fn size(&mut self, fd: usize) -> usize;
    fn flush(&mut self, fd: usize);
    fn close(&mut self, fd: usize);

    // Dir methods
    fn opendir(&mut self, name: &str, mode: FileMode) -> usize;
    fn readdir(&mut self, fd: usize, dirs: &mut [FileDir], size: usize, offset: usize) -> usize;
    fn sizedir(&mut self, fd: usize) -> usize;
    fn closedir(&mut self, fd: usize);

    // Opt methods
    fn exist(&mut self, name: &str, typeid: FileType) -> bool;
    fn remove(&mut self, name: &str) -> bool;
}

// FileSys
pub trait FileSys {
    fn file_system_id(&self) -> usize;
    fn create_volume(&mut self) -> Box<dyn FileVol>;
}

// Struct FileSysWrapper
pub struct FileSysWrapper {
    name: &'static str,
    inner: Box<dyn FileSys>,
}

// Impl FileSysWrapper
impl FileSysWrapper {
    // New with name
    #[inline]
    pub fn new(inner: Box<dyn FileSys>, name: &'static str) -> Self {
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

    // Get system id
    #[inline]
    pub fn get_system_id(&self) -> usize {
        self.inner.file_system_id()
    }

    // Create volume
    #[inline]
    pub fn create_volume(&mut self) -> Box<dyn FileVol> {
        self.inner.create_volume()
    }
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
