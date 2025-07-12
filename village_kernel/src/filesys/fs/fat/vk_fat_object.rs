//###########################################################################
// vk_fat_object.rs
// The specific implementation of functions related to fat object
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_fat_diskio::DiskIndex;
use super::vk_fat_entry::{FatEntryAttr, FatEntryNSFlag};
use super::vk_fat_entry::{FatEntry, FatLongEntry, FatShortEntry};
use crate::traits::vk_filesys::{FileAttr, FileType};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

// Const members
const LONG_NAME_SIZE: u8 = 13;
const DIR_SEQ_FLAG: u8 = 0x40;
const DIR_FREE_FLAG: u8 = 0xe5;

// Struct FatObject
#[derive(Clone)]
pub struct FatObject {
    index: DiskIndex,
    short_entry: FatShortEntry,
    long_entries: Option<Box<[FatLongEntry]>>,
}

// Impl FatObject
impl FatObject {
    // New
    pub const fn new() -> Self {
        Self {
            index: DiskIndex::new(),
            short_entry: FatShortEntry::new(),
            long_entries: None,
        }
    }

    // New dir
    pub fn new_dir(name: &str) -> Self {
        let mut obj = Self::new();
        obj.set_name(name);
        obj.set_attribute(FatEntryAttr::DIRECTORY);
        obj
    }

    // New file
    pub fn new_file(name: &str) -> Self {
        let mut obj = Self::new();
        obj.set_name(name);
        obj.set_attribute(FatEntryAttr::FILE);
        obj
    }

    // New root
    pub fn root() -> Self {
        let mut obj = Self::new();
        obj.set_name("/");
        obj.set_attribute(FatEntryAttr::DIRECTORY);
        obj
    }

    // New dot dir
    pub fn new_dot_dir(fst_clust: u32) -> Self {
        let mut obj = Self::new();
        obj.set_name(".");
        obj.set_fst_clust(fst_clust);
        obj.set_attribute(FatEntryAttr::DIRECTORY | FatEntryAttr::HIDDEN);
        obj
    }

    // New dot dot dir
    pub fn new_dot_dot_dir(fst_clust: u32) -> Self {
        let mut obj = Self::new();
        obj.set_name("..");
        obj.set_fst_clust(fst_clust);
        obj.set_attribute(FatEntryAttr::DIRECTORY | FatEntryAttr::HIDDEN);
        obj
    }

    // from entries
    pub fn from(entries: &mut [FatEntry]) -> Self {
        let mut obj = Self::new();

        let mut long_entries = Vec::new();

        for entry in entries {
            match entry {
                FatEntry::Long(entry) => long_entries.push(entry.clone()),
                FatEntry::Short(entry) => obj.short_entry = entry.clone(),
            }
        }

        if long_entries.len() > 0 {
            obj.long_entries = Some(long_entries.into_boxed_slice());
        }

        obj
    }
}

// Impl FatObject
impl FatObject {
    // Set object free
    pub fn set_object_free(&mut self) {
        self.short_entry.name[0] = DIR_FREE_FLAG;

        if let Some(long_entries) = &mut self.long_entries {
            for entry in long_entries.iter_mut() {
                entry.ord = DIR_FREE_FLAG;
            }
        }
    }

    // Get object name
    pub fn get_object_name(&mut self) -> String {
        if let Some(long_name) = self.get_long_name() {
            long_name
        } else {
            self.get_short_name()
        }
    }

    // Get object type
    pub fn get_object_type(&mut self) -> FileType {
        match self.short_entry.attr & (FatEntryAttr::DIRECTORY | FatEntryAttr::VOLUME_ID) {
            x if x == FatEntryAttr::FILE => FileType::File,
            x if x == FatEntryAttr::DIRECTORY => FileType::Directory,
            x if x == FatEntryAttr::VOLUME_ID => FileType::Volume,
            _ => FileType::Unknown,
        }
    }

    // Get object attr
    pub fn get_object_attr(&mut self) -> FileAttr {
        if (self.short_entry.attr & FatEntryAttr::HIDDEN) != 0 {
            FileAttr::Hidden
        } else {
            FileAttr::Visible
        }
    }

    // Get all entries
    pub fn get_all_entries(&mut self) -> Vec<FatEntry> {
        let mut entries = Vec::new();

        // Add long name entries
        if let Some(long_entries) = &self.long_entries {
            entries.extend(long_entries.iter().map(|e| FatEntry::Long(*e)));
        }

        // Add short name entry
        entries.push(FatEntry::Short(self.short_entry));

        entries
    }
}

