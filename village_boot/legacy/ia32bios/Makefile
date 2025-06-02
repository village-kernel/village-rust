###########################################################################
# Makefile
# The Makefile of ia32bios boot
#
# $Copyright: Copyright (C) village
############################################################################

#######################################
# paths
#######################################
BUILD_DIR     := ./build


######################################
# ASFLAGS
######################################
ASFLAGS += -g -gdwarf-2 -DDEBUG


######################################
# link script
######################################
LDSCRIPT-BOOT  := -T boot.lds


#######################################
# compiler flags
#######################################
# boot loader ld flags
LDFLAGS  += $(LDSCRIPT-BOOT) -ffreestanding -nostdlib
LDFLAGS  += -Wl,--no-warn-rwx-segment
LDFLAGS  += -Wl,-m,elf_i386


#######################################
# build task
#######################################
all:
	$(Q)mkdir -p $(BUILD_DIR)
	$(Q)i686-elf-gcc -x assembler-with-cpp -c $(ASFLAGS) boot.s -o $(BUILD_DIR)/boot.o
	$(Q)i686-elf-gcc $(LDFLAGS) $(BUILD_DIR)/boot.o -o $(BUILD_DIR)/village_boot

clean:
	$(Q)rm $(BUILD_DIR)/*.o $(BUILD_DIR)/village_boot
