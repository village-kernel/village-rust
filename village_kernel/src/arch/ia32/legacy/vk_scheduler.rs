//###########################################################################
// vk_scheduler.rs
// The specific implementation of functions related to scheduler
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_callback::Callback;
use crate::traits::vk_kernel::Scheduler;
use crate::vendor::ia32legacy::core::i686::{PENDSV_IRQN, SYSTICK_IRQN};
use crate::village::kernel;
use core::arch::{asm, naked_asm};

// Struct village scheduler
pub struct VillageScheduler {
    is_ready: bool,
}

// Impl village scheduler
impl VillageScheduler {
    // New
    pub const fn new() -> Self {
        Self { is_ready: false }
    }
}

// Impl village scheduler
impl VillageScheduler {
    // Setup
    pub fn setup(&mut self) {
        // Clear start schedule flag
        self.is_ready = false;

        // Set the PendSV interrupt handler
        let pendsv_cb = Callback::new(Self::pend_sv_handler as u32);
        kernel().interrupt().set_isr_cb(PENDSV_IRQN, pendsv_cb);

        // Add the systick interrupt handler
        let sched_cb = Callback::new(Self::sched as u32).with_instance(self);
        kernel().interrupt().add_isr_cb(SYSTICK_IRQN, sched_cb);

        // Output debug info
        kernel().debug().info("Scheduler setup completed!");
    }

    // Exit
    pub fn exit(&mut self) {
        // Delete the systick interrupt handler
        let sched_cb = Callback::new(Self::sched as u32).with_instance(self);
        kernel().interrupt().del_isr_cb(SYSTICK_IRQN, sched_cb);

        // Clear the pend sv interrupt handler
        kernel().interrupt().clear_isr_cb(PENDSV_IRQN);
    }
}

// Impl scheduler for village scheduler
impl Scheduler for VillageScheduler {
    // Start scheduler
    fn start(&mut self) {
        // Output debug info
        kernel().debug().info("Scheduler starts scheduling!");

        // Set interrupt flag
        kernel().system().enable_irq();

        // Set start schedule flag
        self.is_ready = true;

        // Execute thread idle task
        kernel().thread().idle_task();
    }

    // Rescheduler task
    fn sched(&mut self) {
        // Not ready to schedule
        if !self.is_ready {
            return;
        }

        // Trigger PendSV directly
        unsafe {
            asm!("int $31", options(att_syntax));
        }
    }
}

// Impl village scheduler
impl VillageScheduler {
    // Save task psp
    #[unsafe(no_mangle)]
    unsafe extern "C" fn save_task_psp(psp: u32) {
        kernel().thread().save_task_psp(psp);
    }

    // Select next task
    #[unsafe(no_mangle)]
    unsafe extern "C" fn select_next_task() {
        kernel().thread().select_next_task();
    }

    // Get task psp
    #[unsafe(no_mangle)]
    unsafe extern "C" fn get_task_psp() -> u32 {
        kernel().thread().get_task_psp()
    }

    // Naked pend sv handler
    #[unsafe(naked)]
    unsafe extern "C" fn pend_sv_handler() {
        naked_asm!(
            "pushl %ebp",
            "pushl %ebx",
            "pushl %esi",
            "pushl %edi",
            "movl %esp, %eax",
            "pushl %eax",
            "call save_task_psp",
            "addl $4, %esp",
            "call select_next_task",
            "call get_task_psp",
            "movl %eax, %esp",
            "popl %edi",
            "popl %esi",
            "popl %ebx",
            "popl %ebp",
            "sti",
            "ret",
            options(att_syntax)
        );
    }
}
