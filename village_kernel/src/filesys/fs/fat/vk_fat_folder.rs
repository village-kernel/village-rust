//###########################################################################
// vk_fat_folder.rs
// The specific implementation of functions related to fat folder
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_fat_diskio::DiskIndex;
use super::vk_fat_diskio::FatDiskio;
use super::vk_fat_entry::FatEntry;
use super::vk_fat_entry::FatEntryAttr;
use super::vk_fat_entry::FatEntryIterator;
use super::vk_fat_object::FatObject;
use crate::traits::vk_filesys::FileType;
use crate::traits::vk_linkedlist::LinkedList;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// Struct FatFolder
pub struct FatFolder {
    myself: FatObject,
    fatobjs: LinkedList<FatObject>,
}

// Impl FatFolder
impl FatFolder {
    // New
    pub fn new() -> Self {
        Self {
            myself: FatObject::new(),
            fatobjs: LinkedList::new(),
        }
    }

    // Init
    pub fn init(diskio: &mut FatDiskio, selfobj: FatObject) -> Self {
        let mut folder = Self::new();
        folder.open(diskio, selfobj);
        folder
    }

    // Create fatobj
    pub fn create(diskio: &mut FatDiskio, mut selfobj: FatObject, name: &str, attr: u8) -> Option<FatObject> {
        let myself_clust = selfobj.get_fst_clust();
        let newobj_clust = diskio.alloc_cluster(1);

        let mut newobj = vec![FatObject::new(); 1];
        newobj[0].set_name(name);
        newobj[0].set_attribute(attr);
        newobj[0].set_fst_clust(newobj_clust);

        if Self::init(diskio, selfobj).write(diskio, &mut newobj) {
            if (attr & FatEntryAttr::DIRECTORY) != 0 {
                let mut dotobjs = vec![FatObject::new(); 2];
                dotobjs[0] = FatObject::new_dot_dir(newobj_clust);
                dotobjs[1] = FatObject::new_dot_dot_dir(myself_clust);
                Self::init(diskio, newobj[0].clone()).write(diskio, &mut dotobjs);
            }
            return Some(newobj[0].clone());
        }

        None
    }

    // Search fatobj
    pub fn search(diskio: &mut FatDiskio, selfobj: FatObject, name: &str) -> Option<FatObject> {
        let mut folder = Self::init(diskio, selfobj);

        for fatobj in folder.fatobjs.iter_mut() {
            if fatobj.get_name() == name {
                return Some(fatobj.clone());
            }
        }

        None
    }

    // Update fatobj
    pub fn update(diskio: &mut FatDiskio, mut fatobj: FatObject) {
        let mut folder = Self::new();
        folder.write_obj(diskio, &mut fatobj);
    }

    // Remove fatobj
    pub fn remove(diskio: &mut FatDiskio, mut fatobj: FatObject) {
        fatobj.set_object_free();
        let mut folder = Self::new();
        folder.write_obj(diskio, &mut fatobj);
    }

    // Set volume label
    pub fn set_vol_lab(diskio: &mut FatDiskio, name: &str) {
        let mut folder = Self::init(diskio, FatObject::root());
        for fatobj in folder.fatobjs.iter_mut() {
            if fatobj.get_object_type() == FileType::Volume {
                fatobj.set_name(name);
                Self::update(diskio, fatobj.clone());
                break;
            }
        }
    }

    // Get volume label
    pub fn get_vol_lab(diskio: &mut FatDiskio) -> String {
        let mut folder = Self::init(diskio, FatObject::root());
        for fatobj in folder.fatobjs.iter_mut() {
            if fatobj.get_object_type() == FileType::Volume {
                return fatobj.get_name();
            }
        }
        "NONAME".to_string()
    }
}

// Impl FatFolder
impl FatFolder {
    // Find space
    fn find_space(&mut self, diskio: &mut FatDiskio, req_size: usize) -> Option<DiskIndex> {
        let mut start_idx = DiskIndex::new();
        let mut free_cnt = 0;

        let mut iter = FatEntryIterator::new(diskio, self.myself.get_fst_clust());

        loop {
            if iter.get_item().is_none() {
                free_cnt += 1;

                if free_cnt == 1 {
                    start_idx = iter.get_index();
                }

                if free_cnt >= req_size {
                    return Some(start_idx);
                }
            } else {
                free_cnt = 0;
            }

            if !iter.next() {
                break;
            }
        }

        None
    }

    // Write entries
    fn write_obj(&mut self, diskio: &mut FatDiskio, fatobj: &mut FatObject) -> usize {
        let entries = fatobj.get_all_entries();

        let mut iter = FatEntryIterator::from(diskio, fatobj.get_index()).wrt_mode();

        for (i, entry) in entries.iter().enumerate() {
            iter.set_item(*entry);

            if !iter.next() {
                return i;
            }
        }

        entries.len()
    }
}

// Impl FatFolder
impl FatFolder {
    // Open
    pub fn open(&mut self, diskio: &mut FatDiskio, selfobj: FatObject) {
        self.myself = selfobj;

        if self.myself.get_object_type() == FileType::Directory {
            let mut start_index = DiskIndex::new();
            let mut entries = Vec::new();
            
            let mut iter = FatEntryIterator::new(diskio, self.myself.get_fst_clust());

            loop {
                if let Some(entry) = iter.get_item() {
                    // Record entry index
                    if entries.len() == 0 {
                        start_index = iter.get_index();
                    }

                    // Add entry into entries
                    entries.push(entry);

                    // Create fatobj when entry is short entry
                    if let FatEntry::Short(_) = entry {
                        let mut fatobj = FatObject::from(&mut entries);
                        fatobj.set_index(start_index.clone());
                        self.fatobjs.push(fatobj);
                        entries.clear();
                    }
                }

                if !iter.next() {
                    break;
                }
            }
        }
    }

    // Write
    pub fn write(&mut self, diskio: &mut FatDiskio, subobjs: &mut [FatObject]) -> bool {
        for obj in subobjs {
            let size = obj.get_all_entries().len();
            if let Some(index) = self.find_space(diskio, size) {
                obj.set_index(index.clone());
                if self.write_obj(diskio, obj) != size {
                    return false;
                }
                self.fatobjs.push(obj.clone());
            }
        }
        true
    }

    // Read
    pub fn read(&mut self, subobjs: &mut [FatObject]) -> usize {
        let size = self.fatobjs.len().min(subobjs.len());

        let zip = subobjs.iter_mut().zip(self.fatobjs.iter_mut().take(size));

        for (target, source) in zip {
            target.clone_from(source);
        }

        size
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