// Impl FatObject
impl FatObject {
    // Set name
    pub fn set_name(&mut self, name: &str) {
        self.clear_long_name();

        if Self::requires_long_name(name) {
            self.set_long_name(name);
        }

        self.set_short_name(name)
    }

    // Get name
    pub fn get_name(&mut self) -> String {
        if let Some(long_name) = self.get_long_name() {
            return long_name;
        }
        self.get_short_name()
    }

    // Is long name
    pub fn is_long_name(&mut self) -> bool {
        self.long_entries.is_some()
    }

    // Set short name
    fn set_short_name(&mut self, name: &str) {
        let mut is_body_lowed_case = true;
        let mut is_ext_lowed_caset = true;

        // Special cases for "." and ".."
        if name == "." {
            self.short_entry.name[0] = '.' as u8;
            return;
        } else if name == ".." {
            self.short_entry.name[0] = '.' as u8;
            self.short_entry.name[1] = '.' as u8;
            return;
        }

        // Split 83 name
        let (base, ext) = Self::split_83_name(name);

        // Name body
        for (i, &c) in base.iter().enumerate() {
            if is_body_lowed_case && (c >= b'A' && c <= b'Z') {
                is_body_lowed_case = false;
            }

            self.short_entry.name[i] = c.to_ascii_uppercase();
        }

        // Name ext
        for (i, &c) in ext.iter().enumerate() {
            if is_ext_lowed_caset && (c >= b'A' && c <= b'Z') {
                is_ext_lowed_caset = false;
            }

            self.short_entry.name[8 + i] = c.to_ascii_uppercase();
        }

        // Set nt res
        if is_body_lowed_case {
            self.short_entry.nt_res |= FatEntryNSFlag::BODY;
        }

        if is_ext_lowed_caset {
            self.short_entry.nt_res |= FatEntryNSFlag::EXT;
        }
    }

    // Get short name
    fn get_short_name(&mut self) -> String {
        let mut name = String::new();
        let is_body_lowed_case = (self.short_entry.nt_res & FatEntryNSFlag::BODY) != 0;
        let is_ext_lowed_case = (self.short_entry.nt_res & FatEntryNSFlag::EXT) != 0;

        // 8.3 name body
        for &c in &self.short_entry.name[..8] {
            if c == b' ' {
                break;
            }
            if is_body_lowed_case && (c >= b'A' && c <= b'Z') {
                name.push(c.to_ascii_lowercase() as char);
            } else {
                name.push(c as char);
            }
        }

        if self.short_entry.name[8] != b' ' {
            // 8.3 name dot
            if FileType::Volume == self.get_object_type() {
                name.push(' ');
            } else {
                name.push('.');
            }

            // 8.3 name ext
            for &c in &self.short_entry.name[8..11] {
                if c == b' ' {
                    break;
                }
                if is_ext_lowed_case && (c >= b'A' && c <= b'Z') {
                    name.push(c.to_ascii_lowercase() as char);
                } else {
                    name.push(c as char);
                }
            }
        }

        name
    }

    // Set long name
    fn set_long_name(&mut self, name: &str) {
        let checksum = Self::calculate_checksum(&self.short_entry.name);
        let entries_needed = Self::calc_lfe_needed(name);

        let mut entries = vec![FatLongEntry::new(); entries_needed].into_boxed_slice();

        // Fill long name entries
        let chars = name.encode_utf16().collect::<Vec<_>>();
        let mut pos = 0;

        for (i, entry) in entries.iter_mut().rev().enumerate() {
            entry.ord = (entries_needed - i) as u8 | DIR_SEQ_FLAG;
            entry.attr = 0x0F;
            entry.chksum = checksum;

            // Fill names
            for j in 0..5 {
                entry.name1[j] = chars.get(pos).copied().unwrap_or(0);
                pos += 1;
            }

            for j in 0..6 {
                entry.name2[j] = chars.get(pos).copied().unwrap_or(0);
                pos += 1;
            }

            for j in 0..2 {
                entry.name3[j] = chars.get(pos).copied().unwrap_or(0);
                pos += 1;
            }
        }

        // Set store size
        entries[0].ord = DIR_SEQ_FLAG + entries.len() as u8;

        // Set long entries
        self.long_entries = Some(entries);
    }

    // Get long name
    fn get_long_name(&mut self) -> Option<String> {
        if let Some(long_entries) = &self.long_entries {
            let mut name = String::new();

            for entry in long_entries.iter().rev() {
                for &c in &entry.name1 {
                    if c == 0 {
                        return Some(name);
                    }
                    name.push(char::from_u32(c as u32)?);
                }

                for &c in &entry.name2 {
                    if c == 0 {
                        return Some(name);
                    }
                    name.push(char::from_u32(c as u32)?);
                }

                for &c in &entry.name3 {
                    if c == 0 {
                        return Some(name);
                    }
                    name.push(char::from_u32(c as u32)?);
                }
            }

            return Some(name);
        }

        None
    }

