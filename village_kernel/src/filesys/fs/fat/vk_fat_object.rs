//###########################################################################
// vk_fat_object.rs
// The specific implementation of functions related to fat object
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use super::vk_fat_diskio::DiskIndex;
use crate::traits::vk_filesys::{FileAttr, FileType};

// Const members
const DIR_ENTRY_SIZE: u8 = 32;
const LONG_NAME_SIZE: u8 = 13;
const DIR_SEQ_FLAG: u8 = 0x40;
const DIR_FREE_FLAG: u8 = 0xe5;
const DIR_VALID_FLAG: u8 = 0x0;

// Flag EntryAttr
pub struct EntryAttr;

// Impl EntryAttr
impl EntryAttr{
    pub const FILE: u8           = 0x00;
    pub const READ_ONLY: u8      = 0x01;
    pub const HIDDEN: u8         = 0x02;
    pub const SYSTEM: u8         = 0x04;
    pub const VOLUME_ID: u8      = 0x08;
    pub const DIRECTORY: u8      = 0x10;
    pub const ARCHIVE: u8        = 0x20;
    pub const LONG_NAME: u8      = 0x0f;
    pub const LONG_NAME_MASK: u8 = 0x3f;
}

// Flag NS
pub struct NSFlag;

// Impl NSFlag
impl NSFlag {
    pub const NOE: u8       = 0x00;
    pub const LOSS: u8      = 0x01;   /* Out of 8.3 format */
    pub const LFN: u8       = 0x02;   /* Force to create LFN entry */
    pub const LAST: u8      = 0x04;   /* Last segment */
    pub const BODY: u8      = 0x08;   /* Lower case flag (body) */
    pub const EXT: u8       = 0x10;   /* Lower case flag (ext) */
    pub const DOT: u8       = 0x20;   /* Dot entry */
    pub const NOLFN: u8     = 0x40;   /* Do not find LFN */
    pub const NONAME: u8    = 0x80;   /* Not followed */
}

// Struct FatShortEntry
#[derive(Debug, Clone, Copy)]
pub struct FatShortEntry {
    name: [u8; 11],
    attr: u8,
    nt_res: u8,
    crt_time_tenth: u8,
    crt_time: u16,
    crt_date: u16,
    lst_acc_date: u16,
    fst_clust_hi: u16,
    wrt_time: u16,
    wrt_date: u16,
    fst_clust_lo: u16,
    file_size: u32,
}

// Impl FatShortEntry
impl FatShortEntry {
    // New
    pub const fn new() -> Self {
        Self {
            name: [0x20u8; 11],
            attr: 0,
            nt_res: 0,
            crt_time_tenth: 0,
            crt_time: 0,
            crt_date: 0,
            lst_acc_date: 0,
            fst_clust_hi: 0,
            wrt_time: 0,
            wrt_date: 0,
            fst_clust_lo: 0,
            file_size: 0,
        }
    }

    // From bytes
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut entry = Self::new();
        
        entry.name.copy_from_slice(&data[0..11]);
        entry.attr = data[11];
        entry.nt_res = data[12];
        entry.crt_time_tenth = data[13];
        entry.crt_time = u16::from_le_bytes([data[14], data[15]]);
        entry.crt_date = u16::from_le_bytes([data[16], data[17]]);
        entry.lst_acc_date = u16::from_le_bytes([data[18], data[19]]);
        entry.fst_clust_hi = u16::from_le_bytes([data[20], data[21]]);
        entry.wrt_time = u16::from_le_bytes([data[22], data[23]]);
        entry.wrt_date = u16::from_le_bytes([data[24], data[25]]);
        entry.fst_clust_lo = u16::from_le_bytes([data[26], data[27]]);
        entry.file_size = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);

        entry
    }

    // As bytes
    pub fn as_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        
        bytes[0..11].copy_from_slice(&self.name);
        bytes[11] = self.attr;
        bytes[12] = self.nt_res;
        bytes[13] = self.crt_time_tenth;
        bytes[14..16].copy_from_slice(&self.crt_time.to_le_bytes());
        bytes[16..18].copy_from_slice(&self.crt_date.to_le_bytes());
        bytes[18..20].copy_from_slice(&self.lst_acc_date.to_le_bytes());
        bytes[20..22].copy_from_slice(&self.fst_clust_hi.to_le_bytes());
        bytes[22..24].copy_from_slice(&self.wrt_time.to_le_bytes());
        bytes[24..26].copy_from_slice(&self.wrt_date.to_le_bytes());
        bytes[26..28].copy_from_slice(&self.fst_clust_lo.to_le_bytes());
        bytes[28..32].copy_from_slice(&self.file_size.to_le_bytes());
        
        bytes
    }
}

