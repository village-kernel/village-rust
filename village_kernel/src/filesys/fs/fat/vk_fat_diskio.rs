//###########################################################################
// vk_fat_diskio.rs
// The specific implementation of functions related to fat diskio
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::vec;
use alloc::vec::Vec;
use crate::{misc::fopts::vk_dev_fopt::DevFopt};

// Const mebers
const DIR_ENTRY_SIZE: u8 = 32;
const FAT16_EOC_FLAG: u16 = 0xfff8;
const FAT32_EOC_FLAG: u32 = 0xffffff8;

// Enum FatType
#[derive(PartialEq)]
pub enum FatType {
    None = 0,
    Fat12,
    Fat16,
    Fat32,
    Exfat,
}

// Struct BootSector
pub struct BootSector {
    pub jump_boot: [u8; 3],
    pub oem_name: [u8; 8],
}

// Impl BootSector
impl BootSector {
    // New
    pub const fn new() -> Self {
        Self {
            jump_boot: [0u8; 3],
            oem_name: [0u8; 8],
        }
    }

    // From
    pub fn from(data: &[u8]) -> Self {
        let mut bs = Self::new();
        bs.jump_boot.copy_from_slice(&data[0..3]);
        bs.oem_name.copy_from_slice(&data[3..11]);
        bs
    }
}

// Struct BiosParameterBlock
pub struct BiosParameterBlock {
    pub bytes_per_sec: u16,
    pub sec_per_clust: u8,
    pub reserved_sec_cnt: u16,
    pub num_fats: u8,
    pub root_ent_cnt: u16,
    pub tot_sec_16: u16,
    pub media: u8,
    pub fat_sz_16: u16,
    pub sec_per_trk: u16,
    pub num_heads: u16,
    pub hidd_sec: u32,
    pub tot_sec_32: u32,
}

// Impl BiosParameterBlock
impl BiosParameterBlock {
    // New
    pub const fn new() -> Self {
        Self {
            bytes_per_sec: 0,
            sec_per_clust: 0,
            reserved_sec_cnt: 0,
            num_fats: 0,
            root_ent_cnt: 0,
            tot_sec_16: 0,
            media: 0,
            fat_sz_16: 0,
            sec_per_trk: 0,
            num_heads: 0,
            hidd_sec: 0,
            tot_sec_32: 0,
        }
    }

    // From
    pub fn from(data: &[u8]) -> Self {
        let mut bpb = Self::new();

        bpb.bytes_per_sec = u16::from_le_bytes([data[0], data[1]]);
        bpb.sec_per_clust = data[2];
        bpb.reserved_sec_cnt = u16::from_le_bytes([data[3], data[4]]);
        bpb.num_fats = data[5];
        bpb.root_ent_cnt = u16::from_le_bytes([data[6], data[7]]);
        bpb.tot_sec_16 = u16::from_le_bytes([data[8], data[9]]);
        bpb.media = data[10];
        bpb.fat_sz_16 = u16::from_le_bytes([data[11], data[12]]);
        bpb.sec_per_trk = u16::from_le_bytes([data[13], data[14]]);
        bpb.num_heads = u16::from_le_bytes([data[15], data[16]]);
        bpb.hidd_sec = u32::from_le_bytes([data[17], data[18], data[19], data[20]]);
        bpb.tot_sec_32 = u32::from_le_bytes([data[21], data[22], data[23], data[24]]);

        bpb
    }
}

// Struct Fat1216
pub struct Fat1216 {
    pub drv_num: u8,
    pub reserved1: u8,
    pub boot_sig: u8,
    pub vol_id: u32,
    pub vol_lab: [u8; 11],
    pub fil_sys_type: [u8; 8],
    pub reserved: [u8; 28],
}

// Impl Fat1216
impl Fat1216 {
    // New
    pub const fn new() -> Self {
        Self {
            drv_num: 0,
            reserved1: 0,
            boot_sig: 0,
            vol_id: 0,
            vol_lab: [0u8; 11],
            fil_sys_type: [0u8; 8],
            reserved: [0u8; 28],
        }
    }

