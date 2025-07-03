//###########################################################################
// vk_filesystem.rs
// The specific implementation of functions related to file system
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use crate::village::kernel;
use crate::traits::vk_kernel::FileSystem;
use crate::traits::vk_driver::DriverID;
use crate::traits::vk_filesys::{FileVol, FileSys};
use crate::traits::vk_linkedlist::LinkedList;
use crate::misc::fopts::vk_dev_fopt::DevFopt;

// Enum BootIndicator
pub enum BootIndicator {
    NotBootable = 0x00,
    Bootable = 0x80,
}

// Enum PartitionType
enum PartitionType {
    None = 0,
    MBR,
    GPT,
}

// Struct MBR partition record
struct MBRPartition {
    pub boot_indicator: u8,
    pub starting_head: u8,
    pub starting_chs: u16,
    pub os_indicator: u8,
    pub ending_head: u8,
    pub ending_chs: u16,
    pub starting_lba: u32,
    pub size_in_lba: u32,
}

impl MBRPartition {
    // New
    const fn new() -> Self {
        Self {
            boot_indicator: 0,
            starting_head: 0,
            starting_chs: 0,
            os_indicator: 0,
            ending_head: 0,
            ending_chs: 0,
            starting_lba: 0,
            size_in_lba: 0,
        }
    }

    // From
    const fn from(data: &[u8]) -> Self {
        if data.len() < 16 {
            return Self::new();
        }

        let boot_indicator = data[0];
        let starting_head = data[1];
        let starting_chs = u16::from_le_bytes([data[2], data[3]]);
        let os_indicator = data[4];
        let ending_head = data[5];
        let ending_chs = u16::from_le_bytes([data[6], data[7]]);
        let starting_lba = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        let size_in_lba = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
        
        Self {
            boot_indicator,
            starting_head,
            starting_chs,
            os_indicator,
            ending_head,
            ending_chs,
            starting_lba,
            size_in_lba,
        }
    }

    // Check is vaild
    pub fn is_vaild(&mut self) -> bool {
        if self.os_indicator != 0 && self.size_in_lba != 0 {
            return self.boot_indicator != 0 || 
                self.starting_head != 0 ||
                self.starting_chs != 0 ||
                self.ending_head != 0 ||
                self.ending_chs != 0 ||
                self.starting_lba != 0
        }
        false
    }
}

// Struct MBR partition table
struct MBR {
    pub boot: Vec<u8>,
    pub partitions: [MBRPartition; 4],
    pub magic: u16,
}

impl MBR {
    // New
    pub const fn new() -> Self {
        Self {
            boot: Vec::new(),
            partitions: [const { MBRPartition::new() }; 4],
            magic: 0,
        }
    }

    // From
    pub fn from(data: &[u8]) -> Option<Self> {
        if data.len() < 512 {
            return None;
        }

        let mut mbr = Self::new();

        // Copy boot code
        mbr.boot = vec![0u8; 446];
        mbr.boot.copy_from_slice(&data[0..446]);

        // Parser partitions
        mbr.partitions[0] = MBRPartition::from(&data[446..462]);
        mbr.partitions[1] = MBRPartition::from(&data[462..478]);
        mbr.partitions[2] = MBRPartition::from(&data[478..494]);
        mbr.partitions[3] = MBRPartition::from(&data[494..510]);
 
        // Get magic and check
        mbr.magic = u16::from_le_bytes([data[510], data[511]]);
        
        if mbr.magic != 0xAA55 {
            return None;
        }

        Some(mbr)
    }
}

// Struct GPT partition record
struct GPTPartition {
    pub partition_type_guid: [u8; 16],
    pub unique_partition_guid: [u8; 16],
    pub starting_lba: u64,
    pub ending_lba: u64,
    pub attributes: u64,
    pub partition_name: [u16; 36],
    pub reserved: Vec<u8>,
}

// Impl GPTPartition
impl GPTPartition {
    // New
    pub const fn new() -> Self {
        Self {
            partition_type_guid: [0u8; 16],
            unique_partition_guid: [0u8; 16],
            starting_lba: 0,
            ending_lba: 0,
            attributes: 0,
            partition_name: [0u16; 36],
            reserved: Vec::new(),
        }
    }

