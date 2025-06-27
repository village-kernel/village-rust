//###########################################################################
// vk_fat_folder.rs
// The specific implementation of functions related to fat folder
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use super::vk_fat_diskio::DiskIndex;
use super::vk_fat_diskio::FatDiskio;
use super::vk_fat_object::FatObject;
use super::vk_fat_object::FatEntry;
use super::vk_fat_object::EntryAttr;
use crate::traits::vk_filesys::FileType;
use crate::traits::vk_linkedlist::LinkedList;

// Const mebers
const DIR_ENTRY_SIZE: usize = 32;

// Struct FatFolder
pub struct FatFolder {
    myself: FatObject,
    buffer: Vec<u8>,
    entidx: DiskIndex,
    fatobjs: LinkedList<Box<FatObject>>,
}

// Impl FatFolder
impl FatFolder {
    // New
    pub const fn default() -> Self {
        Self {
            myself: FatObject::new(),
            buffer: Vec::new(),
            entidx: DiskIndex::new(),
            fatobjs: LinkedList::new(),
        }
    }

    // New
    pub fn new(diskio: &mut FatDiskio, fatobj: FatObject) -> Self {
        let mut folder = Self::default();
        folder.open(diskio, fatobj);
        folder
    }

    // Root
    pub fn root(diskio: &mut FatDiskio) -> Self {
        let rootobj = FatObject::root();
        Self::new(diskio, rootobj)
    }
}

// Impl FatFolder
impl FatFolder {
    // Read
    fn begin(&mut self, diskio: &mut FatDiskio) {
        // Get first cluster
        let fst_clust = self.myself.get_first_cluster();
        self.entidx = diskio.get_frist_index(fst_clust);
        diskio.read_sector(&mut self.buffer, self.entidx.sector, 1);
    }

    // Next 
    fn next(&mut self, diskio: &mut FatDiskio, wrt_mode: bool) -> bool {
        self.entidx.index += 1;

        if self.entidx.index >= diskio.get_info().entries_per_sec {
            if wrt_mode {
                diskio.write_sector(&self.buffer, self.entidx.sector, 1);
            }

            self.entidx = diskio.get_next_index(self.entidx.clone());

            if self.entidx.sector != 0 {
                diskio.read_sector(&mut self.buffer, self.entidx.sector, 1);
                self.entidx.index = 0;
            } else {
                return false;
            }
        }

        true
    }

    // Is end
    fn is_end(&self) -> bool {
        self.entidx.sector == 0
    }

    // Set item
    fn set_item(&mut self, entry: FatEntry) {
        let offset = self.entidx.index as usize * DIR_ENTRY_SIZE;
        self.buffer[offset..(offset + DIR_ENTRY_SIZE)].copy_from_slice(&entry.as_bytes());
    }

    // Get item
    fn get_item(&mut self) -> Option<FatEntry> {
        let offset = self.entidx.index as usize * DIR_ENTRY_SIZE;
        FatEntry::from_bytes(&self.buffer[offset..(offset + DIR_ENTRY_SIZE)])
    }
}

// Impl FatFolder
impl FatFolder {
    // Find space
    fn find_space(&mut self, diskio: &mut FatDiskio, req_size: usize) -> bool {
        let mut start_idx = DiskIndex::new();
        let mut free_cnt = 0;
        
        self.begin(diskio);

        while !self.is_end() {
            if self.get_item().is_none() {
                free_cnt += 1;

                if free_cnt == 1 {
                    start_idx = self.entidx.clone();
                }
                
                if free_cnt >= req_size {
                    self.entidx = start_idx;
                    return true;
                }
            } else {
                free_cnt = 0;
            }
            
            self.next(diskio, false);
        }
        
        false
    }

    // Write entries
    fn write_entries(&mut self, diskio: &mut FatDiskio, entries: &[FatEntry]) -> usize {
        let size = entries.len();

        diskio.read_sector(&mut self.buffer, self.entidx.sector, 1);

        for i in 0..size {
            self.set_item(entries[i]);

            if (i < size - 1) && !self.next(diskio, true) {
                return i;
            }
        }

        diskio.write_sector(&mut self.buffer, self.entidx.sector, 1);

        size
    }
}

