//###########################################################################
// i686.c
// Low level file that manages i686
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::arch::asm;

// PIC defines
pub const PIC1: u16 = 0x20;           // IO base address for master PIC
pub const PIC2: u16 = 0xA0;           // IO base address for slave PIC
pub const PIC1_CMD: u16 = PIC1;
pub const PIC1_DATA: u16 = PIC1 + 1;
pub const PIC2_CMD: u16 = PIC2;
pub const PIC2_DATA: u16 = PIC2 + 1;
pub const PIC_EOI: u8 = 0x20;

pub const PIC_READ_IRR: u8 = 0x0a;     // OCW3 irq ready next CMD read
pub const PIC_READ_ISR: u8 = 0x0b;     // OCW3 irq service next CMD read

// ICW1 defines
pub const ICW1_ICW4: u8 = 0x01;       // Indicates that ICW4 will be present
pub const ICW1_SINGLE: u8 = 0x02;      // Single (cascade) mode
pub const ICW1_INTERVAL4: u8 = 0x04;   // Call address interval 4 (8)
pub const ICW1_LEVEL: u8 = 0x08;       // Level triggered (edge) mode
pub const ICW1_INIT: u8 = 0x10;        // Initialization - required!

// ICW4 defines
pub const ICW4_8086: u8 = 0x01;       // 8086/88 (MCS-80/85) mode
pub const ICW4_AUTO: u8 = 0x02;        // Auto (normal) EOI
pub const ICW4_BUF_SLAVE: u8 = 0x08;   // Buffered mode/slave
pub const ICW4_BUF_MASTER: u8 = 0x0C;  // Buffered mode/master
pub const ICW4_SFNM: u8 = 0x10;        // Special fully nested (not)

// Timer defines
pub const TIMER: u16 = 0x40;
pub const TIMER_CH0: u16 = TIMER + 0;
pub const TIMER_CH1: u16 = TIMER + 1;
pub const TIMER_CH2: u16 = TIMER + 2;
pub const TIMER_CMD: u16 = TIMER + 3;

// Serial port defines
pub const COM1: u16 = 0x3f8;
pub const COM2: u16 = 0x2f8;
pub const COM3: u16 = 0x3e8;
pub const COM4: u16 = 0x2e8;
pub const COM5: u16 = 0x5f8;
pub const COM6: u16 = 0x4f8;
pub const COM7: u16 = 0x5e8;
pub const COM8: u16 = 0x4e8;

pub const COM_DATA_POS: u16 = 0;
pub const COM_IT_ENA_POS: u16 = 1;
pub const COM_LO_BAUD_POS: u16 = 0;
pub const COM_HI_BAUD_POS: u16 = 1;
pub const COM_IT_FIFO_POS: u16 = 2;
pub const COM_LINE_CTRL_POS: u16 = 3;
pub const COM_MODEM_CTRL_POS: u16 = 4;
pub const COM_LINE_STATUS_POS: u16 = 5;
pub const COM_MODEM_STATUS_POS: u16 = 6;
pub const COM_SCRATCH_POS: u16 = 7;

pub const COM1_DATA: u16 = COM1 + 0;
pub const COM1_IT_ENA_REG: u16 = COM1 + 1;
pub const COM1_LO_BAUD_REG: u16 = COM1 + 0;
pub const COM1_HI_BAUD_REG: u16 = COM1 + 1;
pub const COM1_IT_FIFO_REG: u16 = COM1 + 2;
pub const COM1_LINE_CTRL_REG: u16 = COM1 + 3;
pub const COM1_MODEM_CTRL_REG: u16 = COM1 + 4;
pub const COM1_LINE_STATUS_REG: u16 = COM1 + 5;
pub const COM1_MODEM_STATUS_REG: u16 = COM1 + 6;
pub const COM1_SCRATCH_REG: u16 = COM1 + 7;

