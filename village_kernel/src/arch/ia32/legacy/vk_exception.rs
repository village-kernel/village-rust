//###########################################################################
// vk_exception.rs
// The specific implementation of functions related to exception
//
// $Copyright: Copyright (C) village
//###########################################################################
use super::vk_registers::Registers;
use crate::traits::vk_callback::Callback;
use crate::vendor::ia32legacy::core::i686::*;
use crate::village::kernel;
use crate::debug_error;
use core::arch::asm;

// Constant members
pub const ISR_NUM: usize = 48;
pub const RSVD_ISR_SIZE: usize = 0;
const IDT_ENTRIES: u32 = 48;
const KERNEL_CODE_SEGMENT: u16 = 8;

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
struct IdtGate {
    low_offset: u16,
    sel: u16,
    rsvd: u8,
    flags: u8,
    high_offset: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
struct IdtRegister {
    limit: u16,
    base: u32,
}

// Struct village exception
pub struct VillageException {
    idt: [IdtGate; ISR_NUM],
    idt_reg: IdtRegister,
}

// Impl village exception
impl VillageException {
    // New
    pub const fn new() -> Self {
        VillageException {
            idt: [IdtGate {
                low_offset: 0,
                sel: 0,
                rsvd: 0,
                flags: 0,
                high_offset: 0,
            }; ISR_NUM],
            idt_reg: IdtRegister { limit: 0, base: 0 },
        }
    }
}

// Impl village exception
impl VillageException {
    // Setup
    pub fn setup(&mut self) {
        // Symbol defined in the linker script
        unsafe extern "C" {
            unsafe static _svector: [unsafe extern "C" fn(); 0];
            unsafe static _evector: [unsafe extern "C" fn(); 0];
        }

        // Calculate the size of isr vector
        let count = unsafe { _evector.as_ptr() as usize - _svector.as_ptr() as usize };
        let count = count / core::mem::size_of::<unsafe extern "C" fn()>();

        // Set interrupt handler
        for i in 1..count {
            unsafe {
                let handler = *(_svector.as_ptr().add(i)) as usize;
                // The first func is _start(), we don't need
                self.install(i - 1, handler);
            }
        }

        // Install handlers
        self.install_handlers();

        // Remap the PIC
        self.remap_pic();

        // Set IDT
        self.set_idt();
    }

    // Exit
    pub fn exit(&mut self) {
        self.uninstall_handlers();
    }

    // Install
    pub fn install(&mut self, irq: usize, handler: usize) {
        let gate = &mut self.idt[irq];
        gate.low_offset = (handler & 0xffff) as u16;
        gate.high_offset = (handler >> 16) as u16;
        gate.sel = KERNEL_CODE_SEGMENT;
        gate.flags = 0x8E;
    }

    // Get pic irq register
    pub fn get_pic_irq_reg(&self, ocw3: u8) -> u16 {
        port_byte_out(PIC1_CMD, ocw3);
        port_byte_out(PIC2_CMD, ocw3);
        (port_byte_in(PIC2_CMD) as u16) << 8 | port_byte_in(PIC1_CMD) as u16
    }

    // Get pic irr
    pub fn get_pic_irr(&self) -> u16 {
        self.get_pic_irq_reg(PIC_READ_IRR)
    }

    // Get pic isr
    pub fn get_pic_isr(&self) -> u16 {
        self.get_pic_irq_reg(PIC_READ_ISR)
    }

    // Set idt
    fn set_idt(&mut self) {
        self.idt_reg.base = self.idt.as_ptr() as u32;
        self.idt_reg.limit = (IDT_ENTRIES * core::mem::size_of::<IdtGate>() as u32) as u16 - 1;

        unsafe {
            asm!("lidt [{}]", in(reg) &self.idt_reg as *const _ as u32);
        }
    }