    // Clear long name
    fn clear_long_name(&mut self) {
        self.long_entries = None;
    }

    // calc chksum
    fn calculate_checksum(name: &[u8]) -> u8 {
        let mut sum = 0u8;
        for &c in name {
            sum = ((sum & 1) << 7) + (sum >> 1) + c;
        }
        sum
    }

    // Requires long name
    fn requires_long_name(name: &str) -> bool {
        let (base, ext) = Self::split_83_name(name);
        base.len() > 8 || ext.len() > 3 || name.chars().any(|c| c > '\u{7F}')
    }

    // Split 8.3 name
    fn split_83_name(name: &str) -> (&[u8], &[u8]) {
        let bytes = name.as_bytes();
        let dot_pos = bytes.iter().position(|&b| b == b'.').unwrap_or(bytes.len());

        let base = if dot_pos > 8 {
            &bytes[..8]
        } else {
            &bytes[..dot_pos]
        };
        let ext = if dot_pos < bytes.len() {
            &bytes[dot_pos + 1..]
        } else {
            &[]
        };

        (base, if ext.len() > 3 { &ext[..3] } else { ext })
    }

    // Calculate lfn entries needed
    fn calc_lfe_needed(name: &str) -> usize {
        let char_count = name.chars().count();
        let long_name_size = LONG_NAME_SIZE as usize;
        (char_count + long_name_size - 2) / (long_name_size - 1)
    }
}

// Impl FatObject info
impl FatObject {
    // Set attribute
    pub fn set_attribute(&mut self, attr: u8) {
        self.short_entry.attr = attr;
    }

    // Get attribute
    pub fn get_attribute(&self) -> u8 {
        self.short_entry.attr
    }

    // Set nt res
    pub fn set_nt_res(&mut self, nt_res: u8) {
        self.short_entry.nt_res = nt_res;
    }

    // Get nt res
    pub fn get_nt_res(&self) -> u8 {
        self.short_entry.nt_res
    }

    // Set create tenth
    pub fn set_create_tenth(&mut self, tenth: u16) {
        self.short_entry.crt_time_tenth = tenth as u8;
    }

    // Get create tenth
    pub fn get_create_tenth(&self) -> u16 {
        self.short_entry.crt_time_tenth as u16
    }

    // Set create time
    pub fn set_create_time(&mut self, time: u16) {
        self.short_entry.crt_time = time;
    }

    // Get create time
    pub fn get_create_time(&self) -> u16 {
        self.short_entry.crt_time
    }

    // Set create date
    pub fn set_create_date(&mut self, date: u16) {
        self.short_entry.crt_date = date;
    }

    // Get create date
    pub fn get_create_date(&self) -> u16 {
        self.short_entry.crt_date
    }

    // Set last acc date
    pub fn set_last_acc_date(&mut self, date: u16) {
        self.short_entry.lst_acc_date = date;
    }

    // Get last acc date
    pub fn get_last_acc_date(&self) -> u16 {
        self.short_entry.lst_acc_date
    }

    // Set write
    pub fn set_write_time(&mut self, time: u16) {
        self.short_entry.wrt_time = time;
    }

    // Get write time
    pub fn get_write_time(&self) -> u16 {
        self.short_entry.wrt_time
    }

    // Set write data
    pub fn set_write_date(&mut self, date: u16) {
        self.short_entry.wrt_date = date;
    }

    // Get write data
    pub fn get_write_date(&self) -> u16 {
        self.short_entry.wrt_date
    }

    // Set first cluster
    pub fn set_fst_clust(&mut self, clust: u32) {
        self.short_entry.fst_clust_hi = (clust >> 16) as u16;
        self.short_entry.fst_clust_lo = clust as u16;
    }

    // Get first cluster
    pub fn get_fst_clust(&mut self) -> u32 {
        ((self.short_entry.fst_clust_hi as u32) << 16) | (self.short_entry.fst_clust_lo as u32)
    }

    // Set file size
    pub fn set_file_size(&mut self, size: u32) {
        self.short_entry.file_size = size;
    }

    // Get file size
    pub fn get_file_size(&mut self) -> u32 {
        self.short_entry.file_size
    }

    // Set disk index
    pub fn set_index(&mut self, index: DiskIndex) {
        self.index = index;
    }

    // Get disk index
    pub fn get_index(&mut self) -> DiskIndex {
        self.index.clone()
    }
}