pub const COM2_DATA: u16 = COM2 + 0;
pub const COM2_IT_ENA_REG: u16 = COM2 + 1;
pub const COM2_LO_BAUD_REG: u16 = COM2 + 0;
pub const COM2_HI_BAUD_REG: u16 = COM2 + 1;
pub const COM2_IT_FIFO_REG: u16 = COM2 + 2;
pub const COM2_LINE_CTRL_REG: u16 = COM2 + 3;
pub const COM2_MODEM_CTRL_REG: u16 = COM2 + 4;
pub const COM2_LINE_STATUS_REG: u16 = COM2 + 5;
pub const COM2_MODEM_STATUS_REG: u16 = COM2 + 6;
pub const COM2_SCRATCH_REG: u16 = COM2 + 7;

pub const COM3_DATA: u16 = COM3 + 0;
pub const COM3_IT_ENA_REG: u16 = COM3 + 1;
pub const COM3_LO_BAUD_REG: u16 = COM3 + 0;
pub const COM3_HI_BAUD_REG: u16 = COM3 + 1;
pub const COM3_IT_FIFO_REG: u16 = COM3 + 2;
pub const COM3_LINE_CTRL_REG: u16 = COM3 + 3;
pub const COM3_MODEM_CTRL_REG: u16 = COM3 + 4;
pub const COM3_LINE_STATUS_REG: u16 = COM3 + 5;
pub const COM3_MODEM_STATUS_REG: u16 = COM3 + 6;
pub const COM3_SCRATCH_REG: u16 = COM3 + 7;

pub const COM4_DATA: u16 = COM4 + 0;
pub const COM4_IT_ENA_REG: u16 = COM4 + 1;
pub const COM4_LO_BAUD_REG: u16 = COM4 + 0;
pub const COM4_HI_BAUD_REG: u16 = COM4 + 1;
pub const COM4_IT_FIFO_REG: u16 = COM4 + 2;
pub const COM4_LINE_CTRL_REG: u16 = COM4 + 3;
pub const COM4_MODEM_CTRL_REG: u16 = COM4 + 4;
pub const COM4_LINE_STATUS_REG: u16 = COM4 + 5;
pub const COM4_MODEM_STATUS_REG: u16 = COM4 + 6;
pub const COM4_SCRATCH_REG: u16 = COM4 + 7;

pub const COM5_DATA: u16 = COM5 + 0;
pub const COM5_IT_ENA_REG: u16 = COM5 + 1;
pub const COM5_LO_BAUD_REG: u16 = COM5 + 0;
pub const COM5_HI_BAUD_REG: u16 = COM5 + 1;
pub const COM5_IT_FIFO_REG: u16 = COM5 + 2;
pub const COM5_LINE_CTRL_REG: u16 = COM5 + 3;
pub const COM5_MODEM_CTRL_REG: u16 = COM5 + 4;
pub const COM5_LINE_STATUS_REG: u16 = COM5 + 5;
pub const COM5_MODEM_STATUS_REG: u16 = COM5 + 6;
pub const COM5_SCRATCH_REG: u16 = COM5 + 7;

pub const COM6_DATA: u16 = COM6 + 0;
pub const COM6_IT_ENA_REG: u16 = COM6 + 1;
pub const COM6_LO_BAUD_REG: u16 = COM6 + 0;
pub const COM6_HI_BAUD_REG: u16 = COM6 + 1;
pub const COM6_IT_FIFO_REG: u16 = COM6 + 2;
pub const COM6_LINE_CTRL_REG: u16 = COM6 + 3;
pub const COM6_MODEM_CTRL_REG: u16 = COM6 + 4;
pub const COM6_LINE_STATUS_REG: u16 = COM6 + 5;
pub const COM6_MODEM_STATUS_REG: u16 = COM6 + 6;
pub const COM6_SCRATCH_REG: u16 = COM6 + 7;

