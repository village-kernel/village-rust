###########################################################################
# Makefile
# The Top Makefile of village rust project
#
# $Copyright: Copyright (C) village
############################################################################

#######################################
# rust env
#######################################
TARGET             := ia32legacy
PROFILE            := debug
Q                  := @

#######################################
# paths
#######################################
BUILD_OUT_DIR      := $(PWD)/build
CRATE_OUT_DIR       = $(BUILD_OUT_DIR)/$(CRATE)/${TARGET}/${PROFILE}
ROOTFS_OUT_DIR      = $(BUILD_OUT_DIR)/village_rootfs
INSTALL_DIR        := '/Volumes/VILLAGE OS'


#######################################
# default action: build all
#######################################
all:
	$(Q)$(MAKE) boot
	$(Q)$(MAKE) kernel
	$(Q)$(MAKE) osbone
	$(Q)$(MAKE) osImage
	$(Q)$(MAKE) rootfs


#######################################
# build the crate
#######################################
%.cargo: FORCE
	$(Q)cd $(@:.cargo=) && \
	BUILD=$(BUILD_OUT_DIR)/$(@:.cargo=) TARGET=$(TARGET) PROFILE=$(PROFILE) \
	cargo make || exit 1;


#######################################
# build the bootloader
#######################################
boot: CRATE = village_boot
boot:
	$(Q)$(MAKE) $(CRATE)/$(TARGET).cargo
	$(Q)cp -rf $(CRATE_OUT_DIR)/*.{elf,hex,bin} $(BUILD_OUT_DIR)/


#######################################
# build the kernel
#######################################
kernel: CRATE = village_kernel
kernel:
	$(Q)$(MAKE) $(CRATE).cargo
	$(Q)cp -rf $(CRATE_OUT_DIR)/*.{elf,hex,bin} $(BUILD_OUT_DIR)/


#######################################
# build the osbone
#######################################
osbone:
	@$(foreach CRATE, $(shell find village_osbone -name Makefile.toml -exec dirname {} \; ), \
		$(MAKE) $(CRATE).cargo;                                                              \
		LIBS_DIR=$$(echo '$(CRATE)' | sed 's:^village_osbone/\(.*\)/[^/]*$$:\1:');           \
		EXEC_DIR=$$(echo '$(CRATE)' | sed 's:^village_osbone/::');                           \
		if [ "$$LIBS_DIR" = "libraries" ]; then                                              \
			mkdir -p  $(ROOTFS_OUT_DIR)/$$LIBS_DIR/;                                         \
			cp    -rf $(CRATE_OUT_DIR)/*.so                                                  \
				      $(ROOTFS_OUT_DIR)/$$LIBS_DIR/ 2>/dev/null || :;                        \
		else                                                                                 \
			mkdir -p  $(ROOTFS_OUT_DIR)/$$EXEC_DIR/;                                         \
			cp    -rf $(CRATE_OUT_DIR)/*.{melf,mhex,mbin,mod,elf,hex,bin,exec}               \
			          $(ROOTFS_OUT_DIR)/$$EXEC_DIR/ 2>/dev/null || :;                        \
		fi;                                                                                  \
	)


#######################################
# generate the os image
#######################################
osImage:
	$(Q)dd if=/dev/zero                           of=$(BUILD_OUT_DIR)/village_os.img bs=512 count=2880
	$(Q)dd if=$(BUILD_OUT_DIR)/village_boot.bin   of=$(BUILD_OUT_DIR)/village_os.img bs=512 seek=0 conv=notrunc
	$(Q)dd if=$(BUILD_OUT_DIR)/village_kernel.bin of=$(BUILD_OUT_DIR)/village_os.img bs=512 seek=1 conv=notrunc


#######################################
# copy to rootfs
#######################################
rootfs:
	$(Q)cp -rf $(ROOTFS_OUT_DIR)/* $(INSTALL_DIR)/
	$(Q)cp -rf rootfs.img          $(BUILD_OUT_DIR)/village_fs.img


#######################################
# clean up
#######################################
clean:
	$(Q)rm -rf $(BUILD_OUT_DIR)


#######################################
# Phony rules
#######################################
PHONY += FORCE
FORCE:

PHONY += all boot kernel osbone osImage clean
.PHONY: $(PHONY)
