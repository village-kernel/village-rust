//###########################################################################
// vk_village.rs
// The specific implementation of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;

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
use crate::traits::vk_kernel::BuildInfo;

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
    memory:    Box<ConcreteMemory>,
    debug:     Box<ConcreteDebug>,
    interrupt: Box<ConcreteInterrupt>,
    system:    Box<ConcreteSystem>,
    scheduler: Box<ConcreteScheduler>,
    thread:    Box<ConcreteThread>,
    workqueue: Box<ConcreteWorkQueue>,
    event:     Box<ConcreteEvent>,
    symbol:    Box<ConcreteSymbol>,
    device:    Box<ConcreteDevice>,
    feature:   Box<ConcreteFeature>,
    filesys:   Box<ConcreteFileSystem>,
    loader:    Box<ConcreteLoader>,
    process:   Box<ConcreteProcess>,
    timer:     Box<ConcreteTimer>,
    terminal:  Box<ConcreteTerminal>,
    signal:    Box<ConcreteSignal>,
    protocol:  Box<ConcreteProtocol>,
}

// Impl village 
impl Village {
    // New village object
    pub fn new() -> Self {
        Self {
            memory:    Box::new(ConcreteMemory::new()),
            debug:     Box::new(ConcreteDebug::new()),
            interrupt: Box::new(ConcreteInterrupt::new()),
            system:    Box::new(ConcreteSystem::new()),
            scheduler: Box::new(ConcreteScheduler::new()),
            thread:    Box::new(ConcreteThread::new()),
            workqueue: Box::new(ConcreteWorkQueue::new()),
            event:     Box::new(ConcreteEvent::new()),
            symbol:    Box::new(ConcreteSymbol::new()),
            device:    Box::new(ConcreteDevice::new()),
            feature:   Box::new(ConcreteFeature::new()),
            filesys:   Box::new(ConcreteFileSystem::new()),
            loader:    Box::new(ConcreteLoader::new()),
            process:   Box::new(ConcreteProcess::new()),
            timer:     Box::new(ConcreteTimer::new()),
            terminal:  Box::new(ConcreteTerminal::new()),
            signal:    Box::new(ConcreteSignal::new()),
            protocol:  Box::new(ConcreteProtocol::new()),
        }
    }
}

// Impl kernel for village 
impl Kernel for Village {
    // Setup
    fn setup(&mut self) {
        // Setup memory
        self.memory.setup();

        // Setup interrupt
        self.interrupt.setup();

        // Setup system
        self.system.setup();

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

    // System
    fn system(&mut self) -> &mut dyn System {
        self.system.as_mut()
    }
    
    // Memory
    fn memory(&mut self) -> &mut dyn Memory {
        self.memory.as_mut()
    }

    // Debug
    fn debug(&mut self) -> &mut dyn Debug {
        self.debug.as_mut()
    }

    // Interrupt
    fn interrupt(&mut self) -> &mut dyn Interrupt {
        self.interrupt.as_mut()
    }

    // Scheduler
    fn scheduler(&mut self) -> &mut dyn Scheduler {
        self.scheduler.as_mut()
    }

    // Thread
    fn thread(&mut self) -> &mut dyn Thread {
        self.thread.as_mut()
    }

    // Workqueue
    fn workqueue(&mut self) -> &mut dyn WorkQueue {
        self.workqueue.as_mut()
    }

    // Event
    fn event(&mut self) -> &mut dyn Event {
        self.event.as_mut()
    }

    // Symbol
    fn symbol(&mut self) -> &mut dyn Symbol {
        self.symbol.as_mut()
    }

    // Device
    fn device(&mut self) -> &mut dyn Device {
        self.device.as_mut()
    }

    // Feature
    fn feature(&mut self) -> &mut dyn Feature {
        self.feature.as_mut()
    }

    // Filesys
    fn filesys(&mut self) -> &mut dyn FileSystem {
        self.filesys.as_mut()
    }

    // Loader
    fn loader(&mut self) -> &mut dyn Loader {
        self.loader.as_mut()
    }

    // Process
    fn process(&mut self) -> &mut dyn Process {
        self.process.as_mut()
    }

    // Timer
    fn timer(&mut self) -> &mut dyn Timer {
        self.timer.as_mut()
    }

    // Terminal
    fn terminal(&mut self) -> &mut dyn Terminal {
        self.terminal.as_mut()
    }

    // Signal
    fn signal(&mut self) -> &mut dyn Signal {
        self.signal.as_mut()
    }

    // Protocol
    fn protocol(&mut self) -> &mut dyn Protocol {
        self.protocol.as_mut()
    }

    // Build info
    fn build_info(&self) -> &BuildInfo {
        const INFO: BuildInfo = BuildInfo {
            year: env!("BUILD_YEAR"),
            date: env!("BUILD_DATE"),
            time: env!("BUILD_TIME"),
            version: env!("BUILD_VER"),
            git_sha: env!("GIT_COMMIT"),
        };
        &INFO
    }
}