pub const COM7_DATA: u16 = COM7 + 0;
pub const COM7_IT_ENA_REG: u16 = COM7 + 1;
pub const COM7_LO_BAUD_REG: u16 = COM7 + 0;
pub const COM7_HI_BAUD_REG: u16 = COM7 + 1;
pub const COM7_IT_FIFO_REG: u16 = COM7 + 2;
pub const COM7_LINE_CTRL_REG: u16 = COM7 + 3;
pub const COM7_MODEM_CTRL_REG: u16 = COM7 + 4;
pub const COM7_LINE_STATUS_REG: u16 = COM7 + 5;
pub const COM7_MODEM_STATUS_REG: u16 = COM7 + 6;
pub const COM7_SCRATCH_REG: u16 = COM7 + 7;

pub const COM8_DATA: u16 = COM8 + 0;
pub const COM8_IT_ENA_REG: u16 = COM8 + 1;
pub const COM8_LO_BAUD_REG: u16 = COM8 + 0;
pub const COM8_HI_BAUD_REG: u16 = COM8 + 1;
pub const COM8_IT_FIFO_REG: u16 = COM8 + 2;
pub const COM8_LINE_CTRL_REG: u16 = COM8 + 3;
pub const COM8_MODEM_CTRL_REG: u16 = COM8 + 4;
pub const COM8_LINE_STATUS_REG: u16 = COM8 + 5;
pub const COM8_MODEM_STATUS_REG: u16 = COM8 + 6;
pub const COM8_SCRATCH_REG: u16 = COM8 + 7;

// COM line control register bits
pub const COM_LINE_CTRL_DATA_BITS_POS: u8 = 0;
pub const COM_LINE_CTRL_DATA_BITS_MSK: u8 = 0x3 << COM_LINE_CTRL_DATA_BITS_POS;
pub const COM_LINE_CTRL_DATA_BITS_0: u8 = 0x1 << COM_LINE_CTRL_DATA_BITS_POS;
pub const COM_LINE_CTRL_DATA_BITS_1: u8 = 0x2 << COM_LINE_CTRL_DATA_BITS_POS;

pub const COM_LINE_CTRL_STOP_BITS_POS: u8 = 2;
pub const COM_LINE_CTRL_STOP_BITS_MSK: u8 = 0x1 << COM_LINE_CTRL_STOP_BITS_POS;

pub const COM_LINE_CTRL_PARITY_POS: u8 = 3;
pub const COM_LINE_CTRL_PARITY_MSK: u8 = 0x7 << COM_LINE_CTRL_PARITY_POS;
pub const COM_LINE_CTRL_PARITY_0: u8 = 0x1 << COM_LINE_CTRL_PARITY_POS;
pub const COM_LINE_CTRL_PARITY_1: u8 = 0x2 << COM_LINE_CTRL_PARITY_POS;
pub const COM_LINE_CTRL_PARITY_2: u8 = 0x4 << COM_LINE_CTRL_PARITY_POS;

// COM interrupt enable register bits
pub const COM_IT_ENA_TYPE_POS: u8 = 0;
pub const COM_IT_ENA_TYPE_MSK: u8 = 0xf << COM_IT_ENA_TYPE_POS;
pub const COM_IT_ENA_TYPE_0: u8 = 0x1 << COM_IT_ENA_TYPE_POS;
pub const COM_IT_ENA_TYPE_1: u8 = 0x2 << COM_IT_ENA_TYPE_POS;
pub const COM_IT_ENA_TYPE_2: u8 = 0x4 << COM_IT_ENA_TYPE_POS;
pub const COM_IT_ENA_TYPE_3: u8 = 0x8 << COM_IT_ENA_TYPE_POS;