    // From
    pub fn from(data: &[u8]) -> Option<Self> {
        if data.len() < 512 {
            return None;
        }

        let mut partition = Self::new();

        // Parser guid
        partition.partition_type_guid.copy_from_slice(&data[0..16]);
        partition.unique_partition_guid.copy_from_slice(&data[16..32]);

        // Parser lba
        partition.starting_lba = u64::from_le_bytes(<[u8; 8]>::try_from(&data[32..40]).ok()?);
        partition.ending_lba = u64::from_le_bytes(<[u8; 8]>::try_from(&data[40..48]).ok()?);
        partition.attributes = u64::from_le_bytes(<[u8; 8]>::try_from(&data[48..56]).ok()?);

        // Parser name
        for i in 0..36 {
            let idx = 56 + i * 2;
            if idx + 1 >= data.len() {
                break;
            }
            partition.partition_name[i] = u16::from_le_bytes([data[idx], data[idx + 1]]);
        }

        // Reserved
        partition.reserved = vec![0u8; 384];
        partition.reserved.copy_from_slice(&data[128..512]);

        // Check
        if Self::is_empty(&partition) {
            return None;
        }

        Some(partition)
    }

    // Check is empty
    pub fn is_empty(&self) -> bool {
        self.partition_type_guid == [0u8; 16]
    }
}

// Struct GPT partition record
struct GPT {
    pub signature: [u8; 8],
    pub revision: u32,
    pub header_size: u32,
    pub header_crc32: u32,
    pub reserved0: u32,
    pub my_lba: u64,
    pub alternate_lba: u64,
    pub first_usable_lba: u64,
    pub last_usable_lba: u64,
    pub disk_guid: [u8; 16],
    pub partition_entry_lba: u64,
    pub number_of_partition_entries: u32,
    pub size_of_partition_entry: u32,
    pub partition_entry_array_crc32: u32,
    pub reserved1: Vec<u8>,
}

// Impl GPT
impl GPT {
    // New
    pub const fn new() -> Self {
        Self {
            signature: [0u8; 8],
            revision: 0,
            header_size: 0,
            header_crc32: 0,
            reserved0: 0,
            my_lba: 0,
            alternate_lba: 0,
            first_usable_lba: 0,
            last_usable_lba: 0,
            disk_guid: [0u8; 16],
            partition_entry_lba: 0,
            number_of_partition_entries: 0,
            size_of_partition_entry: 0,
            partition_entry_array_crc32: 0,
            reserved1: Vec::new(),
        }
    }

    // From
    pub fn from(data: &[u8]) -> Option<Self> {
        if data.len() < 512 {
            return None;
        }

        let mut gpt = Self::new();

        // Get signature
        gpt.signature.copy_from_slice(&data[0..8]);

        // Copy header
        gpt.revision = u32::from_le_bytes(<[u8; 4]>::try_from(&data[8..12]).ok()?);
        gpt.header_size = u32::from_le_bytes(<[u8; 4]>::try_from(&data[12..16]).ok()?);
        gpt.header_crc32 = u32::from_le_bytes(<[u8; 4]>::try_from(&data[16..20]).ok()?);
        gpt.reserved0 = u32::from_le_bytes(<[u8; 4]>::try_from(&data[20..24]).ok()?);
        
        // Copy lba
        gpt.my_lba = u64::from_le_bytes(<[u8; 8]>::try_from(&data[24..32]).ok()?);
        gpt.alternate_lba = u64::from_le_bytes(<[u8; 8]>::try_from(&data[32..40]).ok()?);
        gpt.first_usable_lba = u64::from_le_bytes(<[u8; 8]>::try_from(&data[40..48]).ok()?);
        gpt.last_usable_lba = u64::from_le_bytes(<[u8; 8]>::try_from(&data[48..56]).ok()?);
        
        // Copy guid
        gpt.disk_guid.copy_from_slice(&data[56..72]);
        
        // Copy entry
        gpt.partition_entry_lba = u64::from_le_bytes(<[u8; 8]>::try_from(&data[72..80]).ok()?);
        gpt.number_of_partition_entries = u32::from_le_bytes(<[u8; 4]>::try_from(&data[80..84]).ok()?);
        gpt.size_of_partition_entry = u32::from_le_bytes(<[u8; 4]>::try_from(&data[84..88]).ok()?);
        gpt.partition_entry_array_crc32 = u32::from_le_bytes(<[u8; 4]>::try_from(&data[88..92]).ok()?);
        
        // Copy reserved
        gpt.reserved1 = vec![0u8; 420];
        gpt.reserved1.copy_from_slice(&data[92..512]);

        // Check is valid
        if !Self::is_valid(&gpt) {
            return None
        }

        Some(gpt)
    }