    // From
    pub fn from(data: &[u8]) -> Self {
        let mut fat = Self::new();

        fat.drv_num = data[0];
        fat.reserved1 = data[1];
        fat.boot_sig = data[2];
        fat.vol_id = u32::from_le_bytes([data[3], data[4], data[5], data[6]]);
        fat.vol_lab.copy_from_slice(&data[7..18]);
        fat.fil_sys_type.copy_from_slice(&data[18..26]);
        fat.reserved.copy_from_slice(&data[26..54]);

        fat
    }
}

// Struct Fat32
pub struct Fat32 {
    pub fat_sz_32: u32,
    pub ext_flags: u16,
    pub fs_ver: u16,
    pub root_clust: u32,
    pub fs_info: u16,
    pub bk_boot_sec: u16,
    pub reserved: [u8; 12],
    pub drv_num: u8,
    pub reserved1: u8,
    pub boot_sig: u8,
    pub vol_id: u32,
    pub vol_lab: [u8; 11],
    pub fil_sys_type: [u8; 8],
}

// Impl Fat32
impl Fat32 {
    // New
    pub const fn new() -> Self {
        Self {
            fat_sz_32: 0,
            ext_flags: 0,
            fs_ver: 0,
            root_clust: 0,
            fs_info: 0,
            bk_boot_sec: 0,
            reserved: [0u8; 12],
            drv_num: 0,
            reserved1: 0,
            boot_sig: 0,
            vol_id: 0,
            vol_lab: [0u8; 11],
            fil_sys_type: [0u8; 8],
        }
    }

    // From
    pub fn from(data: &[u8]) -> Self {
        let mut fat = Self::new();
        
        fat.fat_sz_32 = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        fat.ext_flags = u16::from_le_bytes([data[4], data[5]]);
        fat.fs_ver = u16::from_le_bytes([data[6], data[7]]);
        fat.root_clust = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        fat.fs_info = u16::from_le_bytes([data[12], data[13]]);
        fat.bk_boot_sec = u16::from_le_bytes([data[14], data[15]]);

        fat.reserved.copy_from_slice(&data[16..28]);
        fat.drv_num = data[28];
        fat.reserved1 = data[29];
        fat.boot_sig = data[30];
        fat.vol_id = u32::from_le_bytes([data[31], data[32], data[33], data[34]]);
        fat.vol_lab.copy_from_slice(&data[35..46]);
        fat.fil_sys_type.copy_from_slice(&data[46..54]);

        fat
    }
}

// Struct DBR
pub struct DBR {
    pub bs: BootSector,
    pub bpb: BiosParameterBlock,
    pub fat1216: Fat1216,
    pub fat32: Fat32,
    pub reserved: Vec<u8>,
    pub magic: u16,
}

// Impl DBR
impl DBR {
    // New
    pub const fn new() -> Self {
        Self {
            bs: BootSector::new(),
            bpb: BiosParameterBlock::new(),
            fat1216: Fat1216::new(),
            fat32: Fat32::new(),
            reserved: Vec::new(),
            magic: 0,
        }
    }

    // From
    pub fn from(data: &[u8]) -> Option<Self> {
        if data.len() < 512 {
            return None;
        }

        let mut dbr = Self::new();

        // Parser
        dbr.bs = BootSector::from(&data[0..11]);
        dbr.bpb = BiosParameterBlock::from(&data[11..36]);
        dbr.fat1216 = Fat1216::from(&data[36..90]);
        dbr.fat32 = Fat32::from(&data[36..90]);
        
        // Reserved
        //dbr.reserved = vec![0u8; 420];
        //dbr.reserved.copy_from_slice(&data[90..510]);

        // Get magic and check
        dbr.magic = u16::from_le_bytes([data[510], data[511]]);

        if dbr.magic != 0xAA55 {
            return None;
        }

        Some(dbr)
    }
}

// Struct FileSystemInfo
pub struct FileSystemInfo {
    pub fat_type: FatType,
    pub fat_size: u32,

    pub reserved_sec_cnt: u32,
    pub total_sectors: u32,
    pub cluster_count: u32,

    pub fat_start_sector: u32,
    pub fat_end_sector: u32,

    pub root_cluster: u32,
    pub root_start_sector: u32,
    pub root_sector_count: u32,

    pub data_start_sector: u32,
    pub data_sector_count: u32,

    pub entries_per_sec: u32,
    pub bytes_per_sec: u32,
    pub sec_per_clust: u32,
}

