//###########################################################################
// crt0_kernel.c
// Low level file that manages kernel entry
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::arch::naked_asm;

// extern main
unsafe extern "Rust" { unsafe fn main(); }

// irq handler
#[linkage = "weak"]
#[unsafe(naked)]
//#[unsafe(no_mangle)]
pub unsafe extern "C" fn irq_handler() {
    naked_asm!(
        "jmp .",
        options(att_syntax)
    );
}

// stub handler
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stub_handler() {
    naked_asm!(
        // push data into esp
        "pushl %ds",
        "pushl %es",
        "pushl %fs",
        "pushl %gs",
        "pushal",
        
        // sets the segments
        "movw $0x10, %ax",
        "movw %ax, %ds",
        "movw %ax, %es",
        "movw %ax, %fs",
        "movw %ax, %gs",
        
        // call irq_handler(%esp)
        "pushl %esp",
        "call irq_handler",
        "addl $4, %esp",
        
        // pop all data back
        "popal",
        "popl %gs",
        "popl %fs",
        "popl %es",
        "popl %ds",
        
        // skip irq and errcode
        "addl $8, %esp",
        "sti",
        "iret",
        options(att_syntax)
    );
}

// macro to generate exception handlers with error code
macro_rules! interrupt_handler {
    ($name:ident, $irq:expr, $err:expr) => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name() {
            naked_asm!(
                "cli",
                concat!("push $", $err),
                concat!("push $", $irq),
                "jmp stub_handler",
                options(att_syntax)
            );
        }
    };
}

// generate all the interrupt handlers
interrupt_handler!(division_by_zero_handler, 0, 0);
interrupt_handler!(debug_handler, 1, 0);
interrupt_handler!(non_maskable_interrupt_handler, 2, 0);
interrupt_handler!(breakpoint_handler, 3, 0);
interrupt_handler!(into_detected_overflow_handler, 4, 0);
interrupt_handler!(out_of_bounds_handler, 5, 0);
interrupt_handler!(invalid_opcode_handler, 6, 0);
interrupt_handler!(no_coprocessor_handler, 7, 0);
interrupt_handler!(double_fault_handler, 8, 0);
interrupt_handler!(coprocessor_segment_overrun_handler, 9, 0);
interrupt_handler!(bad_tss_handler, 10, 0);
interrupt_handler!(segment_not_present_handler, 11, 0);
interrupt_handler!(stack_fault_handler, 12, 0);
interrupt_handler!(general_protection_fault_handler, 13, 0);
interrupt_handler!(page_fault_handler, 14, 0);
interrupt_handler!(unknown_interrupt_handler, 15, 0);
interrupt_handler!(coprocessor_fault_handler, 16, 0);
interrupt_handler!(alignment_check_handler, 17, 0);
interrupt_handler!(machine_check_handler, 18, 0);
interrupt_handler!(reserved_in_19_handler, 19, 0);
interrupt_handler!(reserved_in_20_handler, 20, 0);
interrupt_handler!(reserved_in_21_handler, 21, 0);
interrupt_handler!(reserved_in_22_handler, 22, 0);
interrupt_handler!(reserved_in_23_handler, 23, 0);
interrupt_handler!(reserved_in_24_handler, 24, 0);
interrupt_handler!(reserved_in_25_handler, 25, 0);
interrupt_handler!(reserved_in_26_handler, 26, 0);
interrupt_handler!(reserved_in_27_handler, 27, 0);
interrupt_handler!(reserved_in_28_handler, 28, 0);
interrupt_handler!(reserved_in_29_handler, 29, 0);
interrupt_handler!(svc_handler, 30, 0);
interrupt_handler!(pendsv_handler, 31, 0);
interrupt_handler!(systick_handler, 32, 0);
interrupt_handler!(keyboard_controller_handler, 33, 1);
interrupt_handler!(reserved_ex_2_handler, 34, 2);
interrupt_handler!(serial_port_com2_handler, 35, 3);
interrupt_handler!(serial_port_com1_handler, 36, 4);
interrupt_handler!(line_print_terminal2_handler, 37, 5);
interrupt_handler!(floppy_controller_handler, 38, 6);
interrupt_handler!(line_print_terminal1_handler, 39, 7);
interrupt_handler!(rtc_timer_handler, 40, 8);
interrupt_handler!(x86_assembly_acpi_handler, 41, 9);
interrupt_handler!(reserved_ex_11_handler, 42, 10);
interrupt_handler!(reserved_ex_12_handler, 43, 11);
interrupt_handler!(mouse_controller_handler, 44, 12);
interrupt_handler!(math_coprocessor_handler, 45, 13);
interrupt_handler!(ata_channel1_handler, 46, 14);
interrupt_handler!(ata_channel2_handler, 47, 15);

