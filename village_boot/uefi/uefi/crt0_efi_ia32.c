//###########################################################################
// crt0_efi_ia32.c
// Low level file that manages efi entry
//
// $Copyright: Copyright (C) village
//###########################################################################
#include "uefi.h"


/// @brief ELF32 dyn type
#define DT_NULL           0
#define DT_RELA           7
#define DT_REL            17
#define DT_RELSZ          18
#define DT_RELENT         19


/// @brief ELF32 rel type
#define R_386_NONE        0
#define R_386_RELATIVE    8


/// @brief Elf32_Dyn
typedef struct 
{
	int32_t tag;
	union
	{
		uint32_t val;
		uint32_t ptr;
	};
} Elf32_Dyn;


/// @brief Elf32_Rel
typedef struct 
{
	uint32_t offset;
	uint8_t  type;
	uint8_t  symbol;
	uint16_t reversed;
} Elf32_Rel;


/// @brief _start
/// @param ImageHandle 
/// @param SystemTable 
EFI_STATUS _start(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE* SystemTable);


/// @brief efi app entry
/// @param ImageHandle 
/// @param SystemTable 
/// @return 
EFI_STATUS efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable);


/// @brief __relocate
/// @param imagebase 
/// @param dynamic 
EFI_STATUS __relocate(uint32_t imagebase, Elf32_Dyn* dynamic)
{
	uint32_t  relsz  = 0;
	uint32_t  relent = 0;
	Elf32_Rel *rel   = NULL;
	uint32_t  *addr  = NULL;

	for (int i = 0; dynamic[i].tag != DT_NULL; ++i)
	{
		switch (dynamic[i].tag)
		{
			case DT_REL:
				rel = (Elf32_Rel*)((uint32_t)dynamic[i].ptr + imagebase);
				break;

			case DT_RELSZ:
				relsz = dynamic[i].val;
				break;

			case DT_RELENT:
				relent = dynamic[i].val;
				break;

			case DT_RELA:
				break;

			default:
				break;
		}
	}

    if (!rel && relent == 0) return EFI_SUCCESS;
	if (!rel || relent == 0) return EFI_LOAD_ERROR;

	while (relsz > 0)
	{
		switch (rel->type)
		{
			case R_386_NONE:
				break;

			case R_386_RELATIVE:
				addr   = (uint32_t*)(imagebase + rel->offset);
				*addr += imagebase;
				break;

			default:
				break;
		}
		rel    = (Elf32_Rel*)((uint8_t*)rel + relent);
		relsz -= relent;
	}

	return EFI_SUCCESS;
}


/// @brief execute init_arrary
/// @param  
void __init_array(void)
{
	extern void (*__init_array_start []) (void);
	extern void (*__init_array_end   []) (void);

	int count = __init_array_end - __init_array_start;
	for (int i = 0; i < count; i++)
		__init_array_start[i]();
}


/// @brief execute fini_arrary
/// @param  
void __fini_array(void)
{
	extern void (*__fini_array_start []) (void);
	extern void (*__fini_array_end   []) (void);

	int count = __fini_array_end - __fini_array_start;
	
	for (int i = 0; i < count; i++)
	{
		__fini_array_start[i]();
	}
}


/// @brief _start
/// @param  
EFI_STATUS _start(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE* SystemTable)
{
	extern uint32_t   ImageBase;
	extern Elf32_Dyn* _DYNAMIC;
	EFI_STATUS status;
	
	status = __relocate(ImageBase, _DYNAMIC);
	if (EFI_SUCCESS != status) return status;

	__init_array();
	status = efi_main(ImageHandle, SystemTable);
	__fini_array();

	return status;
}


/// @brief hand-craft a dummy .reloc section so EFI knows it's a relocatable executable
static const uint32_t dummy_reloc[] __attribute__((section (".reloc#"), used)) = { 0, 12, 0 };
