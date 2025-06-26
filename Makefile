###########################################################################
# Makefile
# The Top Makefile of village rust project
#
# $Copyright: Copyright (C) village
############################################################################
VERSION        = 0.1.0


#######################################
# paths
#######################################
WORKSPACE     := $(PWD)
BUILD_DIR     := $(WORKSPACE)/build
BOOT_DIR      := $(BUILD_DIR)/village_boot/ia32legacy/debug
KERNEL_DIR    := $(BUILD_DIR)/village_kernel/ia32legacy/debug


#######################################
# tasks
#######################################
# default action: build all
all:
	$(Q)$(MAKE) boot
	$(Q)$(MAKE) kernel
	$(Q)$(MAKE) osImage
	$(Q)$(MAKE) rootfs


#######################################
# build the bootloader
#######################################
boot:
	$(Q)$(MAKE) -C village_boot/legacy/ia32bios BUILD_DIR=$(BOOT_DIR)
	$(Q)cp -rf $(BOOT_DIR)/village_boot $(BUILD_DIR)/village_boot.elf 
	$(Q)i686-elf-objcopy -O binary -S $(BUILD_DIR)/village_boot.elf $(BUILD_DIR)/village_boot.bin


#######################################
# build the kernel
#######################################
kernel:
	cd village_kernel && cargo build && cd ..
	$(Q)cp -rf $(KERNEL_DIR)/village_kernel $(BUILD_DIR)/village_kernel.elf
	$(Q)i686-elf-objcopy -O binary -S $(BUILD_DIR)/village_kernel.elf $(BUILD_DIR)/village_kernel.bin


#######################################
# generate the os image
#######################################
osImage:
	$(Q)dd if=/dev/zero                       of=$(BUILD_DIR)/village_os.img bs=512 count=2880
	$(Q)dd if=$(BUILD_DIR)/village_boot.bin   of=$(BUILD_DIR)/village_os.img bs=512 seek=0 conv=notrunc
	$(Q)dd if=$(BUILD_DIR)/village_kernel.bin of=$(BUILD_DIR)/village_os.img bs=512 seek=1 conv=notrunc


#######################################
# copy to rootfs
#######################################
rootfs:
	$(Q)cp -rf rootfs.img               $(BUILD_DIR)/village_fs.img


#######################################
# clean up
#######################################
clean:
	$(Q)$(MAKE) -C village_boot/legacy/ia32bios BUILD_DIR=$(BOOT_DIR) clean
	cd village_kernel && cargo clean && cd ..

distclean:
	$(Q)rm -rf $(BUILD_DIR)


#######################################
# Phony rules
#######################################
PHONY += FORCE
FORCE:

PHONY += all boot kernel osImage 
PHONY += clean distclean
.PHONY: $(PHONY)
