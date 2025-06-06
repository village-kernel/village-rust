//###########################################################################
// vk_village.rs
// The specific implementation of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::Kernel;
use crate::traits::vk_kernel::System;
use crate::traits::vk_kernel::Memory;
use crate::traits::vk_kernel::Debug;
use crate::traits::vk_kernel::Interrupt;
use crate::traits::vk_kernel::Scheduler;
use crate::traits::vk_kernel::Thread;
use crate::traits::vk_kernel::WorkQueue;
use crate::traits::vk_kernel::Event;
use crate::traits::vk_kernel::Symbol;
use crate::traits::vk_kernel::Device;
use crate::traits::vk_kernel::Feature;
use crate::traits::vk_kernel::FileSystem;
use crate::traits::vk_kernel::Loader;
use crate::traits::vk_kernel::Process;
use crate::traits::vk_kernel::Timer;
use crate::traits::vk_kernel::Terminal;
use crate::traits::vk_kernel::Signal;
use crate::traits::vk_kernel::Protocol;

use super::vk_debug::ConcreteDebug;
use super::vk_device::ConcreteDevice;
use super::vk_event::ConcreteEvent;
use super::vk_feature::ConcreteFeature;
use super::vk_loader::ConcreteLoader;
use super::vk_memory::ConcreteMemory;
use super::vk_interrupt::ConcreteInterrupt;
use super::vk_process::ConcreteProcess;
use super::vk_signal::ConcreteSignal;
use super::vk_symbol::ConcreteSymbol;
use super::vk_thread::ConcreteThread;
use super::vk_timer::ConcreteTimer;
use super::vk_workqueue::ConcreteWorkQueue;
use crate::filesys::vk_filesystem::ConcreteFileSystem;
use crate::protocol::vk_protocol::ConcreteProtocol;
use crate::terminal::vk_terminal::ConcreteTerminal;
use crate::arch::ia32::legacy::vk_system::ConcreteSystem;
use crate::arch::ia32::legacy::vk_scheduler::ConcreteScheduler;

// Struct village
pub struct Village {
    system:    ConcreteSystem,
    memory:    ConcreteMemory,
    debug:     ConcreteDebug,
    interrupt: ConcreteInterrupt,
    scheduler: ConcreteScheduler,
    thread:    ConcreteThread,
    workqueue: ConcreteWorkQueue,
    event:     ConcreteEvent,
    symbol:    ConcreteSymbol,
    device:    ConcreteDevice,
    feature:   ConcreteFeature,
    filesys:   ConcreteFileSystem,
    loader:    ConcreteLoader,
    process:   ConcreteProcess,
    timer:     ConcreteTimer,
    terminal:  ConcreteTerminal,
    signal:    ConcreteSignal,
    protocol:  ConcreteProtocol,
}

// Impl village 
impl Village {
    // New village object
    pub const fn new() -> Self {
        Self {
            system:    ConcreteSystem::new(),
            memory:    ConcreteMemory::new(),
            debug:     ConcreteDebug::new(),
            interrupt: ConcreteInterrupt::new(),
            scheduler: ConcreteScheduler::new(),
            thread:    ConcreteThread::new(),
            workqueue: ConcreteWorkQueue::new(),
            event:     ConcreteEvent::new(),
            symbol:    ConcreteSymbol::new(),
            device:    ConcreteDevice::new(),
            feature:   ConcreteFeature::new(),
            filesys:   ConcreteFileSystem::new(),
            loader:    ConcreteLoader::new(),
            process:   ConcreteProcess::new(),
            timer:     ConcreteTimer::new(),
            terminal:  ConcreteTerminal::new(),
            signal:    ConcreteSignal::new(),
            protocol:  ConcreteProtocol::new(),
        }
    }
}

