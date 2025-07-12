//###########################################################################
// vk_fat_entry.rs
// The specific implementation of functions related to fat entry
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_fat_diskio::{FatDiskio, DiskIndex};
use alloc::vec;
use alloc::vec::Vec;

// Const members
const DIR_ENTRY_SIZE: u8 = 32;
const DIR_SEQ_FLAG: u8 = 0x40;
const DIR_FREE_FLAG: u8 = 0xe5;
const DIR_VALID_FLAG: u8 = 0x0;

// Flag FatEntryAttr
pub struct FatEntryAttr;

// Impl EntryAttr
impl FatEntryAttr {
    pub const FILE: u8 = 0x00;
    pub const READ_ONLY: u8 = 0x01;
    pub const HIDDEN: u8 = 0x02;
    pub const SYSTEM: u8 = 0x04;
    pub const VOLUME_ID: u8 = 0x08;
    pub const DIRECTORY: u8 = 0x10;
    pub const ARCHIVE: u8 = 0x20;
    pub const LONG_NAME: u8 = 0x0f;
    pub const LONG_NAME_MASK: u8 = 0x3f;
}

// Flag FatEntryNS
pub struct FatEntryNSFlag;

// Impl FatEntryNS
impl FatEntryNSFlag {
    pub const NOE: u8 = 0x00;
    pub const LOSS: u8 = 0x01; /* Out of 8.3 format */
    pub const LFN: u8 = 0x02; /* Force to create LFN entry */
    pub const LAST: u8 = 0x04; /* Last segment */
    pub const BODY: u8 = 0x08; /* Lower case flag (body) */
    pub const EXT: u8 = 0x10; /* Lower case flag (ext) */
    pub const DOT: u8 = 0x20; /* Dot entry */
    pub const NOLFN: u8 = 0x40; /* Do not find LFN */
    pub const NONAME: u8 = 0x80; /* Not followed */
}

// Struct FatShortEntry
#[derive(Debug, Clone, Copy)]
pub struct FatShortEntry {
    pub name: [u8; 11],
    pub attr: u8,
    pub nt_res: u8,
    pub crt_time_tenth: u8,
    pub crt_time: u16,
    pub crt_date: u16,
    pub lst_acc_date: u16,
    pub fst_clust_hi: u16,
    pub wrt_time: u16,
    pub wrt_date: u16,
    pub fst_clust_lo: u16,
    pub file_size: u32,
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
    pub ord: u8,
    pub name1: [u16; 5],
    pub attr: u8,
    pub typ: u8,
    pub chksum: u8,
    pub name2: [u16; 6],
    pub fst_clust_lo: u16,
    pub name3: [u16; 2],
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
            entry.name1[i] = u16::from_le_bytes([data[1 + i * 2], data[1 + i * 2 + 1]]);
        }

        entry.attr = data[11];
        entry.typ = data[12];
        entry.chksum = data[13];

        // Copy name2
        for i in 0..6 {
            entry.name2[i] = u16::from_le_bytes([data[14 + i * 2], data[14 + i * 2 + 1]]);
        }

        entry.fst_clust_lo = u16::from_le_bytes([data[26], data[27]]);

        // Copy name3
        for i in 0..2 {
            entry.name3[i] = u16::from_le_bytes([data[28 + i * 2], data[28 + i * 2 + 1]]);
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
            bytes[1 + i * 2] = bytes_le[0];
            bytes[1 + i * 2 + 1] = bytes_le[1];
        }

        bytes[11] = self.attr;
        bytes[12] = self.typ;
        bytes[13] = self.chksum;

        // Copy name2
        for i in 0..6 {
            let bytes_le = self.name2[i].to_le_bytes();
            bytes[14 + i * 2] = bytes_le[0];
            bytes[14 + i * 2 + 1] = bytes_le[1];
        }

        // Copy fst_clust_lo
        let fst_clust_bytes = self.fst_clust_lo.to_le_bytes();
        bytes[26] = fst_clust_bytes[0];
        bytes[27] = fst_clust_bytes[1];

        // Copy name3
        for i in 0..2 {
            let bytes_le = self.name3[i].to_le_bytes();
            bytes[28 + i * 2] = bytes_le[0];
            bytes[28 + i * 2 + 1] = bytes_le[1];
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
        if bytes.len() >= DIR_ENTRY_SIZE as usize
            && bytes[0] != DIR_FREE_FLAG
            && bytes[0] > DIR_VALID_FLAG
        {
            let attr = bytes[11] & (FatEntryAttr::DIRECTORY | FatEntryAttr::VOLUME_ID);
            if attr == FatEntryAttr::FILE
                || attr == FatEntryAttr::DIRECTORY
                || attr == FatEntryAttr::VOLUME_ID
                || Self::is_long_entry(bytes)
            {
                return true;
            }
        }
        return false;
    }

    // Is long name entry
    fn is_long_entry(bytes: &[u8]) -> bool {
        (bytes[11] & FatEntryAttr::LONG_NAME_MASK) == FatEntryAttr::LONG_NAME
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

    // Size
    pub fn size(&self) -> usize {
        match self {
            FatEntry::Long(entry) => (entry.ord - DIR_SEQ_FLAG + 1) as usize,
            FatEntry::Short(_) => 1,
        }
    }
}