// COM modem control register bits
pub const COM_MODEM_CTRL_DTR_POS: u8 = 0;
pub const COM_MODEM_CTRL_DTR_MSK: u8 = 0x1 << COM_MODEM_CTRL_DTR_POS;
pub const COM_MODEM_CTRL_RTS_POS: u8 = 1;
pub const COM_MODEM_CTRL_RTS_MSK: u8 = 0x1 << COM_MODEM_CTRL_RTS_POS;
pub const COM_MODEM_CTRL_OUT1_POS: u8 = 2;
pub const COM_MODEM_CTRL_OUT1_MSK: u8 = 0x1 << COM_MODEM_CTRL_OUT1_POS;
pub const COM_MODEM_CTRL_OUT2_POS: u8 = 3;
pub const COM_MODEM_CTRL_OUT2_MSK: u8 = 0x1 << COM_MODEM_CTRL_OUT2_POS;
pub const COM_MODEM_CTRL_LOOP_POS: u8 = 4;
pub const COM_MODEM_CTRL_LOOP_MSK: u8 = 0x1 << COM_MODEM_CTRL_LOOP_POS;

// COM line status register bits
pub const COM_LINE_STATUS_DR_POS: u8 = 0;
pub const COM_LINE_STATUS_DR_MSK: u8 = 0x1 << COM_LINE_STATUS_DR_POS;
pub const COM_LINE_STATUS_OE_POS: u8 = 1;
pub const COM_LINE_STATUS_OE_MSK: u8 = 0x1 << COM_LINE_STATUS_OE_POS;
pub const COM_LINE_STATUS_PE_POS: u8 = 2;
pub const COM_LINE_STATUS_PE_MSK: u8 = 0x1 << COM_LINE_STATUS_PE_POS;
pub const COM_LINE_STATUS_FE_POS: u8 = 3;
pub const COM_LINE_STATUS_FE_MSK: u8 = 0x1 << COM_LINE_STATUS_FE_POS;
pub const COM_LINE_STATUS_BI_POS: u8 = 4;
pub const COM_LINE_STATUS_BI_MSK: u8 = 0x1 << COM_LINE_STATUS_BI_POS;
pub const COM_LINE_STATUS_THRE_POS: u8 = 5;
pub const COM_LINE_STATUS_THRE_MSK: u8 = 0x1 << COM_LINE_STATUS_THRE_POS;
pub const COM_LINE_STATUS_TEMT_POS: u8 = 6;
pub const COM_LINE_STATUS_TEMT_MSK: u8 = 0x1 << COM_LINE_STATUS_TEMT_POS;
pub const COM_LINE_STATUS_IPE_POS: u8 = 7;
pub const COM_LINE_STATUS_IPE_MSK: u8 = 0x1 << COM_LINE_STATUS_IPE_POS;

// COM modem status register bits
pub const COM_MODEM_STATUS_DCTS_POS: u8 = 0;
pub const COM_MODEM_STATUS_DCTS_MSK: u8 = 0x1 << COM_MODEM_STATUS_DCTS_POS;
pub const COM_MODEM_STATUS_DDSR_POS: u8 = 1;
pub const COM_MODEM_STATUS_DDSR_MSK: u8 = 0x1 << COM_MODEM_STATUS_DDSR_POS;
pub const COM_MODEM_STATUS_TERI_POS: u8 = 2;
pub const COM_MODEM_STATUS_TERI_MSK: u8 = 0x1 << COM_MODEM_STATUS_TERI_POS;
pub const COM_MODEM_STATUS_DDCD_POS: u8 = 3;
pub const COM_MODEM_STATUS_DDCD_MSK: u8 = 0x1 << COM_MODEM_STATUS_DDCD_POS;
pub const COM_MODEM_STATUS_CTS_POS: u8 = 4;
pub const COM_MODEM_STATUS_CTS_MSK: u8 = 0x1 << COM_MODEM_STATUS_CTS_POS;
pub const COM_MODEM_STATUS_DSR_POS: u8 = 5;
pub const COM_MODEM_STATUS_DSR_MSK: u8 = 0x1 << COM_MODEM_STATUS_DSR_POS;
pub const COM_MODEM_STATUS_RI_POS: u8 = 6;
pub const COM_MODEM_STATUS_RI_MSK: u8 = 0x1 << COM_MODEM_STATUS_RI_POS;
pub const COM_MODEM_STATUS_DCD_POS: u8 = 7;
pub const COM_MODEM_STATUS_DCD_MSK: u8 = 0x1 << COM_MODEM_STATUS_DCD_POS;

