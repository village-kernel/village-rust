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

// struct village
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

// impl village 
impl Village {
    // new village object
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

// impl kernel for village 
impl Kernel for Village {
    // setup
    fn setup(&self) {
        // setup system
        self.system.setup();

        // setup memory
        self.memory.setup();

        // setup interrupt
        self.interrupt.setup();

        // setup device
        self.device.setup();

        // setup debug
        self.debug.setup();

        // setup scheduler
        self.scheduler.setup();

        // setup thread
        self.thread.setup();

        // setup work queue
        self.workqueue.setup();

        // setup event
        self.event.setup();

        // setup symbol
        self.symbol.setup();

        // setup timer
        self.timer.setup();

        // setup filesys
        self.filesys.setup();

        // setup terminal
        self.terminal.setup();

        // setup feature
        self.feature.setup();

        // setup loader
        self.loader.setup();

        // setup process
        self.process.setup();

        // setup signal
        self.signal.setup();

        // setup protocol
        self.protocol.setup();
    }

    // start
    fn start(&self) {
        // start thread
        self.thread.start();

        // start scheduler
        self.scheduler.start();

        // should not go to here
        loop {}
    }

    // exit
    fn exit(&self) {
        // exit protocol
        self.protocol.exit();

        // exit signal
        self.signal.exit();

        // exit process
        self.process.exit();

        // exit loader
        self.loader.exit();

        // exit feature
        self.feature.exit();

        // exit terminal
        self.terminal.exit();

        // exit filesys
        self.filesys.exit();

        // exit timer
        self.timer.exit();

        // exit symbol
        self.symbol.exit();

        // exit event
        self.event.exit();

        // exit work queue
        self.workqueue.exit();

        // exit thread
        self.thread.exit();

        // exit debug
        self.debug.exit();

        // exit device
        self.device.exit();

        // exit interrupt
        self.interrupt.exit();

        // exit memory
        self.memory.exit();

        // exit system
        self.system.exit();
    }

    // sleep
    fn sleep(&self) {
        self.system.sleep();
    }

    // standby
    fn standby(&self) {
        self.system.standby();
    }

    // shutdown
    fn shutdown(&self) {
        self.system.shutdown();
    }

    // reboot
    fn reboot(&self) {
        self.system.reboot();
    }

    // get build date
    fn get_build_date(&self) {
        
    }

    // get build time
    fn get_build_time(&self) {
        
    }

    // get build version
    fn get_build_version(&self) {
        
    }

    // get build git sha
    fn get_build_git_sha(&self) {
        
    }

    // system
    fn system(&self) -> &dyn System {
        &self.system
    }
    
    // memory
    fn memory(&self) -> &dyn Memory {
        &self.memory
    }

    // debug
    fn debug(&self) -> &dyn Debug {
        &self.debug
    }

    // interrupt
    fn interrupt(&self) -> &dyn Interrupt {
        &self.interrupt
    }

    // scheduler
    fn scheduler(&self) -> &dyn Scheduler {
        &self.scheduler
    }

    // thread
    fn thread(&self) -> &dyn Thread {
        &self.thread
    }

    // workqueue
    fn workqueue(&self) -> &dyn WorkQueue {
        &self.workqueue
    }

    // event
    fn event(&self) -> &dyn Event {
        &self.event
    }

    // symbol
    fn symbol(&self) -> &dyn Symbol {
        &self.symbol
    }

    // device
    fn device(&self) -> &dyn Device {
        &self.device
    }

    // feature
    fn feature(&self) -> &dyn Feature {
        &self.feature
    }

    // filesys
    fn filesys(&self) -> &dyn FileSystem {
        &self.filesys
    }

    // loader
    fn loader(&self) -> &dyn Loader {
        &self.loader
    }

    // process
    fn process(&self) -> &dyn Process {
        &self.process
    }

    // timer
    fn timer(&self) -> &dyn Timer {
        &self.timer
    }

    // terminal
    fn terminal(&self) -> &dyn Terminal {
        &self.terminal
    }

    // signal
    fn signal(&self) -> &dyn Signal {
        &self.signal
    }

    // protocol
    fn protocol(&self) -> &dyn Protocol {
        &self.protocol
    }
}

// static kernel instance
pub static KERNEL_INSTANCE: Village = Village::new();
