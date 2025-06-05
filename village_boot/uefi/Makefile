###########################################################################
# Makefile
# The Makefile of uefi boot
#
# $Copyright: Copyright (C) village
############################################################################

######################################
# paths
######################################
BUILD_DIR     := ./build


######################################
# link script
######################################
LDSCRIPT      := -T ./uefi/elf_ia32_efi.lds


#######################################
# compiler flags
#######################################
# c flags
CFLAGS        += -m32 -mno-sse -mno-mmx -DCONFIG_32_BIT
CFLAGS        += -Wno-error=pragmas -mno-red-zone -mno-avx -fPIE
CFLAGS        += -g -O2 -Wall -Wextra -Wno-pointer-sign
CFLAGS        += -funsigned-char -fshort-wchar -fno-strict-aliasing
CFLAGS        += -ffreestanding -fno-stack-protector -fno-stack-check
CFLAGS        += -fno-merge-all-constants -std=c11
CFLAGS        += -maccumulate-outgoing-args

# ld flags
LDFLAGS       += -nostdlib -z nocombreloc -shared -Bsymbolic
LDFLAGS       += --warn-common --no-undefined --fatal-warnings --build-id=sha1
LDFLAGS       += -melf_i386
LDFLAGS       += $(LDSCRIPT)

# efi flags
EFIFLAGS      += -j .text -j .sdata -j .data -j .dynamic -j .rodata -j .rel
EFIFLAGS      += -j .rela -j .rel.* -j .rela.* -j .rel* -j .rela*
EFIFLAGS      += -j .areloc -j .reloc --target efi-app-ia32

# includes
INCLUDES      += -I./uefi/

#######################################
# build task
#######################################
all:
	$(Q)mkdir -p $(BUILD_DIR)
	$(Q)x86_64-elf-gcc $(CFLAGS) $(INCLUDES) -c ./uefi/crt0_efi_ia32.c -o $(BUILD_DIR)/crt0_efi_ia32.o
	$(Q)x86_64-elf-gcc $(CFLAGS) $(INCLUDES) -c ./boot/boot.c -o $(BUILD_DIR)/boot.o
	$(Q)x86_64-elf-ld $(LDFLAGS) $(BUILD_DIR)/crt0_efi_ia32.o $(BUILD_DIR)/boot.o -o $(BUILD_DIR)/boot.so
	$(Q)x86_64-elf-objcopy $(EFIFLAGS) $(BUILD_DIR)/boot.so $(BUILD_DIR)/boot.efi

test:
	$(Q)cp ./ovmf/bios32.bin bios32.bin
	$(Q)mkdir -p ./efi/boot
	$(Q)cp ./build/boot.efi ./efi/boot/bootia32.efi
	$(Q)qemu-system-i386 -bios bios32.bin -m 64 -hda fat:rw:./

clean:
	$(Q)rm $(BUILD_DIR)/*