    // Check is valid
    pub fn is_valid(&self) -> bool {
        &self.signature == b"EFI PART" &&
        self.header_size >= 92 &&
        self.size_of_partition_entry >= 128 &&
        self.number_of_partition_entries > 0
    }
}

// Struct MountNode
pub struct MountNode {
    pub target: String,
    pub source: String,
    pub access: u16,
}

// Impl MountNode
impl MountNode {
    // New
    pub const fn new() -> Self {
        Self {
            target: String::new(),
            source: String::new(),
            access: 0755,
        }
    }

    // From
    pub fn from(target: &str, source: &str, access: u16) -> Self {
        Self {
            target: target.to_string(),
            source: source.to_string(),
            access,
        }
    }
}

// Struct DiskMedia
struct DiskMedia {
    typ: PartitionType,
    name: String,
    vols: LinkedList<Box<dyn FileVol>>,
}

// Impl DiskMedia
impl DiskMedia {
    pub const fn new() -> Self {
        Self {
            typ: PartitionType::None,
            name: String::new(),
            vols: LinkedList::new(),
        }
    }

    // Get volume
    pub fn get_volume(&mut self, path: &str) -> Option<&mut Box<dyn FileVol>> {
        for volume in self.vols.iter_mut() {
            if volume.get_mount_path() == path {
                return Some(volume);
            }
        }
        None
    }
}

// Struct concrete file system
pub struct ConcreteFileSystem {
    filesyses: LinkedList<Box<dyn FileSys>>,
    medias: LinkedList<DiskMedia>,
    mounts: LinkedList<MountNode>,
}

// Impl concrete file system
impl ConcreteFileSystem {
    // New
    pub const fn new() -> Self {
        Self {
            filesyses: LinkedList::new(),
            medias: LinkedList::new(),
            mounts: LinkedList::new(),
        }
    }
}

// Impl concrete file system
impl ConcreteFileSystem {
    // Setup
    pub fn setup(&mut self) {
        // Initialize all hard disk
        for block in kernel().device().get_drivers().iter_mut() {
            if block.info().get_id() == DriverID::Block {
                self.mount_hard_drive(block.info().get_name());
            }
        }

        // Mount root node
        if self.mount_root_node() {
            kernel().debug().info("File system setup completed!");
        } else {
            kernel().debug().error("File system setup failed!");
        }
    }

    // Exit
    pub fn exit(&mut self) {
        // Create an disks list
        let mut disks = LinkedList::<String>::new();

        // Clone all the disk name
        for media in self.medias.iter_mut() {
            disks.add(media.name.clone());
        }

        // Exit hard drive
        for disk in disks.iter_mut() {
            self.unmount_hard_drive(&disk);
        }

        // Clear medias
        self.medias.clear();

        // Clear file system
        self.filesyses.clear();

        // Clear mount node
        self.mounts.clear();
    }
}

// Impl concrete file system
impl ConcreteFileSystem {
    // Mount root node
    fn mount_root_node(&mut self) -> bool {
        // Create root mount node
        let root = MountNode::from("/", "/media/VILLAGE OS", 0755);

        // Try to mount root node
        if self.mount_system_node(root) {
            return true;
        }

        // Output info
        kernel().debug().error("Mount root node failed, 'VILLAGE OS' not found");
        false
    }

    // Mount system node
    fn mount_system_node(&mut self, mount: MountNode) -> bool {
        for media in self.medias.iter_mut() {
            for volume in media.vols.iter_mut() {
                if volume.get_mount_path() == mount.source {
                    self.mounts.add(mount);
                    return true;
                }
            }
        }
        false
    }