// Impl FatFolder
impl FatFolder{
    // Open
    pub fn open(&mut self, diskio: &mut FatDiskio, selfobj: FatObject) {
        self.myself = selfobj;

        let bytes_per_sec = diskio.get_info().bytes_per_sec as usize;
        self.buffer = vec![0u8; bytes_per_sec];

        if self.myself.get_object_type() == FileType::Directory {
            let mut index = DiskIndex::new();
            let mut entries = Vec::new();

            self.begin(diskio);
            
            while !self.is_end() {

                if let Some(entry) = self.get_item() {
                    // Record entry index
                    if entries.len() == 0 {
                        index = self.entidx.clone();
                    }

                    // Add entry into entries
                    entries.push(entry);

                    // Create fatobj when entry is short entry
                    if let FatEntry::Short(_) = entry {
                        let mut fatobj = FatObject::from_entries(&mut entries);
                        fatobj.set_index(index.clone());
                        self.fatobjs.add(Box::new(fatobj));
                        entries.clear();
                    }
                }
                
                self.next(diskio, false);
            }
        }
    }

    // Write
    pub fn write(&mut self, diskio: &mut FatDiskio, sub_objs: &mut [FatObject]) -> bool {
        for obj in sub_objs {
            let entries = obj.get_all_entries();
            let size = entries.len();

            if !self.find_space(diskio, size) {
                return false;
            }

            obj.set_index(self.entidx.clone());
            if self.write_entries(diskio, &entries) != size {
                return false;
            }

            self.fatobjs.add(Box::new(obj.clone()));
        }
        true
    }

    // Read
    pub fn read(&mut self, sub_objs: &mut [FatObject]) -> usize {
        let read_size = self.fatobjs.len().min(sub_objs.len());

        for (i, fatobj) in self.fatobjs.iter_mut().enumerate() {
            sub_objs[i] = *fatobj.clone();
        }

        read_size
    }

    // Size
    pub fn size(&mut self) -> usize {
        self.fatobjs.len()
    }

    // Close
    pub fn close(&mut self) {
        self.fatobjs.clear();
    }
}

// Impl Drop for FatFolder
impl Drop for FatFolder {
    fn drop(&mut self) {
        self.close();
    }
}

// Impl FatFolder
impl FatFolder{
    // Set volume label
    pub fn set_volume_label(diskio: &mut FatDiskio, label: &str) {
        let mut folder = Self::root(diskio);
        for fatobj in folder.fatobjs.iter_mut() {
            if fatobj.get_object_type() == FileType::Volume {
                fatobj.set_name(label);
                Self::update(diskio, *fatobj.clone());
                break;
            }
        }
    }

    // Get volume label
    pub fn get_volume_label(diskio: &mut FatDiskio) -> String {
        let mut folder = Self::root(diskio);
        for fatobj in folder.fatobjs.iter_mut() {
            if fatobj.get_object_type() == FileType::Volume {
                return fatobj.get_name();
            }
        }
        "NONAME".to_string()
    }

    // Remove fatobj
    pub fn remove(diskio: &mut FatDiskio, mut fatobj: FatObject) {
        fatobj.set_object_free();
        let mut folder = Self::default();
        folder.buffer = vec![0u8; diskio.get_info().bytes_per_sec as usize];
        folder.entidx = fatobj.get_index();
        folder.write_entries(diskio, &fatobj.get_all_entries());
    }

    // Update fatobj
    pub fn update(diskio: &mut FatDiskio, mut fatobj: FatObject) {
        let mut folder = Self::default();
        folder.buffer = vec![0u8; diskio.get_info().bytes_per_sec as usize];
        folder.entidx = fatobj.get_index();
        folder.write_entries(diskio, &fatobj.get_all_entries());
    }

    // Search
    pub fn search(diskio: &mut FatDiskio, fatobj: FatObject, name: &str) -> Option<FatObject> {
        let mut folder = Self::new(diskio, fatobj);

        for fatobj in folder.fatobjs.iter_mut() {
            if fatobj.get_name() == name {
                return Some(*fatobj.clone());
            }
        }

        None
    }

    // Create
    pub fn create(diskio: &mut FatDiskio, mut parent: FatObject, name: &str, attr: EntryAttr) -> Option<FatObject> {
        let parent_clust = parent.get_first_cluster();
        let newobj_clust = diskio.alloc_cluster(1);
        
        let mut newobj = vec![FatObject::new(); 1];
        newobj[0].set_name(name);
        newobj[0].set_attribute(attr.as_u8());
        newobj[0].set_first_cluster(newobj_clust);

        if Self::new(diskio, parent).write(diskio, &mut newobj) {
            if attr.contains(EntryAttr::DIRECTORY) {
                let mut dotobjs = vec![FatObject::new(); 2];
                dotobjs[0] = FatObject::new_dot_dir(newobj_clust);
                dotobjs[1] = FatObject::new_dot_dot_dir(parent_clust);
                Self::new(diskio, newobj[0].clone()).write(diskio, &mut dotobjs);
            }
            return Some(newobj[0].clone())
        }

        None
    }
}