#[used]
#[unsafe(link_section = ".isr_vector")]
pub static G_PFN_VECTORS: [unsafe extern "C" fn(); 49] = [
    _start,
    division_by_zero_handler,
    debug_handler,
    non_maskable_interrupt_handler,
    breakpoint_handler,
    into_detected_overflow_handler,
    out_of_bounds_handler,
    invalid_opcode_handler,
    no_coprocessor_handler,
    double_fault_handler,
    coprocessor_segment_overrun_handler,
    bad_tss_handler,
    segment_not_present_handler,
    stack_fault_handler,
    general_protection_fault_handler,
    page_fault_handler,
    unknown_interrupt_handler,
    coprocessor_fault_handler,
    alignment_check_handler,
    machine_check_handler,
    reserved_in_19_handler,
    reserved_in_20_handler,
    reserved_in_21_handler,
    reserved_in_22_handler,
    reserved_in_23_handler,
    reserved_in_24_handler,
    reserved_in_25_handler,
    reserved_in_26_handler,
    reserved_in_27_handler,
    reserved_in_28_handler,
    reserved_in_29_handler,
    svc_handler,
    pendsv_handler,
    systick_handler,
    keyboard_controller_handler,
    reserved_ex_2_handler,
    serial_port_com2_handler,
    serial_port_com1_handler,
    line_print_terminal2_handler,
    floppy_controller_handler,
    line_print_terminal1_handler,
    rtc_timer_handler,
    x86_assembly_acpi_handler,
    reserved_ex_11_handler,
    reserved_ex_12_handler,
    mouse_controller_handler,
    math_coprocessor_handler,
    ata_channel1_handler,
    ata_channel2_handler,
];

// init data bss
#[unsafe(no_mangle)]
pub extern "C" fn __init_data_bss() {
    unsafe extern "C" {
        unsafe static mut _sidata: u8;
        unsafe static mut _sdata: u8;
        unsafe static mut _edata: u8;
        unsafe static mut _sbss: u8;
        unsafe static mut _ebss: u8;
    }

    unsafe {
        // Copy data segment initializers from disk to SRAM
        let mut src = &raw const _sidata as *const u8;
        let mut dst = &raw mut _sdata as *mut u8;
        while dst < &raw mut _edata as *mut u8 {
            *dst = *src;
            src = src.add(1);
            dst = dst.add(1);
        }

        // Zero fill the bss segment
        let mut dst = &raw mut _sbss as *mut u8;
        while dst < &raw mut _ebss as *mut u8 {
            *dst = 0;
            dst = dst.add(1);
        }
    }
}

// preinit array
#[unsafe(no_mangle)]
pub extern "C" fn __preinit_array() {
    unsafe extern "C" {
        unsafe static __preinit_array_start: [Option<unsafe extern "C" fn()>; 0];
        unsafe static __preinit_array_end: [Option<unsafe extern "C" fn()>; 0];
    }

    unsafe {
        let start = &__preinit_array_start as *const _ as *const unsafe extern "C" fn();
        let end = &__preinit_array_end as *const _ as *const unsafe extern "C" fn();
        let count = (end as usize - start as usize) / core::mem::size_of::<unsafe extern "C" fn()>();

        for i in 0..count {
            let func = start.add(i);
            (*func)();
        }
    }
}

// init array
#[unsafe(no_mangle)]
pub extern "C" fn __init_array() {
    unsafe extern "C" {
        unsafe static __init_array_start: [Option<unsafe extern "C" fn()>; 0];
        unsafe static __init_array_end: [Option<unsafe extern "C" fn()>; 0];
    }

    unsafe {
        let start = &__init_array_start as *const _ as *const unsafe extern "C" fn();
        let end = &__init_array_end as *const _ as *const unsafe extern "C" fn();
        let count = (end as usize - start as usize) / core::mem::size_of::<unsafe extern "C" fn()>();

        for i in 0..count {
            let func = start.add(i);
            (*func)();
        }
    }
}

// fini array
#[unsafe(no_mangle)]
pub extern "C" fn __fini_array() {
    unsafe extern "C" {
        unsafe static __fini_array_start: [Option<unsafe extern "C" fn()>; 0];
        unsafe static __fini_array_end: [Option<unsafe extern "C" fn()>; 0];
    }

    unsafe {
        let start = &__fini_array_start as *const _ as *const unsafe extern "C" fn();
        let end = &__fini_array_end as *const _ as *const unsafe extern "C" fn();
        let count = (end as usize - start as usize) / core::mem::size_of::<unsafe extern "C" fn()>();

        for i in 0..count {
            let func = start.add(i);
            (*func)();
        }
    }
}

// _start
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() {
    __init_data_bss();

    __preinit_array();

    __init_array();

    unsafe { main() };

    __fini_array();

    loop {}
}