// Impl FileSystemInfo
impl FileSystemInfo {
    // New
    pub const fn new() -> Self {
        Self {
            fat_type: FatType::None,
            fat_size: 0,
            reserved_sec_cnt: 0,
            total_sectors: 0,
            cluster_count: 0,
            fat_start_sector: 0,
            fat_end_sector: 0,
            root_cluster: 0,
            root_start_sector: 0,
            root_sector_count: 0,
            data_start_sector: 0,
            data_sector_count: 0,
            entries_per_sec: 0,
            bytes_per_sec: 0,
            sec_per_clust: 0,
        }
    }
}

// Struct Index
#[derive(Clone)]
pub struct DiskIndex {
    pub index: u32,
    pub clust: u32,
    pub sector: u32,
}

// Impl Index
impl DiskIndex {
    // New
    pub const fn new() -> Self {
        DiskIndex {
            index: 0,
            clust: 0,
            sector: 0,
        }
    }
}

// Struct FatDiskio
pub struct FatDiskio {
    starting_lba: u32,
    device: DevFopt,
    info: FileSystemInfo,
}

// Impl FatDiskio
impl FatDiskio {
    // New
    pub const fn new() -> Self {
        Self {
            starting_lba: 0,
            device: DevFopt::new(),
            info: FileSystemInfo::new(),
        }
    }

    // Setup
    pub fn setup(&mut self, disk: &str, starting_lba: u32) -> bool {
        self.starting_lba = starting_lba;

        if self.device.open(disk) {
            if self.check_file_system() {
                return true;
            }
        }

        false
    }

    // Exit
    pub fn exit(&mut self) {
        self.device.close();
    }
}


// Impl FatDiskio
impl FatDiskio {
    // Check file system
    fn check_file_system(&mut self) -> bool {
        // Read dbr sector        
        let mut sector = vec![0u8; 512];
        self.read_sector(&mut sector, 0, 1);

        // Check dbr
        if let Some(dbr) = DBR::from(&sector) {
            // Calc fat size
            if 0 != dbr.bpb.fat_sz_16 {
                self.info.fat_size = dbr.bpb.fat_sz_16 as u32;
            } else {
                self.info.fat_size = dbr.fat32.fat_sz_32;
            }
        
            // Calc total sectors
            if 0 != dbr.bpb.tot_sec_16 {
                self.info.total_sectors = dbr.bpb.tot_sec_16 as u32;
            } else {
                self.info.total_sectors = dbr.bpb.tot_sec_32;
            }

            // Calc rsvd sector count
            self.info.reserved_sec_cnt = dbr.bpb.reserved_sec_cnt as u32;

            // Calc the sector number of start/ended of FAT
            self.info.fat_start_sector = dbr.bpb.reserved_sec_cnt as u32;
            self.info.fat_end_sector = dbr.bpb.reserved_sec_cnt as u32 + (dbr.bpb.num_fats as u32 * self.info.fat_size) - 1;

            // Calc fat12/16 root dir sector
            self.info.root_start_sector = dbr.bpb.reserved_sec_cnt as u32 + (dbr.bpb.num_fats as u32 * self.info.fat_size);
            self.info.root_sector_count = ((dbr.bpb.root_ent_cnt as u32 * DIR_ENTRY_SIZE as u32) + (dbr.bpb.bytes_per_sec as u32 - 1)) / dbr.bpb.bytes_per_sec as u32;
            
            // Calc fat data sector
            self.info.data_start_sector = dbr.bpb.reserved_sec_cnt as u32 + (dbr.bpb.num_fats as u32 * self.info.fat_size) + self.info.root_sector_count;
            self.info.data_sector_count = self.info.total_sectors - (dbr.bpb.reserved_sec_cnt as u32 + (dbr.bpb.num_fats as u32 * self.info.fat_size) - self.info.root_sector_count);

            // Calc counts of clusters
            self.info.cluster_count = self.info.data_sector_count / dbr.bpb.sec_per_clust as u32;

            // Detected fat type
            if self.info.cluster_count < 4085 {
                self.info.fat_type = FatType::Fat12;
            } else if self.info.cluster_count < 65525 {
                self.info.fat_type = FatType::Fat16;
            } else {
                self.info.fat_type = FatType::Fat32;
            }

            // Fat32 root cluster
            self.info.root_cluster = if self.info.fat_type == FatType::Fat32 {
                dbr.fat32.root_clust
            } else {
                0
            };

            // Calc the info data
            self.info.entries_per_sec = dbr.bpb.bytes_per_sec as u32 / DIR_ENTRY_SIZE as u32;
            self.info.bytes_per_sec = dbr.bpb.bytes_per_sec as u32;
            self.info.sec_per_clust = dbr.bpb.sec_per_clust as u32;

            return true;
        }

        false
    }

