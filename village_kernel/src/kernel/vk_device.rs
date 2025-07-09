//###########################################################################
// vk_device.rs
// The specific implementation of functions related to device
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_driver::DriverID;
use crate::traits::vk_driver::DriverWrapper;
use crate::traits::vk_driver::PlatDevWrapper;
use crate::traits::vk_driver::PlatDrvWrapper;
use crate::traits::vk_kernel::Device;
use crate::traits::vk_linkedlist::LinkedList;
use crate::village::kernel;

// Struct village device
pub struct VillageDevice {
    is_runtime: bool,
    base_devs: LinkedList<DriverWrapper>,
    plat_devs: LinkedList<PlatDevWrapper>,
    plat_drvs: LinkedList<PlatDrvWrapper>,
}

// Impl village device
impl VillageDevice {
    pub const fn new() -> Self {
        Self {
            is_runtime: false,
            base_devs: LinkedList::new(),
            plat_devs: LinkedList::new(),
            plat_drvs: LinkedList::new(),
        }
    }
}

// Impl village device
impl VillageDevice {
    // Setup
    pub fn setup(&mut self) {
        // Clear the flag
        self.is_runtime = false;

        // Platform probe
        self.platform_probe_all();

        // Set the flag
        self.is_runtime = true;

        //output debug info
        kernel().debug().info("Device setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Platform remove
        self.platform_remove_all();

        // Platform devices
        self.devices_release_all();
    }
}

// Impl village device
impl VillageDevice {
    // Platform match
    fn platform_match(device: &mut PlatDevWrapper, driver: &mut PlatDrvWrapper) -> bool {
        device.name() == driver.name()
    }

    // Platform probe
    fn platform_probe(device: &mut PlatDevWrapper, driver: &mut PlatDrvWrapper) -> bool {
        if Self::platform_match(device, driver) {
            if !device.plat().is_attach() {
                device.config();
                if !driver.probe(device) {
                    device.release();
                }
            }
            return true;
        }
        false
    }

    // Platform remove
    fn platform_remove(device: &mut PlatDevWrapper, driver: &mut PlatDrvWrapper) -> bool {
        if Self::platform_match(device, driver) {
            if device.plat().is_attach() {
                driver.remove(device);
                device.release();
            }
        }
        false
    }

    // Platform device probe
    fn platform_device_probe(
        plat_devs: &mut LinkedList<PlatDevWrapper>,
        driver: &mut PlatDrvWrapper,
    ) {
        for device in plat_devs.iter_mut() {
            Self::platform_probe(&mut *device, driver);
        }
    }

    // Platform device remove
    fn platform_device_remove(
        plat_devs: &mut LinkedList<PlatDevWrapper>,
        driver: &mut PlatDrvWrapper,
    ) {
        for device in plat_devs.iter_mut() {
            Self::platform_remove(&mut *device, driver);
        }
    }

    // Platform driver probe
    fn platform_driver_probe(
        plat_drvs: &mut LinkedList<PlatDrvWrapper>,
        device: &mut PlatDevWrapper,
    ) {
        for driver in plat_drvs.iter_mut() {
            Self::platform_probe(device, &mut *driver);
        }
    }

    // Platform driver remove
    fn platform_driver_remove(
        plat_drvs: &mut LinkedList<PlatDrvWrapper>,
        device: &mut PlatDevWrapper,
    ) {
        for driver in plat_drvs.iter_mut() {
            Self::platform_remove(device, &mut *driver);
        }
    }

    // Platform probe
    fn platform_probe_all(&mut self) {
        for device in self.plat_devs.iter_mut() {
            Self::platform_driver_probe(&mut self.plat_drvs, &mut *device);
        }
    }

    // Platform remove
    fn platform_remove_all(&mut self) {
        for device in self.plat_devs.rev_iter_mut() {
            Self::platform_driver_remove(&mut self.plat_drvs, &mut *device);
        }
    }

    // Devices release
    fn devices_release_all(&mut self) {
        self.base_devs.clear();
        self.plat_devs.clear();
        self.plat_drvs.clear();
    }
}

// Impl deivce for village device
impl Device for VillageDevice {
    // Register driver
    fn register_driver(&mut self, driver: DriverWrapper) {
        if self.is_runtime {
            if driver.id() == DriverID::Block {
                kernel()
                    .filesys()
                    .mount_hard_drive(driver.name());
            } else if driver.id() == DriverID::Input {
                kernel().event().init_input_device(driver.name());
            }
        }
        self.base_devs.push(driver);
    }

    // Unregister driver
    fn unregister_driver(&mut self, name: &str) {
        self.base_devs.retain_mut(|driver| {
            if driver.name() == name {
                if self.is_runtime {
                    if driver.id() == DriverID::Block {
                        kernel()
                            .filesys()
                            .unmount_hard_drive(driver.name());
                    } else if driver.id() == DriverID::Input {
                        kernel().event().exit_input_device(driver.name());
                    }
                }
                false
            } else {
                true
            }
        });
    }

    // Register plat drvier
    fn register_plat_driver(&mut self, mut driver: PlatDrvWrapper) {
        if self.is_runtime {
            Self::platform_device_probe(&mut self.plat_devs, &mut driver);
        }
        self.plat_drvs.push(driver);
    }

    // Unregister plat driver
    fn unregister_plat_driver(&mut self, name: &str) {
        self.plat_drvs.retain_mut(|driver| {
            if driver.name() == name {
                if self.is_runtime {
                    Self::platform_device_remove(&mut self.plat_devs, driver);
                }
                false
            } else {
                true
            }
        });
    }

    // Register plat device
    fn register_plat_device(&mut self, mut device: PlatDevWrapper) {
        if self.is_runtime {
            Self::platform_driver_probe(&mut self.plat_drvs, &mut device);
        }
        self.plat_devs.push(device);
    }

    // Unregister plat device
    fn unregister_plat_device(&mut self, name: &str) {
        self.plat_devs.retain_mut(|device| {
            if device.name() == name {
                if self.is_runtime {
                    Self::platform_driver_remove(&mut self.plat_drvs, device);
                }
                false
            } else {
                true
            }
        });
    }

    // Get driver fopts
    fn get_driver(&mut self, name: &str) -> Option<&mut DriverWrapper> {
        for driver in self.base_devs.iter_mut() {
            if driver.name() == name {
                return Some(driver);
            }
        }
        None
    }

    // Get drivers
    fn get_drivers(&mut self) -> &mut LinkedList<DriverWrapper> {
        &mut self.base_devs
    }
}
