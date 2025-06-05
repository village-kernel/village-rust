//###########################################################################
// vk_village.rs
// The specific implementation of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use crate::traits::vk_kernel::*;
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
    fn setup(&self) {
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
    fn start(&self) {
        // Start thread
        self.thread.start();

        // Start scheduler
        self.scheduler.start();

        // Should not go to here
        loop {}
    }

    // Exit
    fn exit(&self) {
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
    fn sleep(&self) {
        self.system.sleep();
    }

    // Standby
    fn standby(&self) {
        self.system.standby();
    }

    // Shutdown
    fn shutdown(&self) {
        self.system.shutdown();
    }

    // Reboot
    fn reboot(&self) {
        self.system.reboot();
    }

    // Get build date
    fn get_build_date(&self) {
        
    }

    // Get build time
    fn get_build_time(&self) {
        
    }

    // Get build version
    fn get_build_version(&self) {
        
    }

    // Get build git sha
    fn get_build_git_sha(&self) {
        
    }

    // System
    fn system(&self) -> &dyn System {
        &self.system
    }
    
    // Memory
    fn memory(&self) -> &dyn Memory {
        &self.memory
    }

    // Debug
    fn debug(&self) -> &dyn Debug {
        &self.debug
    }

    // Interrupt
    fn interrupt(&self) -> &dyn Interrupt {
        &self.interrupt
    }

    // Scheduler
    fn scheduler(&self) -> &dyn Scheduler {
        &self.scheduler
    }

    // Thread
    fn thread(&self) -> &dyn Thread {
        &self.thread
    }

    // Workqueue
    fn workqueue(&self) -> &dyn WorkQueue {
        &self.workqueue
    }

    // Event
    fn event(&self) -> &dyn Event {
        &self.event
    }

    // Symbol
    fn symbol(&self) -> &dyn Symbol {
        &self.symbol
    }

    // Device
    fn device(&self) -> &dyn Device {
        &self.device
    }

    // Feature
    fn feature(&self) -> &dyn Feature {
        &self.feature
    }

    // Filesys
    fn filesys(&self) -> &dyn FileSystem {
        &self.filesys
    }

    // Loader
    fn loader(&self) -> &dyn Loader {
        &self.loader
    }

    // Process
    fn process(&self) -> &dyn Process {
        &self.process
    }

    // Timer
    fn timer(&self) -> &dyn Timer {
        &self.timer
    }

    // Terminal
    fn terminal(&self) -> &dyn Terminal {
        &self.terminal
    }

    // Signal
    fn signal(&self) -> &dyn Signal {
        &self.signal
    }

    // Protocol
    fn protocol(&self) -> &dyn Protocol {
        &self.protocol
    }
}
