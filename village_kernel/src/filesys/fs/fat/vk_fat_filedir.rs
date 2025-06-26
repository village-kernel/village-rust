//###########################################################################
// vk_fat_filedir.rs
// The specific implementation of functions related to fat file dir
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec::Vec;
use super::vk_fat_diskio::FatDiskio;
use super::vk_fat_object::FatObject;
use super::vk_fat_folder::FatFolder;
use crate::traits::vk_filesys::{FileDir, FileMode};

// Struct FatFile
pub struct FatFile {
    myself: FatObject,
    file_id: usize,
    file_mode: FileMode,
    file_size: u32,
    fst_clust: u32,
    sector_size: u32,
    clust_size: u32,
    buffer: Vec<u8>,
}

// Impl FatFile
impl FatFile {
    // New
    pub fn new() -> Self {
        Self {
            myself: FatObject::new(),
            file_id: 0,
            file_mode: FileMode::OPEN_EXISTING,
            file_size: 0,
            fst_clust: 0,
            sector_size: 0,
            clust_size: 0,
            buffer: Vec::new(),
        }
    }

    // Set id
    pub fn set_id(&mut self, id: usize) {
        self.file_id = id;
    }

    // Get id
    pub fn get_id(&self) -> usize {
        self.file_id
    }
}

// Impl FatFile
impl FatFile  {
    // Open
    pub fn open(&mut self, diskio: &mut FatDiskio, mut fatobj: FatObject, mode: FileMode) {
        let bytes_per_sec = diskio.get_info().bytes_per_sec;
        let sec_per_clust = diskio.get_info().sec_per_clust;

        self.file_mode = mode;
        self.file_size = fatobj.get_file_size();

        if self.file_size > 0 {
            self.fst_clust = fatobj.get_first_cluster();
            self.sector_size = (self.file_size + bytes_per_sec - 1) / bytes_per_sec;
            self.clust_size = (self.sector_size + sec_per_clust - 1) / sec_per_clust;
            
            let buf_len = (self.clust_size * sec_per_clust * bytes_per_sec) as usize;
            self.buffer.resize(buf_len, 0);
            
            self.clust_size = diskio.read_cluster(&mut self.buffer, self.fst_clust, self.clust_size);
        }

        self.myself = fatobj;
    }

    // Write
    pub fn write(&mut self, data: &[u8], size: usize, offset: usize) -> usize {
        let mut buff_offset = 0usize;

        if self.file_mode.contains(FileMode::OPEN_APPEND) {
            buff_offset = self.file_size as usize;
        } else if self.file_mode.contains(FileMode::WRITE) {
            buff_offset = 0;
        }

        let new_size = buff_offset + size;
        
        if new_size as usize > self.buffer.len() {
            self.buffer.resize(new_size as usize, 0);
        }

        let src = &data[offset as usize..(offset + size) as usize];
        self.buffer[buff_offset as usize..(buff_offset + size) as usize].copy_from_slice(src);

        for i in 0..size {
            self.buffer[buff_offset + i] = data[i + offset];
        }

        self.file_size = new_size as u32;
        self.file_mode = FileMode::OPEN_APPEND;

        size
    }

    // Read
    pub fn read(&mut self, data: &mut [u8], mut size: usize, offset: usize) -> usize {
        if self.file_size > 0 {

            if (self.file_size as usize) < size {
                size = self.file_size as usize;
            }

            for i in 0..size {
                data[i + offset] = self.buffer[i];
            }
            
            size
        } else {
            0
        }
    }

    // Size
    pub fn size(&mut self) -> usize {
        self.file_size as usize
    }

    // Flush
    pub fn flush(&mut self, diskio: &mut FatDiskio) {
        if self.clust_size == diskio.write_cluster(&self.buffer, self.fst_clust, self.clust_size) {
            self.myself.set_file_size(self.file_size);
            FatFolder::update(diskio, self.myself.clone());
        }
    }

    // Close
    pub fn close(&mut self) {
        self.buffer.clear();
    }
}

// Struct FatDir
pub struct FatDir  {
    myself: FatObject,
    dir_id: usize,
    dir_mode: FileMode,
    sub_size: usize,
    sub_objs: Vec<FatObject>,
}

// Impl FatDir
impl FatDir  {
    // New
    pub const fn new() -> Self {
        Self {
            myself: FatObject::new(),
            dir_id: 0,
            dir_mode: FileMode::OPEN_EXISTING,
            sub_size: 0,
            sub_objs: Vec::new(),
        }
    }
    
    // Set id
    pub fn set_id(&mut self, id: usize) {
        self.dir_id = id;
    }

    // Get id
    pub fn get_id(&self) -> usize {
        self.dir_id
    }
}

// Impl FatDir
impl FatDir {
    // Open
    pub fn open(&mut self, diskio: &mut FatDiskio, fatobj: FatObject, mode: FileMode) {
        let mut folder = FatFolder::new(diskio, fatobj.clone());
        
        self.myself = fatobj;
        self.dir_mode = mode;
        self.sub_size = folder.size();
        self.sub_objs.resize(self.sub_size, FatObject::new());
        
        if self.sub_size > 0 {
            folder.read(&mut self.sub_objs);
        }
        
        folder.close();
    }

    // Read
    pub fn read(&mut self, dirs: &mut [FileDir], size: usize, offset: usize) -> usize {
        let resize = if self.sub_size < size {
            self.sub_size
        } else {
            size
        };
        
        for i in 0..resize as usize {
            let obj = &mut self.sub_objs[offset + i];
            dirs[i].name = obj.get_object_name();
            dirs[i].attr = obj.get_object_attr();
            dirs[i].typid = obj.get_object_type();
        }
        
        resize
    }

    // Size
    pub fn size(&mut self) -> usize {
        self.sub_size
    }

    // Close
    pub fn close(&mut self) {
        self.sub_objs.clear();
    }
}