// LBA disk defines
pub const ATA: u16 = 0x1f0;
pub const ATA_DATA: u16 = ATA + 0;
pub const ATA_ERR: u16 = ATA + 1;
pub const ATA_SECTOR_CNT: u16 = ATA + 2;
pub const ATA_SECTOR_0_7_BITS: u16 = ATA + 3;
pub const ATA_SECTOR_8_15_BITS: u16 = ATA + 4;
pub const ATA_SECTOR_16_23_BITS: u16 = ATA + 5;
pub const ATA_MODE: u16 = ATA + 6;
pub const ATA_CMD: u16 = ATA + 7;
pub const ATA_STATUS: u16 = ATA + 7;

pub const ATA_MODE_LBA: u8 = 0xE0;
pub const ATA_MODE_CHS: u8 = 0xA0;
pub const ATA_MODE_DRV_POS: u8 = 4;

pub const ATA_CMD_READ: u8 = 0x20;
pub const ATA_CMD_WRITE: u8 = 0x30;
pub const ATA_CMD_FLUSH: u8 = 0xE7;

// ATA status register bits
pub const ATA_STATUS_ERR_POS: u8 = 0;
pub const ATA_STATUS_ERR_MSK: u8 = 0x1 << ATA_STATUS_ERR_POS;
pub const ATA_STATUS_IDX_POS: u8 = 1;
pub const ATA_STATUS_IDX_MSK: u8 = 0x1 << ATA_STATUS_IDX_POS;
pub const ATA_STATUS_CORR_POS: u8 = 2;
pub const ATA_STATUS_CORR_MSK: u8 = 0x1 << ATA_STATUS_CORR_POS;
pub const ATA_STATUS_DRQ_POS: u8 = 3;
pub const ATA_STATUS_DRQ_MSK: u8 = 0x1 << ATA_STATUS_DRQ_POS;
pub const ATA_STATUS_SRV_POS: u8 = 4;
pub const ATA_STATUS_SRV_MSK: u8 = 0x1 << ATA_STATUS_SRV_POS;
pub const ATA_STATUS_DF_POS: u8 = 5;
pub const ATA_STATUS_DF_MSK: u8 = 0x1 << ATA_STATUS_DF_POS;
pub const ATA_STATUS_RDY_POS: u8 = 6;
pub const ATA_STATUS_RDY_MSK: u8 = 0x1 << ATA_STATUS_RDY_POS;
pub const ATA_STATUS_BSY_POS: u8 = 7;
pub const ATA_STATUS_BSY_MSK: u8 = 0x1 << ATA_STATUS_BSY_POS;

pub const ATA_PRIMARY_PORT_CTRL: u16 = 0x3F6;
pub const ATA_SECOND_PORT_CTRL: u16 = 0x3F7;

pub const ATA_CTRL_N_IEN: u8 = 0x01 << 1;

// PS/2 Controller IO Ports
pub const PS2_READ_DATA: u16 = 0x60;
pub const PS2_WRITE_DATA: u16 = 0x60;
pub const PS2_READ_STATUS: u16 = 0x64;
pub const PS2_WRITE_COMMAND: u16 = 0x64;

