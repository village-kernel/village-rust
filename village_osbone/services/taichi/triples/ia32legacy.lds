/* Script for -pie */
ENTRY(_start)

SECTIONS
{
	_IMGOFFS = .;
	
	.entry :
	{
		. = ALIGN(4);
		KEEP(*(.entry))
		. = ALIGN(4);
	}

	.text :
	{
		. = ALIGN(4);
		*(.text)
		*(.text*)

		KEEP (*(.init))
		KEEP (*(.fini))

		. = ALIGN(4);
		 _etext = .; 
	}

	.preinit_array :
	{
		PROVIDE_HIDDEN (__preinit_array_start = .);
		KEEP (*(SORT(.preinit_array.*)))
		KEEP (*(.preinit_array*))
		PROVIDE_HIDDEN (__preinit_array_end = .);
	}

	.init_array :
	{
		PROVIDE_HIDDEN (__init_array_start = .);
		KEEP (*(SORT(.init_array.*)))
		KEEP (*(.init_array*))
		KEEP (*(.ctors*))
		PROVIDE_HIDDEN (__init_array_end = .);
	}

	.fini_array :
	{
		PROVIDE_HIDDEN (__fini_array_start = .);
		KEEP (*(SORT(.fini_array.*)))
		KEEP (*(.fini_array*))
		KEEP (*(.dtors*))
		PROVIDE_HIDDEN (__fini_array_end = .);
	}

	_sidata = LOADADDR(.data);

	.data :
	{
		. = ALIGN(4);
		_sdata = .;  
		*(.data .data.* .gnu.linkonce.d.*)
        *(.got .got.*)
        *(.got.plt .got.plt.*)

		. = ALIGN(4);
		_sbss = .; 
		*(.dynbss)
		*(.bss .bss.* .gnu.linkonce.b.*)
		*(COMMON)
		. = ALIGN(4);
		_ebss = .; 

		. = ALIGN(4);
		_edata = .; 
	}

	.rodata :
	{
		. = ALIGN(4);
		*(.rodata)
		*(.rodata*)
		*(.gnu.linkonce.r.*)
		. = ALIGN(4);
	}

	/*/DISCARD/ :
	{
		libc.a ( * )
		libm.a ( * )
		libgcc.a ( * )
	}*/
}