// Struct FatLongEntry
#[derive(Debug, Clone, Copy)]
pub struct FatLongEntry {
    ord: u8,
    name1: [u16; 5],
    attr: u8,
    typ: u8,
    chksum: u8,
    name2: [u16; 6],
    fst_clust_lo: u16,
    name3: [u16; 2],
}

// Impl FatLongEntry
impl FatLongEntry {
    // New
    pub const fn new() -> Self {
        Self {
            ord: 0,
            name1: [0xffffu16; 5],
            attr: 0,
            typ: 0,
            chksum: 0,
            name2: [0xffffu16; 6],
            fst_clust_lo: 0,
            name3: [0xffffu16; 2],
        }
    }

    // From bytes
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut entry = Self::new();

        entry.ord = data[0];

        // Copy name1
        for i in 0..5 {
            entry.name1[i] = u16::from_le_bytes([data[1 + i*2], data[1 + i*2 + 1]]);
        }
        
        entry.attr = data[11];
        entry.typ = data[12];
        entry.chksum = data[13];
        
        // Copy name2
        for i in 0..6 {
            entry.name2[i] = u16::from_le_bytes([data[14 + i*2], data[14 + i*2 + 1]]);
        }
        
        entry.fst_clust_lo = u16::from_le_bytes([data[26], data[27]]);
        
        // Copy name3
        for i in 0..2 {
            entry.name3[i] = u16::from_le_bytes([data[28 + i*2], data[28 + i*2 + 1]]);
        }
        
        entry
    }

    // As bytes
    pub fn as_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        
        bytes[0] = self.ord;
        
        // Copy name1
        for i in 0..5 {
            let bytes_le = self.name1[i].to_le_bytes();
            bytes[1 + i*2] = bytes_le[0];
            bytes[1 + i*2 + 1] = bytes_le[1];
        }
        
        bytes[11] = self.attr;
        bytes[12] = self.typ;
        bytes[13] = self.chksum;
        
        // Copy name2
        for i in 0..6 {
            let bytes_le = self.name2[i].to_le_bytes();
            bytes[14 + i*2] = bytes_le[0];
            bytes[14 + i*2 + 1] = bytes_le[1];
        }
        
        // Copy fst_clust_lo
        let fst_clust_bytes = self.fst_clust_lo.to_le_bytes();
        bytes[26] = fst_clust_bytes[0];
        bytes[27] = fst_clust_bytes[1];
        
        // Copy name3
        for i in 0..2 {
            let bytes_le = self.name3[i].to_le_bytes();
            bytes[28 + i*2] = bytes_le[0];
            bytes[28 + i*2 + 1] = bytes_le[1];
        }
        
        bytes
    }
}

// Enum FatEntry
#[derive(Debug, Clone, Copy)]
pub enum FatEntry {
    Long(FatLongEntry),
    Short(FatShortEntry),
}

// Impl FatEntry
impl FatEntry {
    // Is Valid
    fn is_valid(bytes: &[u8]) -> bool {
        if bytes.len() >= DIR_ENTRY_SIZE as usize && bytes[0] != DIR_FREE_FLAG && bytes[0] > DIR_VALID_FLAG {
            let attr = bytes[11] & (EntryAttr::DIRECTORY | EntryAttr::VOLUME_ID);
            if attr == EntryAttr::FILE      ||
               attr == EntryAttr::DIRECTORY ||
               attr == EntryAttr::VOLUME_ID ||
               Self::is_long_entry(bytes) {
                return true;
            }
        }
        return false;
    }

    // Is long name entry
    fn is_long_entry(bytes: &[u8]) -> bool {
        (bytes[11] & EntryAttr::LONG_NAME_MASK) == EntryAttr::LONG_NAME
    }

    // From bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Check is valid
        if !Self::is_valid(bytes) {
            return None;
        }

        // Long Entry
        if Self::is_long_entry(bytes) {
            return Some(FatEntry::Long(FatLongEntry::from_bytes(bytes)));
        }
        
