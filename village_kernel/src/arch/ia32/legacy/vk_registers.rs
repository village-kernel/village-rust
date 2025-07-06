//###########################################################################
// vk_registers.rs
// The specific implementation of functions related to registers
//
// $Copyright: Copyright (C) village
//###########################################################################

#[repr(C)]
pub struct Registers {
    // Pushed by pusha.
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,

    // Segment selector
    pub gs: u32,
    pub fs: u32,
    pub es: u32,
    pub ds: u32,

    // Interrupt number and error code
    pub irq: u32,
    pub err: u32,

    // Pushed by the processor automatically
    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
    pub psp: u32,
    pub ss: u32,
}

impl Registers {
    pub const fn new(
        eip: u32,
        eax: u32,
        ecx: u32,
        edx: u32,
        ebx: u32,
        esp: u32,
        ebp: u32,
        esi: u32,
        edi: u32,
    ) -> Self {
        Self {
            edi,
            esi,
            ebp,
            esp,
            ebx,
            edx,
            ecx,
            eax,
            gs: 0,
            fs: 0,
            es: 0,
            ds: 0,
            irq: 0,
            err: 0,
            eip,
            cs: 0,
            eflags: 0x00000200,
            psp: 0,
            ss: 0,
        }
    }
}

#[repr(C)]
pub struct TaskContext {
    pub edi: u32,
    pub esi: u32,
    pub ebx: u32,
    pub ebp: u32,
    pub eip: u32,

    pub ret: u32,
    pub arg0: u32,
    pub arg1: u32,
    pub arg2: u32,
    pub arg3: u32,
}

impl TaskContext {
    pub const fn new(eip: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> Self {
        Self {
            edi: 0,
            esi: 0,
            ebx: 0,
            ebp: 0x2000000,
            eip,
            ret: 0,
            arg0,
            arg1,
            arg2,
            arg3,
        }
    }
}