    // Cluster to sector
    fn cluster_to_sector(&mut self, clust: u32) -> u32 {
        ((clust - 2) * self.info.sec_per_clust) + self.info.data_start_sector
    }

    // Get nexct cluster
    fn get_next_cluster(&mut self, cluster: u32) -> u32 {
        let mut is_eoc = false;
        let mut fat_offset = 0;
        let mut fat_max_offset = 0;
        let mut fat_cluster = 0;

        if self.info.fat_type == FatType::Fat16 {
            fat_offset = cluster as u32 * 2;
            fat_max_offset = self.info.bytes_per_sec / 2;
        } else if self.info.fat_type == FatType::Fat32 {
            fat_offset = cluster as u32 * 4;
            fat_max_offset = self.info.bytes_per_sec / 4;
        }

        let this_fat_sec_num = self.info.fat_start_sector + (fat_offset / self.info.bytes_per_sec);
        let this_fat_ent_offset = cluster % fat_max_offset;

        let mut sec_buff = vec![0u8; self.info.bytes_per_sec as usize];

        self.read_sector(&mut sec_buff, this_fat_sec_num, 1);

        if self.info.fat_type == FatType::Fat16 {
            let ptr = sec_buff.as_ptr() as *const u16;
            fat_cluster = unsafe { *ptr.add(this_fat_ent_offset as usize) } as u32;
            if fat_cluster >= FAT16_EOC_FLAG as u32 {
                is_eoc = true;
            }
        } else if self.info.fat_type == FatType::Fat32 {
            let ptr = sec_buff.as_ptr() as *const u32;
            fat_cluster = unsafe { *ptr.add(this_fat_ent_offset as usize) } & 0x0fffffff;
            if fat_cluster >= FAT32_EOC_FLAG {
                is_eoc = true;
            }
        }

        if is_eoc {
            0
        } else {
            fat_cluster
        }
    }

