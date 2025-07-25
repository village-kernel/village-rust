OUTPUT_FORMAT("elf32-i386", "elf32-i386","elf32-i386")
OUTPUT_ARCH(i386)
ENTRY(_start)

_estack     = 0x2000000;
_rsvd_heap  = 0x0000400;
_rsvd_stack = 0x0000400;

MEMORY
{
	RAM  (xrw) : ORIGIN = 0x100000, LENGTH = 10M
}

PHDRS {
    load  PT_LOAD  FLAGS(4);     /* R   */
    text  PT_LOAD  FLAGS(5);     /* R E */
    data  PT_LOAD  FLAGS(6);     /* RW  */
}

SECTIONS
{
    /* Program start address */
    . = ORIGIN(RAM);

    /* ISR vector sections */
    .isr_vector : {
        _svector = .;
        KEEP(*(.isr_vector))
        _evector = .;
    } :load

    /* Preinit array linking section */
    .preinit_array : {
        __perinit_array_start = .;
        *(.preinit_array .preinit_array.*)
        __preinit_array_end = .;
    } :load

    /* Init array linking section */
    .init_array : {
        __init_array_start = .;
        *(.ctors .init_array .init_array.*)
        __init_array_end = .;
    } :load

    /* Fini array linking section */
    .fini_array : {
        __fini_array_start = .;
        *(.dtors .fini_array .fini_array.*)
        __fini_array_end = .;
    } :load

    /* Read-only data */
    .rodata : {
        *(.rodata .rodata.* .gnu.linkonce.r.*)
    } :load

    /* Executable code */
    .text : {
        *(.text .text.* .gnu.linkonce.t.*)
    } :text

    /* Data sections */
    _sidata = LOADADDR(.data);
    .data : {
        _sdata = .;  
        *(.data .data.*)
        _edata = .; 
    } :data

    /* Bss sections */
    .bss : {
        _sbss = .;
        *(.dynbss .bss .bss.* .gnu.linkonce.b.*)
        _ebss = .;
    } :data

    /* Standard sections */
    /DISCARD/ : {
        *(.note.GNU-stack)
        *(.gnu_debuglink)
        *(.gnu.lto_*)
    }
}