// PS2 commands
pub const PS2_CMD_READ_BYTE_0: u8 = 0x20;
pub const PS2_CMD_READ_BYTE_1: u8 = 0x21;
pub const PS2_CMD_WRITE_NEXT_BYTE_0: u8 = 0x60;
pub const PS2_CMD_WRITE_NEXT_BYTE_1: u8 = 0x61;
pub const PS2_CMD_DIS_SEC_PS2_PORT: u8 = 0xA7;
pub const PS2_CMD_ENA_SEC_PS2_PORT: u8 = 0xA8;
pub const PS2_CMD_TEST_SEC_PS2_PORT: u8 = 0xA9;
pub const PS2_CMD_TEST_PS2_CTL: u8 = 0xAA;
pub const PS2_CMD_TEST_FIRST_PS2_PORT: u8 = 0xAB;
pub const PS2_CMD_DIAGNOSTIC: u8 = 0xAC;
pub const PS2_CMD_DIS_FIRST_PS2_PORT: u8 = 0xAD;
pub const PS2_CMD_ENA_FIRST_PS2_PORT: u8 = 0xAE;
pub const PS2_CMD_READ_CTL_INPUT_PORT: u8 = 0xC0;
pub const PS2_CMD_COPY_BIT03_TO_BIT47: u8 = 0xC1;
pub const PS2_CMD_COPY_BIT47_TO_BIT47: u8 = 0xC2;
pub const PS2_CMD_READ_CTL_OUTPUT_PORT: u8 = 0xD0;
pub const PS2_CMD_WR_CTL_OUTPUT_PORT: u8 = 0xD1;
pub const PS2_CMD_WR_CTL_FST_OUTPUT_BUFF: u8 = 0xD2;
pub const PS2_CMD_WR_CTL_SEC_OUTPUT_BUFF: u8 = 0xD3;
pub const PS2_CMD_WR_CTL_SEC_INPUT_BUFF: u8 = 0xD4;

// PS2 mouse commands
pub const PS2_MOUSE_CMD_SCALING_1_1: u8 = 0xE6;
pub const PS2_MOUSE_CMD_SCALING_2_1: u8 = 0xE7;
pub const PS2_MOUSE_CMD_SET_RESOLUTION: u8 = 0xE8;
pub const PS2_MOUSE_CMD_STATUS_REQ: u8 = 0xE9;
pub const PS2_MOUSE_CMD_SET_STREAM_MODE: u8 = 0xEA;
pub const PS2_MOUSE_CMD_READ_DATA: u8 = 0xEB;
pub const PS2_MOUSE_CMD_RESET_WRAP_MODE: u8 = 0xEC;
pub const PS2_MOUSE_CMD_SET_WRAP_MODE: u8 = 0xEE;
pub const PS2_MOUSE_CMD_SET_REMOTE_MODE: u8 = 0xF0;
pub const PS2_MOUSE_CMD_GET_DEVICE_ID: u8 = 0xF2;
pub const PS2_MOUSE_CMD_SET_SAMPLE_RATE: u8 = 0xF3;
pub const PS2_MOUSE_CMD_ENA_DATA_REPORTING: u8 = 0xF4;
pub const PS2_MOUSE_CMD_DIS_DATA_REPORTING: u8 = 0xF5;
pub const PS2_MOUSE_CMD_SET_DEFAULTS: u8 = 0xF6;
pub const PS2_MOUSE_CMD_RESEND: u8 = 0xFE;
pub const PS2_MOUSE_CMD_RESET: u8 = 0xFF;

// PS2 status register bits
pub const PS2_STATUS_OUTPUT_BUFFER_POS: u8 = 0;
pub const PS2_STATUS_OUTPUT_BUFFER_MSK: u8 = 0x1 << PS2_STATUS_OUTPUT_BUFFER_POS;
pub const PS2_STATUS_INPUT_BUFFER_POS: u8 = 1;
pub const PS2_STATUS_INPUT_BUFFER_MSK: u8 = 0x1 << PS2_STATUS_INPUT_BUFFER_POS;
pub const PS2_STATUS_SYSTEM_FLAG_POS: u8 = 2;
pub const PS2_STATUS_SYSTEM_FLAG_MSK: u8 = 0x1 << PS2_STATUS_SYSTEM_FLAG_POS;