// Impl kernel for village 
impl Kernel for Village {
    // Setup
    fn setup(&mut self) {
        // Setup system
        self.system.setup();

        // Setup memory
        self.memory.setup();

        // Setup interrupt
        self.interrupt.setup();

        // Setup device
        self.device.setup();

        // Setup debug
        self.debug.setup();

        // Setup scheduler
        self.scheduler.setup();

        // Setup thread
        self.thread.setup();

        // Setup work queue
        self.workqueue.setup();

        // Setup event
        self.event.setup();

        // Setup symbol
        self.symbol.setup();

        // Setup timer
        self.timer.setup();

        // Setup filesys
        self.filesys.setup();

        // Setup terminal
        self.terminal.setup();

        // Setup feature
        self.feature.setup();

        // Setup loader
        self.loader.setup();

        // Setup process
        self.process.setup();

        // Setup signal
        self.signal.setup();

        // Setup protocol
        self.protocol.setup();
    }

    // Start
    fn start(&mut self) {
        // Start thread
        self.thread.start();

        // Start scheduler
        self.scheduler.start();

        // Should not go to here
        loop {}
    }

    // Exit
    fn exit(&mut self) {
        // Exit protocol
        self.protocol.exit();

        // Exit signal
        self.signal.exit();

        // Exit process
        self.process.exit();

        // Exit loader
        self.loader.exit();

        // Exit feature
        self.feature.exit();

        // Exit terminal
        self.terminal.exit();

        // Exit filesys
        self.filesys.exit();

        // Exit timer
        self.timer.exit();

        // Exit symbol
        self.symbol.exit();

        // Exit event
        self.event.exit();

        // Exit work queue
        self.workqueue.exit();

        // Exit thread
        self.thread.exit();

        // Exit debug
        self.debug.exit();

        // Exit device
        self.device.exit();

        // Exit interrupt
        self.interrupt.exit();

        // Exit memory
        self.memory.exit();

        // Exit system
        self.system.exit();
    }

    // Sleep
    fn sleep(&mut self) {
        self.system.sleep();
    }

    // Standby
    fn standby(&mut self) {
        self.system.standby();
    }

    // Shutdown
    fn shutdown(&mut self) {
        self.system.shutdown();
    }

    // Reboot
    fn reboot(&mut self) {
        self.system.reboot();
    }

    // Get build date
    fn get_build_date(&mut self) {
        
    }

    // Get build time
    fn get_build_time(&mut self) {
        
    }

    // Get build version
    fn get_build_version(&mut self) {
        
    }

    // Get build git sha
    fn get_build_git_sha(&mut self) {
        
    }

    // System
    fn system(&mut self) -> &mut dyn System {
        &mut self.system
    }
    
    // Memory
    fn memory(&mut self) -> &mut dyn Memory {
        &mut self.memory
    }

    // Debug
    fn debug(&mut self) -> &mut dyn Debug {
        &mut self.debug
    }

    // Interrupt
    fn interrupt(&mut self) -> &mut dyn Interrupt {
        &mut self.interrupt
    }

    // Scheduler
    fn scheduler(&mut self) -> &mut dyn Scheduler {
        &mut self.scheduler
    }

    // Thread
    fn thread(&mut self) -> &mut dyn Thread {
        &mut self.thread
    }

    // Workqueue
    fn workqueue(&mut self) -> &mut dyn WorkQueue {
        &mut self.workqueue
    }

    // Event
    fn event(&mut self) -> &mut dyn Event {
        &mut self.event
    }

    // Symbol
    fn symbol(&mut self) -> &mut dyn Symbol {
        &mut self.symbol
    }

    // Device
    fn device(&mut self) -> &mut dyn Device {
        &mut self.device
    }

    // Feature
    fn feature(&mut self) -> &mut dyn Feature {
        &mut self.feature
    }

    // Filesys
    fn filesys(&mut self) -> &mut dyn FileSystem {
        &mut self.filesys
    }

    // Loader
    fn loader(&mut self) -> &mut dyn Loader {
        &mut self.loader
    }

    // Process
    fn process(&mut self) -> &mut dyn Process {
        &mut self.process
    }

    // Timer
    fn timer(&mut self) -> &mut dyn Timer {
        &mut self.timer
    }

    // Terminal
    fn terminal(&mut self) -> &mut dyn Terminal {
        &mut self.terminal
    }

    // Signal
    fn signal(&mut self) -> &mut dyn Signal {
        &mut self.signal
    }

    // Protocol
    fn protocol(&mut self) -> &mut dyn Protocol {
        &mut self.protocol
    }
}