// Struct FatEntryIterator
pub struct FatEntryIterator<'a> {
    diskio: &'a mut FatDiskio,
    diskidx: DiskIndex,
    disksec: Vec<u8>,
    wrt_mode: bool,
}

// Impl FatEntryIterator
impl<'a> FatEntryIterator<'a> {
    // New
    pub fn new(diskio: &'a mut FatDiskio, fst_clust: u32) -> Self {
        let diskidx = diskio.get_frist_index(fst_clust);
        let mut disksec = vec![0u8; diskio.get_info().bytes_per_sec as usize];
        diskio.read_sector(&mut disksec, diskidx.sector, 1);
        Self {
            diskio,
            diskidx,
            disksec,
            wrt_mode: false,
        }
    }

    // From
    pub fn from(diskio: &'a mut FatDiskio, diskidx: DiskIndex) -> Self {
        let mut disksec = vec![0u8; diskio.get_info().bytes_per_sec as usize];
        diskio.read_sector(&mut disksec, diskidx.sector, 1);
        Self {
            diskio,
            diskidx,
            disksec,
            wrt_mode: false,
        }
    }

    // Write mode
    pub fn wrt_mode(mut self) -> Self {
        self.wrt_mode = true;
        self
    }

    // Next
    pub fn next(&mut self) -> bool {
        self.diskidx.index += 1;

        if self.diskidx.index >= self.diskio.get_info().entries_per_sec {
            if self.wrt_mode {
                self.diskio.write_sector(&self.disksec, self.diskidx.sector, 1);
            }

            self.diskio.get_next_index(&mut self.diskidx);

            if self.diskidx.sector != 0 {
                self.diskio.read_sector(&mut self.disksec, self.diskidx.sector, 1);
                self.diskidx.index = 0;
            } else {
                return false;
            }
        }

        true
    }

    // Set item
    pub fn set_item(&mut self, entry: FatEntry) {
        let offset = self.diskidx.index as usize * DIR_ENTRY_SIZE  as usize;
        self.disksec[offset..(offset + DIR_ENTRY_SIZE as usize)].copy_from_slice(&entry.as_bytes());
    }

    // Get item
    pub fn get_item(&self) -> Option<FatEntry> {
        let offset = self.diskidx.index as usize * DIR_ENTRY_SIZE as usize;
        FatEntry::from_bytes(&self.disksec[offset..(offset + DIR_ENTRY_SIZE as usize)])
    }

    // Get index
    pub fn get_index(&self) -> DiskIndex {
        self.diskidx.clone()
    }
}

// Impl Drop for FatEntryIterator
impl<'a> Drop for FatEntryIterator<'a> {
    fn drop(&mut self) {
        if self.wrt_mode {
            self.diskio.write_sector(&mut self.disksec, self.diskidx.sector, 1);
        }
        self.disksec.clear();
    }
}