// IRQ numbers
pub const DIVISION_BY_ZERO_IRQN: isize = 0;
pub const DEBUG_IRQN: isize = 1;
pub const NON_MASKABLE_INTERRUPT_IRQN: isize = 2;
pub const BREAKPOINT_IRQN: isize = 3;
pub const INTO_DETECTED_OVERFLOW_IRQN: isize = 4;
pub const OUT_OF_BOUNDS_IRQN: isize = 5;
pub const INVALID_OPCODE_IRQN: isize = 6;
pub const NO_COPROCESSOR_IRQN: isize = 7;
pub const DOUBLE_FAULT_IRQN: isize = 8;
pub const COPROCESSOR_SEGMENT_OVERRUN_IRQN: isize = 9;
pub const BAD_TSS_IRQN: isize = 10;
pub const SEGMENT_NOT_PRESENT_IRQN: isize = 11;
pub const STACK_FAULT_IRQN: isize = 12;
pub const GENERAL_PROTECTION_FAULT_IRQN: isize = 13;
pub const PAGE_FAULT_IRQN: isize = 14;
pub const UNKNOWN_INTERRUPT_IRQN: isize = 15;
pub const COPROCESSOR_FAULT_IRQN: isize = 16;
pub const ALIGNMENT_CHECK_IRQN: isize = 17;
pub const MACHINE_CHECK_IRQN: isize = 18;
pub const SVC_IRQN: isize = 30;
pub const PENDSV_IRQN: isize = 31;

pub const SYSTICK_IRQN: isize = 32;
pub const KEYBOARD_CONTROLLER_IRQN: isize = 33;
pub const SERIAL_PORT_COM2_IRQN: isize = 35;
pub const SERIAL_PORT_COM1_IRQN: isize = 36;
pub const LINE_PRINT_TERMINAL2_IRQN: isize = 37;
pub const FLOPPY_CONTROLLER_IRQN: isize = 38;
pub const LINE_PRINT_TERMINAL1_IRQN: isize = 39;
pub const RTC_TIMER_IRQN: isize = 40;
pub const X86_ASSEMBLY_ACPI_IRQN: isize = 41;
pub const MOUSE_CONTROLLER_IRQN: isize = 44;
pub const MATH_COPROCESSOR_IRQN: isize = 45;
pub const ATA_CHANNEL1_IRQN: isize = 46;
pub const ATA_CHANNEL2_IRQN: isize = 47;

// Read a byte from the specified port
#[inline(always)]
pub fn port_byte_in(port: u16) -> u8 {
    let mut val: u8;
    unsafe { asm!("inb %dx, %al", in("dx") port, out("al") val, options(att_syntax)); }
    val
}

// Write a byte to the specified port
#[inline(always)]
pub fn port_byte_out(port: u16, val: u8) {
    unsafe { asm!("outb %al, %dx", in("al") val, in("dx") port, options(att_syntax)); }
}

// Read a word from the specified port
#[inline(always)]
pub fn port_word_in(port: u16) -> u16 {
    let mut val: u16;
    unsafe { asm!("inw %dx, %ax", in("dx") port, out("ax") val, options(att_syntax)); }
    val
}

// Write a word to the specified port
#[inline(always)]
pub fn port_word_out(port: u16, val: u16) {
    unsafe { asm!("outw %ax, %dx", in("ax") val, in("dx") port, options(att_syntax)); }
}

// Read a long word from the specified port
#[inline(always)]
pub fn port_long_in(port: u16) -> u32 {
    let mut val: u32;
    unsafe { asm!("inl %dx, %eax", in("dx") port, out("eax") val, options(att_syntax)); }
    val
}

// Write a long word to the specified port
#[inline(always)]
pub fn port_long_out(port: u16, val: u32) {
    unsafe { asm!("outl %eax, %dx", in("eax") val, in("dx") port, options(att_syntax)); }
}