    // Set next cluster
    fn set_next_cluster(&mut self, mut clust: u32) -> u32 {
        let mut fat_offset = 0;
        let mut fat_max_offset = 0;
        let mut fat_cluster = if clust < 2 { 2 } else { clust };
        let is_alloc_mode = clust < 2;

        if self.info.fat_type == FatType::Fat16 {
            fat_offset = fat_cluster as u32 * 2;
            fat_max_offset = self.info.bytes_per_sec / 2;
        } else if self.info.fat_type == FatType::Fat32 {
            fat_offset = fat_cluster as u32 * 4;
            fat_max_offset = self.info.bytes_per_sec / 4;
        }

        let this_fat_sec_num = self.info.fat_start_sector + (fat_offset / self.info.bytes_per_sec);
        let this_fat_ent_offset = fat_cluster % fat_max_offset;
        let mut next_fat_sec_num = this_fat_sec_num;
        let mut next_fat_ent_offset = this_fat_ent_offset;

        let mut sec_buff = vec![0u8; self.info.bytes_per_sec as usize];

        // Read sector buff
        self.read_sector(&mut sec_buff, this_fat_sec_num, 1);

        // Search next fat clust
        loop {
            next_fat_ent_offset += 1;

            if next_fat_ent_offset >= fat_max_offset {
                next_fat_ent_offset = 0;

                if next_fat_sec_num <= self.info.fat_end_sector {
                    self.read_sector(&mut sec_buff, next_fat_sec_num, 1);
                    next_fat_sec_num += 1;
                } else {
                    drop(sec_buff);
                    return 0;
                }
            }
            
            if self.info.fat_type == FatType::Fat16 {
                let ptr = sec_buff.as_ptr() as *const u16;
                clust = unsafe { *ptr.add(next_fat_ent_offset as usize) } as u32;
            } else if self.info.fat_type == FatType::Fat32 {
                let ptr = sec_buff.as_ptr() as *const u32;
                clust = unsafe { *ptr.add(next_fat_ent_offset as usize) } & 0x0fffffff;
            }

            fat_cluster += 1;

            if clust == 0 {
                break;
            }
        }

        // Set fat clust list table
        if self.info.fat_type == FatType::Fat16 {
            if next_fat_sec_num == this_fat_sec_num {
                if !is_alloc_mode {
                    let ptr = sec_buff.as_mut_ptr() as *mut u16;
                    unsafe { *ptr.add(this_fat_ent_offset as usize) = fat_cluster as u16 };
                }
                
                let ptr = sec_buff.as_mut_ptr() as *mut u16;
                unsafe { *ptr.add(next_fat_ent_offset as usize) = FAT16_EOC_FLAG };
                self.write_sector(&sec_buff, next_fat_sec_num, 1);
            } else {
                let ptr = sec_buff.as_mut_ptr() as *mut u16;
                unsafe { *ptr.add(next_fat_ent_offset as usize) = FAT16_EOC_FLAG };
                self.write_sector(&sec_buff, next_fat_sec_num, 1);

                if !is_alloc_mode {
                    self.read_sector(&mut sec_buff, this_fat_sec_num, 1);
                    let ptr = sec_buff.as_mut_ptr() as *mut u16;
                    unsafe { *ptr.add(this_fat_ent_offset as usize) = fat_cluster as u16 };
                    self.write_sector(&sec_buff, this_fat_sec_num, 1);
                }
            }
        } else if self.info.fat_type == FatType::Fat32 {
            if next_fat_sec_num == this_fat_sec_num {
                if !is_alloc_mode {
                    let ptr = sec_buff.as_mut_ptr() as *mut u32;
                    unsafe { 
                        *ptr.add(this_fat_ent_offset as usize) &= 0xf0000000;
                        *ptr.add(this_fat_ent_offset as usize) |= fat_cluster;
                    }
                }

                let ptr = sec_buff.as_mut_ptr() as *mut u32;
                unsafe { 
                    *ptr.add(next_fat_ent_offset as usize) &= 0xf0000000;
                    *ptr.add(next_fat_ent_offset as usize) |= FAT32_EOC_FLAG;
                }
                self.write_sector(&sec_buff, next_fat_sec_num, 1);
            } else {
                let ptr = sec_buff.as_mut_ptr() as *mut u32;
                unsafe { 
                    *ptr.add(next_fat_ent_offset as usize) &= 0xf0000000;
                    *ptr.add(next_fat_ent_offset as usize) |= FAT32_EOC_FLAG;
                }
                self.write_sector(&sec_buff, next_fat_sec_num, 1);
                
                if !is_alloc_mode {
                    self.read_sector(&mut sec_buff, this_fat_sec_num, 1);
                    let ptr = sec_buff.as_mut_ptr() as *mut u32;
                    unsafe { 
                        *ptr.add(this_fat_ent_offset as usize) &= 0xf0000000;
                        *ptr.add(this_fat_ent_offset as usize) |= fat_cluster;
                    }
                    self.write_sector(&sec_buff, this_fat_sec_num, 1);
                }
            }
        }

        drop(sec_buff);

        fat_cluster
    }