        // Short Entry
        Some(FatEntry::Short(FatShortEntry::from_bytes(bytes)))
    }

    // As bytes
    pub fn as_bytes(&self) -> [u8; 32] {
        match self {
            FatEntry::Long(entry) => entry.as_bytes(),
            FatEntry::Short(entry) => entry.as_bytes(),
        }
    }

    // Store Size
    pub fn get_store_size(&self) -> usize {
        match self {
            FatEntry::Long(entry) => (entry.ord - DIR_SEQ_FLAG + 1) as usize,
            FatEntry::Short(_) => 1,
        }
    }
}

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
        obj.set_attribute(EntryAttr::DIRECTORY);
        obj
    }

    // New file
    pub fn new_file(name: &str) -> Self {
        let mut obj = Self::new();
        obj.set_name(name);
        obj.set_attribute(EntryAttr::FILE);
        obj
    }

    // New root
    pub fn root() -> Self {
        let mut obj = Self::new();
        obj.set_name("/");
        obj.set_attribute(EntryAttr::DIRECTORY);
        obj
    }

    // New dot dir
    pub fn new_dot_dir(fst_clust: u32) -> Self {
        let mut obj = Self::new();
        obj.set_name(".");
        obj.set_first_cluster(fst_clust);
        obj.set_attribute(EntryAttr::DIRECTORY | EntryAttr::HIDDEN);
        obj
    }

    // New dot dot dir
    pub fn new_dot_dot_dir(fst_clust: u32) -> Self {
        let mut obj = Self::new();
        obj.set_name("..");
        obj.set_first_cluster(fst_clust);
        obj.set_attribute(EntryAttr::DIRECTORY | EntryAttr::HIDDEN);
        obj
    }

    // from entries
    pub fn from_entries(entries: &mut [FatEntry]) -> Self {
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
        match self.short_entry.attr & (EntryAttr::DIRECTORY | EntryAttr::VOLUME_ID) {
            x if x == EntryAttr::FILE => FileType::File,
            x if x == EntryAttr::DIRECTORY => FileType::Directory,
            x if x == EntryAttr::VOLUME_ID => FileType::Volume,
            _ => FileType::Unknown,
        }
    }

    // Get object attr
    pub fn get_object_attr(&mut self) -> FileAttr {
        if (self.short_entry.attr & EntryAttr::HIDDEN) != 0 {
            FileAttr::Hidden
        } else {
            FileAttr::Visible
        }
    }

    // Get all entries
    pub fn get_all_entries(&mut self) ->Vec<FatEntry> {
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
            return long_name
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
            self.short_entry.nt_res |= NSFlag::BODY;
        }

        if is_ext_lowed_caset {
            self.short_entry.nt_res |= NSFlag::EXT;
        }
    }

    // Get short name
    fn get_short_name(&mut self) -> String {
        let mut name = String::new();
        let is_body_lowed_case = (self.short_entry.nt_res & NSFlag::BODY) != 0;
        let is_ext_lowed_case  = (self.short_entry.nt_res & NSFlag::EXT) != 0;
        
        // 8.3 name body
        for &c in &self.short_entry.name[..8] {
            if c == b' ' { break; }
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
                if c == b' ' { break; }
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
        let entries_needed = Self::calculate_lfn_entries_needed(name);
        
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
                    if c == 0 { return Some(name); }
                    name.push(char::from_u32(c as u32)?);
                }
                
                for &c in &entry.name2 {
                    if c == 0 { return Some(name); }
                    name.push(char::from_u32(c as u32)?);
                }
                
                for &c in &entry.name3 {
                    if c == 0 { return Some(name); }
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
        
        let base = if dot_pos > 8 { &bytes[..8] } else { &bytes[..dot_pos] };
        let ext = if dot_pos < bytes.len() { &bytes[dot_pos+1..] } else { &[] };
        
        (base, if ext.len() > 3 { &ext[..3] } else { ext })
    }

    // Calculate lfn entries needed
    fn calculate_lfn_entries_needed(name: &str) -> usize {
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
    pub fn set_first_cluster(&mut self, clust: u32) {
        self.short_entry.fst_clust_hi = (clust >> 16) as u16;
        self.short_entry.fst_clust_lo = clust as u16;
    }

    // Get first cluster
    pub fn get_first_cluster(&mut self) -> u32 {
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
