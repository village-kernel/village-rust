//###########################################################################
// vk_fat_system.rs
// The specific implementation of functions related to fat system
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_fat_diskio::FatDiskio;
use super::vk_fat_entry::FatEntryAttr;
use super::vk_fat_filedir::{FatDir, FatFile};
use super::vk_fat_folder::FatFolder;
use super::vk_fat_object::FatObject;
use crate::traits::vk_filesys::{FileDir, FileMode, FileSys, FileType, FileVol};
use crate::traits::vk_linkedlist::LinkedList;
use crate::register_filesys;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// Struct FatVolume
struct FatVolume {
    diskio: FatDiskio,
    mount_path: String,
    dir_cnt: usize,
    file_cnt: usize,
    dirs: LinkedList<FatDir>,
    files: LinkedList<FatFile>,
}

// Impl FatVolume
impl FatVolume {
    // New
    pub const fn new() -> Self {
        Self {
            diskio: FatDiskio::new(),
            mount_path: String::new(),
            dir_cnt: 1,
            file_cnt: 1,
            dirs: LinkedList::new(),
            files: LinkedList::new(),
        }
    }
}

// Impl FatVolume
impl FatVolume {
    // Assign file id
    fn assign_file_id(&mut self) -> usize {
        let id = self.file_cnt;
        self.file_cnt += 1;
        id
    }

    // Assign dir id
    fn assign_dir_id(&mut self) -> usize {
        let id = self.dir_cnt;
        self.dir_cnt += 1;
        id
    }

    // Get Base name
    fn base_name(&mut self, path: &str) -> String {
        match path.rfind('/') {
            Some(pos) => path[pos + 1..].to_string(),
            None => path.to_string(),
        }
    }

    // Search path
    fn search_path(&mut self, path: &str, reserve: usize) -> Option<FatObject> {
        let names: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let deep = names.len() - reserve;

        let mut someobj = Some(FatObject::root());

        for i in 0..deep {
            if let Some(fatobj) = someobj {
                someobj = FatFolder::search_obj(&mut self.diskio, fatobj, names[i]);
            }

            if someobj.is_none() {
                break;
            }
        }

        someobj
    }

    // Create path
    fn create_path(&mut self, path: &str, attr: u8) -> Option<FatObject> {
        if let Some(mut parent) = self.search_path(path, 1) {
            if parent.get_object_type() == FileType::Directory {
                let name = self.base_name(path);
                return FatFolder::create_obj(&mut self.diskio, parent, &name, attr);
            }
        }
        None
    }

    // Delete path
    fn delete_path(&mut self, path: &str) -> bool {
        if let Some(mut fatobj) = self.search_path(path, 0) {
            if fatobj.get_object_type() == FileType::File ||
               fatobj.get_object_type() == FileType::Directory
            {
                FatFolder::remove_obj(&mut self.diskio, fatobj);
                return true;
            }
        }
        false
    }
}

// Impl FatVolume
impl FileVol for FatVolume {
    // Setup
    fn setup(&mut self, disk: &str, starting_lba: u32) -> bool {
        self.diskio.setup(disk, starting_lba)
    }

    // Exit
    fn exit(&mut self) {
        self.diskio.exit();
    }

    // Set mount path
    fn set_mount_path(&mut self, path: &str) {
        self.mount_path = path.to_string();
    }

    // Get mount path
    fn get_mount_path(&mut self) -> &str {
        &self.mount_path
    }

    // Set name
    fn set_name(&mut self, name: &str) -> bool {
        FatFolder::set_vol_lab(&mut self.diskio, name);
        self.get_name() == name
    }

    // Get name
    fn get_name(&mut self) -> String {
        FatFolder::get_vol_lab(&mut self.diskio)
    }

    // Open
    fn open(&mut self, name: &str, mode: FileMode) -> usize {
        // Search or create path
        let someobj = match self.search_path(name, 0) {
            Some(obj) => Some(obj),
            None if mode.contains(FileMode::CREATE_NEW) => self.create_path(name, FatEntryAttr::FILE),
            None => None,
        };

        // Open file
        if let Some(fatobj) = someobj {
            let fd = self.assign_file_id();
            let mut file = FatFile::new();

            file.set_id(fd);
            file.open(&mut self.diskio, fatobj, mode);

            self.files.push(file);

            return fd;
        }
        0
    }

    // Write
    fn write(&mut self, fd: usize, data: &[u8], size: usize, offset: usize) -> usize {
        if let Some(file) = self.files.iter_mut().find(|f| f.get_id() == fd) {
            return file.write(data, size, offset);
        }
        0
    }

    // Read
    fn read(&mut self, fd: usize, data: &mut [u8], size: usize, offset: usize) -> usize {
        if let Some(file) = self.files.iter_mut().find(|f| f.get_id() == fd) {
            return file.read(data, size, offset);
        }
        0
    }

    // Size
    fn size(&mut self, fd: usize) -> usize {
        if let Some(file) = self.files.iter_mut().find(|f| f.get_id() == fd) {
            return file.size();
        }
        0
    }

    // Flush
    fn flush(&mut self, fd: usize) {
        if let Some(file) = self.files.iter_mut().find(|f| f.get_id() == fd) {
            file.flush(&mut self.diskio);
        }
    }

    // Close
    fn close(&mut self, fd: usize) {
        self.files.retain_mut(|file| {
            if file.get_id() == fd {
                file.close();
                false
            } else {
                true
            }
        });
    }

    // Open dir
    fn opendir(&mut self, name: &str, mode: FileMode) -> usize {
        // Search or create path
        let someobj = match self.search_path(name, 0) {
            Some(obj) => Some(obj),
            None if mode.contains(FileMode::CREATE_NEW) => {
                self.create_path(name, FatEntryAttr::DIRECTORY)
            }
            None => None,
        };

        // Open dir
        if let Some(fatobj) = someobj {
            let fd = self.assign_dir_id();

            let mut dir = FatDir::new();
            dir.set_id(fd);
            dir.open(&mut self.diskio, fatobj, mode);

            self.dirs.push(dir);

            return fd;
        }
        0
    }

    // Read dir
    fn readdir(&mut self, fd: usize, dirs: &mut [FileDir], size: usize, offset: usize) -> usize {
        if let Some(dir) = self.dirs.iter_mut().find(|d| d.get_id() == fd) {
            return dir.read(dirs, size, offset);
        }
        0
    }

    // Size dir
    fn sizedir(&mut self, fd: usize) -> usize {
        if let Some(dir) = self.dirs.iter_mut().find(|d| d.get_id() == fd) {
            return dir.size();
        }
        0
    }

    // Close dir
    fn closedir(&mut self, fd: usize) {
        self.dirs.retain_mut(|dir| {
            if dir.get_id() == fd {
                dir.close();
                false
            } else {
                true
            }
        });
    }

    // Is exist
    fn exist(&mut self, name: &str, typeid: FileType) -> bool {
        if let Some(mut fatobj) = self.search_path(name, 0) {
            return fatobj.get_object_type() == typeid;
        }
        false
    }

    // Remove
    fn remove(&mut self, name: &str) -> bool {
        self.delete_path(name)
    }
}

// Struct fat system
struct FatSystem;

// Impl filesys for fat system
impl FileSys for FatSystem {
    // Get file system id
    fn file_system_id(&self) -> usize {
        const SYSTEM_ID: usize = 11;
        SYSTEM_ID
    }

    // Create volume
    fn create_volume(&mut self) -> Box<dyn FileVol> {
        Box::new(FatVolume::new())
    }
}

// Register filesys
register_filesys!(FatSystem, fat);