    // Clear prev cluster
    fn clear_prev_cluster(&mut self, clust: u32) -> u32 {
        let mut fat_offset = 0;
        let mut fat_max_offset = 0;
        let mut fat_cluster: u32;

        if self.info.fat_type == FatType::Fat16 {
            fat_offset = clust as u32 * 2;
            fat_max_offset = self.info.bytes_per_sec / 2;
        } else if self.info.fat_type == FatType::Fat32 {
            fat_offset = clust as u32 * 4;
            fat_max_offset = self.info.bytes_per_sec / 4;
        }

        let this_fat_sec_num = self.info.fat_start_sector + (fat_offset / self.info.bytes_per_sec);
        let this_fat_ent_offset = clust % fat_max_offset;
        let mut prev_fat_sec_num = this_fat_sec_num;
        let mut prev_fat_ent_offset = this_fat_ent_offset;

        let mut sec_buff = vec![0u8; self.info.bytes_per_sec as usize];

        // Read sector buff
        self.read_sector(&mut sec_buff, this_fat_sec_num, 1);

        // Search prev fat clust
        loop {
            if prev_fat_ent_offset == 0 {
                prev_fat_ent_offset = fat_max_offset;

                if prev_fat_sec_num >= self.info.fat_start_sector {
                    self.read_sector(&mut sec_buff, prev_fat_sec_num, 1);
                    prev_fat_sec_num -= 1;
                } else {
                    drop(sec_buff);
                    return 0;
                }
            } else {
                prev_fat_ent_offset -= 1;
            }
            
            if self.info.fat_type == FatType::Fat16 {
                let ptr = sec_buff.as_ptr() as *const u16;
                fat_cluster = unsafe { *ptr.add(prev_fat_ent_offset as usize) } as u32;
                if fat_cluster == clust {
                    break;
                }
            } else if self.info.fat_type == FatType::Fat32 {
                let ptr = sec_buff.as_ptr() as *const u32;
                fat_cluster = unsafe { *ptr.add(prev_fat_ent_offset as usize) } & 0x0fffffff;
                if fat_cluster == clust {
                    break;
                }
            }
        }

        // Set fat clust list table
        if self.info.fat_type == FatType::Fat16 {
            if prev_fat_sec_num == this_fat_sec_num {
                let ptr = sec_buff.as_mut_ptr() as *mut u16;
                unsafe { 
                    *ptr.add(this_fat_ent_offset as usize) = 0;
                    *ptr.add(prev_fat_ent_offset as usize) = FAT16_EOC_FLAG;
                }
                self.write_sector(&sec_buff, prev_fat_sec_num, 1);
            } else {
                let ptr = sec_buff.as_mut_ptr() as *mut u16;
                unsafe { *ptr.add(prev_fat_ent_offset as usize) = FAT16_EOC_FLAG };
                self.write_sector(&sec_buff, prev_fat_sec_num, 1);

                self.read_sector(&mut sec_buff, this_fat_sec_num, 1);
                let ptr = sec_buff.as_mut_ptr() as *mut u16;
                unsafe { *ptr.add(this_fat_ent_offset as usize) = 0 };
                self.write_sector(&sec_buff, this_fat_sec_num, 1);
            }
        } else if self.info.fat_type == FatType::Fat32 {
            if prev_fat_sec_num == this_fat_sec_num {
                let ptr = sec_buff.as_mut_ptr() as *mut u32;
                unsafe { 
                    *ptr.add(this_fat_ent_offset as usize) &= 0xf0000000;
                    *ptr.add(this_fat_ent_offset as usize) |= 0;
                    *ptr.add(prev_fat_ent_offset as usize) &= 0xf0000000;
                    *ptr.add(prev_fat_ent_offset as usize) |= FAT32_EOC_FLAG;
                }
                self.write_sector(&sec_buff, prev_fat_sec_num, 1);
            } else {
                let ptr = sec_buff.as_mut_ptr() as *mut u32;
                unsafe { 
                    *ptr.add(prev_fat_ent_offset as usize) &= 0xf0000000;
                    *ptr.add(prev_fat_ent_offset as usize) |= FAT32_EOC_FLAG;
                }
                self.write_sector(&sec_buff, prev_fat_sec_num, 1);
                
                self.read_sector(&mut sec_buff, this_fat_sec_num, 1);
                let ptr = sec_buff.as_mut_ptr() as *mut u32;
                unsafe { 
                    *ptr.add(this_fat_ent_offset as usize) &= 0xf0000000;
                    *ptr.add(this_fat_ent_offset as usize) |= 0;
                }
                self.write_sector(&sec_buff, this_fat_sec_num, 1);
            }
        }

        drop(sec_buff);

        fat_cluster
    }
}

// Impl FatDiskio
impl FatDiskio {
    // Get frist index
    pub fn get_frist_index(&mut self, fst_clust: u32) -> DiskIndex {
        let mut index = DiskIndex::new();

        if fst_clust < 2 {
            if self.info.fat_type == FatType::Fat16 {
                index.clust  = 0;
                index.sector = self.info.root_start_sector;
            } else if self.info.fat_type == FatType::Fat32 {
                index.clust  = self.info.root_cluster;
                index.sector = self.cluster_to_sector(index.clust);
            }
        } else {
            index.clust  = fst_clust;
            index.sector = self.cluster_to_sector(fst_clust);
        }

        index
    }