    // Setup volume
    fn setup_volume(&mut self, media: &mut DiskMedia, starting_lba: u32) {
        for filesys in self.filesyses.iter_mut() {
            // Create new volume
            let mut volume = filesys.create_volume();
            
            // Setup volume
            if volume.setup(&media.name, starting_lba) {
                let mount_path = &format!("/media/{}", volume.get_name());
                volume.set_mount_path(mount_path);
                media.vols.add(volume);
                return;
            }
        }
    }
}

// Impl file system for concrete file system
impl FileSystem for ConcreteFileSystem {
    // Register fs
    fn register_fs(&mut self, fs: Box<dyn FileSys>) {
        self.filesyses.add(fs);
    }

    // Unregister fs
    fn unregister_fs(&mut self, name: &str) {
        self.filesyses.retain_mut(|fs| {
            !(fs.info().get_name() == name)
        });
    }

    // Mount hard drive
    fn mount_hard_drive(&mut self, disk: &str) -> bool {
        kernel().debug().info(&format!("Setup the hard drive {}", disk));

        // Create an devstream object
        let mut device = DevFopt::new();

        // Open the disk device
        if !device.open(disk) {
            kernel().debug().error(&format!("hard drive {} open failed", disk));
            return false;
        }

        // Read the master boot record from sector 0
        let mut sector = vec![0u8; 512];
        device.read(&mut sector, 1, 0);

        // Parser master boot record
        if let Some(mut mbr) = MBR::from(&sector) {
            // Create an new disk media
            let mut media = DiskMedia::new();

            // Set the media name
            media.name = disk.to_string();

            // Media with partition table
            if mbr.partitions[0].is_vaild() {
                // Set the media for GPT partition format
                if mbr.partitions[0].os_indicator == 0xee {
                    // Set media partition type as GPT
                    media.typ = PartitionType::GPT;

                    // Read GPT header for staring lba sector
                    device.read(&mut sector, 1, mbr.partitions[0].starting_lba as usize);

                    // Parser gpt header
                    if let Some(gpt) = GPT::from(&sector) {
                        // Calculate the size of volume
                        let size = gpt.number_of_partition_entries / gpt.size_of_partition_entry;

                        // Setup partitions
                        for i in 0..size {
                            // Read partition record
                            device.read(&mut sector, 1, (gpt.partition_entry_lba as usize).wrapping_add(i as usize));

                            // Create GPT partition table object
                            if let Some(partition) = GPTPartition::from(&sector) {
                                // Setup GPT volume
                                self.setup_volume(&mut media, partition.starting_lba as u32);
                            }
                        }
                    }
                }
                // Set the media for MBR partition format
                else {
                    // Set media partition type as MBR
                    media.typ = PartitionType::MBR;
                    
                    // Setup MBR partitions
                    for i in 0..4 {
                        if mbr.partitions[i].is_vaild() {
                            self.setup_volume(&mut media, mbr.partitions[i].starting_lba);
                        }
                    }
                }
            } else {
                // Setup single partition
                self.setup_volume(&mut media, 0);
            }

            // Close device
            device.close();

            // Add to medias list
            self.medias.add(media);
        } else {
            kernel().debug().error("Not a vaild disk");
            device.close();
            return false;
        }
        
        true
    }

    // Unmount hard drive
    fn unmount_hard_drive(&mut self, disk: &str) -> bool {
        let mut result = false;

        // Get and remove media
        self.medias.retain_mut(|media| {
            // Get the disk media
            if media.name == disk {
                // Exit volumes
                for volume in media.vols.iter_mut() {
                    volume.exit();
                }

                // Clear volumes
                media.vols.clear();

                result = true;
            }
            !result
        });

        result
    }

    // Get volume
    fn get_volume(&mut self, name: &str) -> Option<&mut Box<dyn FileVol>> {
        if let Some(mount) = self.mounts.iter_mut().find(|mount| name.starts_with(&mount.target)) {
            for media in self.medias.iter_mut() {
                let volume = media.get_volume(&mount.source);
                if volume.is_some() {
                    return volume;
                }
            }
        }
        None
    }
}
