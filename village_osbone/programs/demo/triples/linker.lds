ENTRY(_start)

PHDRS {
    load    PT_LOAD    FLAGS(4);     /* R   */
    text    PT_LOAD    FLAGS(5);     /* R E */
    data    PT_LOAD    FLAGS(6);     /* RW  */
    dynamic PT_DYNAMIC FLAGS(6);     /* RW  */
    relro   PT_GNU_RELRO;
}

SECTIONS {
    /* Elf entry sections */
    .entry : { KEEP(*(.entry)) } :load

    /* Dynamic linking sections */
    .dynsym   : { *(.dynsym)   } :load
    .dynstr   : { *(.dynstr)   } :load
    .gnu.hash : { *(.gnu.hash) } :load
    .hash     : { *(.hash)     } :load
    .rel.dyn  : { *(.rel.dyn)  } :load
    .rel.plt  : { *(.rel.plt)  } :load

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

    /* PLT */
    .plt : {
        *(.plt)
        *(.plt.*)
    } :text

    /* Bss sections */
    .bss : {
        _sbss = .;
        *(.dynbss .bss .bss.* .gnu.linkonce.b.*)
        _ebss = .;
    } :data

    /* Data sections */
    .data.rel.ro : {
        *(.data.rel.ro .data.rel.ro.*)
    } :data

    .dynamic : { *(.dynamic) } :data :dynamic
    .got     : { *(.got)     } :data
    .got.plt : { *(.got.plt) } :data

    /* Relro padding */
    .relro_padding : {
        . = ALIGN(4);
        *(.relro_padding)
        . = ALIGN(4);
    } :data

    /* Standard sections */
    /DISCARD/ : {
        *(.note.GNU-stack)
        *(.gnu_debuglink)
        *(.gnu.lto_*)
    }
}