    // Get next index
    pub fn get_next_index(&mut self, mut index: DiskIndex) -> DiskIndex {
       // FAT16 root dir
        if index.clust < 2 {
            let dir_ended_sec = self.info.root_start_sector + self.info.root_sector_count;
            index.sector = if index.sector + 1 < dir_ended_sec {
                index.sector + 1
            } else {
                0
            };
        }
        // FAT data dir
        else {
            index.sector += 1;

            if (index.sector - self.cluster_to_sector(index.clust)) >= self.info.sec_per_clust {
                index.clust = self.get_next_cluster(index.clust);
                if index.clust != 0 {
                    index.sector = self.cluster_to_sector(index.clust);
                } else {
                    index.sector = 0;
                }
            }
        }

        index
    }
}

// Impl FatDiskio
impl FatDiskio {
    // Write sector
    pub fn write_sector(&mut self, data: &[u8], sector: u32, sec_size: u32) -> u32 {
        self.device.write(data, sec_size as usize, (sector + self.starting_lba) as usize) as u32
    }

    // Read Sector
    pub fn read_sector(&mut self, data: &mut [u8], sector: u32, sec_size: u32) -> u32 {
        self.device.read(data, sec_size as usize, (sector + self.starting_lba)as usize) as u32
    }
}

// Impl FatDiskio
impl FatDiskio {
    // Write cluster
    pub fn write_cluster(&mut self, data: &[u8], clust: u32, clust_size: u32) -> u32 {
        let mut cluster = clust;
        let bytes_per_sec = self.info.bytes_per_sec;
        let sec_per_clust = self.info.sec_per_clust;

        for i in 0..clust_size {
            let sector = self.cluster_to_sector(cluster);
            let offset = i * bytes_per_sec * sec_per_clust;

            if sec_per_clust != self.write_sector(&data[offset as usize ..], sector, sec_per_clust) {
                return i + 1;
            }

            if clust_size > 1 {
                let next_clust = self.set_next_cluster(cluster);
                if next_clust == 0 {
                    return i + 1;
                }
                cluster = next_clust;
            }
        }

        clust_size
    }

    // Read cluster
    pub fn read_cluster(&mut self, data: &mut [u8], clust: u32, clust_size: u32) -> u32 {
        let mut cluster = clust;
        let bytes_per_sec = self.info.bytes_per_sec;
        let sec_per_clust = self.info.sec_per_clust;

        for i in 0..clust_size {
            let sector = self.cluster_to_sector(cluster);
            let offset = i * bytes_per_sec * sec_per_clust;
            
            if sec_per_clust != self.read_sector(&mut data[offset as usize ..], sector, sec_per_clust) {
                return i + 1;
            }

            if clust_size > 1 {
                let next_clust = self.get_next_cluster(cluster);
                if next_clust == 0 {
                    return i + 1;
                }
                cluster = next_clust;
            }
        }

        clust_size
    }

    // Clear cluster
    pub fn clear_cluster(&mut self, clust: u32, clust_size: u32) -> u32 {
        let mut cluster = clust;
        let bytes_per_sec = self.info.bytes_per_sec;
        let sec_per_clust = self.info.sec_per_clust;
        let zero = vec![0u8; (bytes_per_sec * sec_per_clust) as usize];

        for i in 0..clust_size {
            let sector = self.cluster_to_sector(cluster);

            if sec_per_clust != self.write_sector(&zero, sector, sec_per_clust) {
                return i + 1;
            }

            if clust_size > 1 {
                let next_cluster = self.get_next_cluster(cluster);
                if next_cluster == 0 { 
                    return i + 1; 
                }
                cluster = next_cluster;
            }
        }

        clust_size
    }

    // Alloc cluster
    pub fn alloc_cluster(&mut self, clust_size: u32) -> u32 {
        let mut cluster = 0;
        let mut first_cluster = 0;
        for _ in 0..clust_size {
            cluster = self.set_next_cluster(cluster);
            if first_cluster == 0 { 
                first_cluster = cluster; 
            }
            if cluster == 0 { 
                return 0; 
            }
            self.clear_cluster(cluster, 1);
        }
        first_cluster
    }

    // Free cluster
    pub fn free_cluster(&mut self, clust: u32, clust_size: u32) -> u32 {
        let mut cluster = clust;
        for _ in 0..clust_size {
            cluster = self.clear_prev_cluster(cluster);
            if cluster == 0 { 
                return 0; 
            }
        }
        cluster
    }
}

// Impl FatDiskio
impl FatDiskio {
    // Get info
    pub fn get_info(&mut self) -> &mut FileSystemInfo {
        &mut self.info
    }
}