    // Remap pic
    fn remap_pic(&self) {
        // Save masks
        let a1 = port_byte_in(PIC1_DATA);
        let a2 = port_byte_in(PIC2_DATA);

        // Starts the initialization sequence (in cascade mode)
        port_byte_out(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
        port_byte_out(PIC2_CMD, ICW1_INIT | ICW1_ICW4);

        // ICW2: Master PIC vector offset
        port_byte_out(PIC1_DATA, 0x20);
        // ICW2: Slave PIC vector offset
        port_byte_out(PIC2_DATA, 0x28);

        // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
        port_byte_out(PIC1_DATA, 0x04);
        // ICW3: tell Slave PIC its cascade identity (0000 0010)
        port_byte_out(PIC2_DATA, 0x02);

        // ICW4: have the PICs use 8086 mode (and not 8080 mode)
        port_byte_out(PIC1_DATA, ICW4_8086);
        port_byte_out(PIC2_DATA, ICW4_8086);

        // Restore saved masks
        port_byte_out(PIC1_DATA, a1);
        port_byte_out(PIC2_DATA, a2);
    }

    // Install handlers
    fn install_handlers(&mut self) {
        macro_rules! install_handler {
            ($irq:expr, $handler:expr) => {
                kernel()
                    .interrupt()
                    .add_isr_cb($irq, Callback::new($handler as u32));
            };
        }

        install_handler!(0, Self::division_by_zero_handler);
        install_handler!(1, Self::debug_handler);
        install_handler!(2, Self::non_maskable_interrupt_handler);
        install_handler!(3, Self::breakpoint_handler);
        install_handler!(4, Self::into_detected_overflow_handler);
        install_handler!(5, Self::out_of_bounds_handler);
        install_handler!(6, Self::invalid_opcode_handler);
        install_handler!(7, Self::no_coprocessor_handler);
        install_handler!(8, Self::double_fault_handler);
        install_handler!(9, Self::coprocessor_segment_overrun_handler);
        install_handler!(10, Self::bad_tss_handler);
        install_handler!(11, Self::segment_not_present_handler);
        install_handler!(12, Self::stack_fault_handler);
        install_handler!(13, Self::general_protection_fault_handler);
        install_handler!(14, Self::page_fault_handler);
        install_handler!(15, Self::unknown_interrupt_handler);
        install_handler!(16, Self::coprocessor_fault_handler);
        install_handler!(17, Self::alignment_check_handler);
        install_handler!(18, Self::machine_check_handler);
    }

    // Uninstall handlers
    fn uninstall_handlers(&mut self) {
        macro_rules! uninstall_handler {
            ($irq:expr, $handler:expr) => {
                kernel()
                    .interrupt()
                    .del_isr_cb($irq, Callback::new($handler as u32));
            };
        }

        uninstall_handler!(0, Self::division_by_zero_handler);
        uninstall_handler!(1, Self::debug_handler);
        uninstall_handler!(2, Self::non_maskable_interrupt_handler);
        uninstall_handler!(3, Self::breakpoint_handler);
        uninstall_handler!(4, Self::into_detected_overflow_handler);
        uninstall_handler!(5, Self::out_of_bounds_handler);
        uninstall_handler!(6, Self::invalid_opcode_handler);
        uninstall_handler!(7, Self::no_coprocessor_handler);
        uninstall_handler!(8, Self::double_fault_handler);
        uninstall_handler!(9, Self::coprocessor_segment_overrun_handler);
        uninstall_handler!(10, Self::bad_tss_handler);
        uninstall_handler!(11, Self::segment_not_present_handler);
        uninstall_handler!(12, Self::stack_fault_handler);
        uninstall_handler!(13, Self::general_protection_fault_handler);
        uninstall_handler!(14, Self::page_fault_handler);
        uninstall_handler!(15, Self::unknown_interrupt_handler);
        uninstall_handler!(16, Self::coprocessor_fault_handler);
        uninstall_handler!(17, Self::alignment_check_handler);
        uninstall_handler!(18, Self::machine_check_handler);
    }

    // Division by zero handler
    fn division_by_zero_handler() {
        debug_error!("Division By Zero");
        loop {}
    }

    // Debug handler
    fn debug_handler() {
        debug_error!("Debug");
        loop {}
    }

    // Non maskable interrupthandler
    fn non_maskable_interrupt_handler() {
        debug_error!("Non Maskable Interrupt");
        loop {}
    }

    // Breakpoint handler
    fn breakpoint_handler() {
        debug_error!("Breakpoint");
        loop {}
    }

    // Into detected overflow handler
    fn into_detected_overflow_handler() {
        debug_error!("Into Detected Overflow");
        loop {}
    }

    // Out of bounds handler
    fn out_of_bounds_handler() {
        debug_error!("Out Of Bounds");
        loop {}
    }

    // Invalid opcode handler
    fn invalid_opcode_handler() {
        debug_error!("Invalid Opcode");
        loop {}
    }

    // No coprocessor handler
    fn no_coprocessor_handler() {
        debug_error!("No Coprocessor");
        loop {}
    }

    // Double fault handler
    fn double_fault_handler() {
        debug_error!("Double Fault");
        loop {}
    }

    // Coprocessor segment overrun handler
    fn coprocessor_segment_overrun_handler() {
        debug_error!("Coprocessor Segment Overrun");
        loop {}
    }

    // Bad tss handler
    fn bad_tss_handler() {
        debug_error!("Bad TSS");
        loop {}
    }

    // Segment not present handler
    fn segment_not_present_handler() {
        debug_error!("Segment Not Present");
        loop {}
    }

    // Stack fault handler
    fn stack_fault_handler() {
        debug_error!("Stack Fault");
        loop {}
    }

    // General protection fault handler
    fn general_protection_fault_handler() {
        debug_error!("General Protection Fault");
        loop {}
    }

    // Page fault handler
    fn page_fault_handler() {
        debug_error!("Page Fault");
        loop {}
    }

    // Unknown interrupt handler
    fn unknown_interrupt_handler() {
        debug_error!("Unknown Interrupt");
        loop {}
    }

    // Coprocessor fault handler
    fn coprocessor_fault_handler() {
        debug_error!("Coprocessor Fault");
        loop {}
    }

    // Alignment check handler
    fn alignment_check_handler() {
        debug_error!("Alignment Check");
        loop {}
    }

    // Machine check handler
    fn machine_check_handler() {
        debug_error!("Machine Check");
        loop {}
    }
}

// Stacked info
fn stacked_info(regs: &Registers) {
    debug_error!("Exception_Handler:");

    debug_error!("irq:    0x{:08x}", regs.irq);
    debug_error!("err:    0x{:08x}", regs.err);
    debug_error!("psp:    0x{:08x}", regs.psp);

    debug_error!("CPU:");
    debug_error!("eax:    0x{:08x}", regs.eax);
    debug_error!("ecx:    0x{:08x}", regs.ecx);
    debug_error!("edx:    0x{:08x}", regs.edx);
    debug_error!("ebx:    0x{:08x}", regs.ebx);
    debug_error!("ebx:    0x{:08x}", regs.ebx);
    debug_error!("esp:    0x{:08x}", regs.esp);
    debug_error!("ebp:    0x{:08x}", regs.ebp);
    debug_error!("esi:    0x{:08x}", regs.esi);
    debug_error!("edi:    0x{:08x}", regs.edi);
    debug_error!("eip:    0x{:08x}", regs.eip);
    debug_error!("eflags: 0x{:08x}", regs.eflags);

    debug_error!("Segs:");
    debug_error!("cs:     0x{:08x}", regs.cs);
    debug_error!("ss:     0x{:08x}", regs.ss);
    debug_error!("ds:     0x{:08x}", regs.ds);
    debug_error!("es:     0x{:08x}", regs.es);
    debug_error!("fs:     0x{:08x}", regs.fs);
    debug_error!("gs:     0x{:08x}", regs.gs);
}

// IRQ handler
#[unsafe(no_mangle)]
pub unsafe extern "C" fn irq_handler(regs: *const Registers) {
    let regs = unsafe { core::ptr::read(regs) };

    // Send an EOI to the PICs
    if (32..=47).contains(&regs.irq) {
        if regs.irq >= 40 {
            port_byte_out(PIC2_CMD, PIC_EOI); //slave
        }
        if regs.irq >= 32 {
            port_byte_out(PIC1_CMD, PIC_EOI); //master
        }
    }

    // Output stacked info
    if (0..=18).contains(&regs.irq) {
        stacked_info(&regs);
    }

    // Handle the interrupt in a more modular way
    kernel().interrupt().handler(regs.irq as isize);
}
