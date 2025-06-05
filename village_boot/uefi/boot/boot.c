//###########################################################################
// boot.c
// Low level file that manages kernel entry
//
// $Copyright: Copyright (C) village
//###########################################################################
#include <uefi.h>

EFI_STATUS
efi_main(EFI_HANDLE ImageHandle EFI_UNUSED, EFI_SYSTEM_TABLE *SystemTable)
{
	SystemTable->ConOut->ClearScreen(SystemTable->ConOut);
	SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Hello UEFI!\n");

    while(1) {}
    
	return EFI_SUCCESS;
}
