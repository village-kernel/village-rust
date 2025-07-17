//###########################################################################
// vk_village.rs
// The specific implementation of functions related to kernel
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;

use crate::traits::vk_kernel::BuildInfo;
use crate::traits::vk_kernel::Debug;
use crate::traits::vk_kernel::Device;
use crate::traits::vk_kernel::Event;
use crate::traits::vk_kernel::Feature;
use crate::traits::vk_kernel::Executer;
use crate::traits::vk_kernel::FileSystem;
use crate::traits::vk_kernel::Interrupt;
use crate::traits::vk_kernel::Kernel;
use crate::traits::vk_kernel::Module;
use crate::traits::vk_kernel::Memory;
use crate::traits::vk_kernel::Library;
use crate::traits::vk_kernel::Process;
use crate::traits::vk_kernel::Protocol;
use crate::traits::vk_kernel::Scheduler;
use crate::traits::vk_kernel::Signal;
use crate::traits::vk_kernel::Symbol;
use crate::traits::vk_kernel::System;
use crate::traits::vk_kernel::Terminal;
use crate::traits::vk_kernel::Thread;
use crate::traits::vk_kernel::Timer;
use crate::traits::vk_kernel::WorkQueue;

use super::vk_debug::VillageDebug;
use super::vk_device::VillageDevice;
use super::vk_event::VillageEvent;
use super::vk_feature::VillageFeature;
use super::vk_executer::VillageExecuter;
use super::vk_interrupt::VillageInterrupt;
use super::vk_module::VillageModule;
use super::vk_memory::VillageMemory;
use super::vk_library::VillageLibrary;
use super::vk_process::VillageProcess;
use super::vk_signal::VillageSignal;
use super::vk_symbol::VillageSymbol;
use super::vk_thread::VillageThread;
use super::vk_timer::VillageTimer;
use super::vk_workqueue::VillageWorkQueue;
use crate::arch::ia32::legacy::vk_scheduler::VillageScheduler;
use crate::arch::ia32::legacy::vk_system::VillageSystem;
use crate::filesys::vk_filesystem::VillageFileSystem;
use crate::protocol::vk_protocol::VillageProtocol;
use crate::terminal::vk_terminal::VillageTerminal;

// Struct village kernel
pub struct VillageKernel {
    memory: Box<VillageMemory>,
    debug: Box<VillageDebug>,
    interrupt: Box<VillageInterrupt>,
    system: Box<VillageSystem>,
    scheduler: Box<VillageScheduler>,
    thread: Box<VillageThread>,
    workqueue: Box<VillageWorkQueue>,
    event: Box<VillageEvent>,
    symbol: Box<VillageSymbol>,
    device: Box<VillageDevice>,
    executer: Box<VillageExecuter>,
    feature: Box<VillageFeature>,
    filesys: Box<VillageFileSystem>,
    library: Box<VillageLibrary>,
    module: Box<VillageModule>,
    process: Box<VillageProcess>,
    timer: Box<VillageTimer>,
    terminal: Box<VillageTerminal>,
    signal: Box<VillageSignal>,
    protocol: Box<VillageProtocol>,
}

// Impl village kernel
impl VillageKernel {
    // New village kernel
    pub fn new() -> Self {
        Self {
            memory: Box::new(VillageMemory::new()),
            debug: Box::new(VillageDebug::new()),
            interrupt: Box::new(VillageInterrupt::new()),
            system: Box::new(VillageSystem::new()),
            scheduler: Box::new(VillageScheduler::new()),
            thread: Box::new(VillageThread::new()),
            workqueue: Box::new(VillageWorkQueue::new()),
            event: Box::new(VillageEvent::new()),
            symbol: Box::new(VillageSymbol::new()),
            device: Box::new(VillageDevice::new()),
            executer: Box::new(VillageExecuter::new()),
            feature: Box::new(VillageFeature::new()),
            filesys: Box::new(VillageFileSystem::new()),
            library: Box::new(VillageLibrary::new()),
            module: Box::new(VillageModule::new()),
            process: Box::new(VillageProcess::new()),
            timer: Box::new(VillageTimer::new()),
            terminal: Box::new(VillageTerminal::new()),
            signal: Box::new(VillageSignal::new()),
            protocol: Box::new(VillageProtocol::new()),
        }
    }
}

// Impl kernel for village kernel
impl Kernel for VillageKernel {
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

        // Setup library
        self.library.setup();

        // Setup module
        self.module.setup();

        // Setup executer
        self.executer.setup();

        // Setup feature
        self.feature.setup();

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

        // Will go to here
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

        // Exit feature
        self.feature.exit();

        // Exit executer
        self.executer.exit();

        // Exit module
        self.module.exit();

        // Exit library
        self.library.exit();

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

    // Executer
    fn executer(&mut self) -> &mut dyn Executer {
        self.executer.as_mut()
    }

    // Feature
    fn feature(&mut self) -> &mut dyn Feature {
        self.feature.as_mut()
    }

    // Filesys
    fn filesys(&mut self) -> &mut dyn FileSystem {
        self.filesys.as_mut()
    }

    // Library
    fn library(&mut self) -> &mut dyn Library {
        self.library.as_mut()
    }

    // Module
    fn module(&mut self) -> &mut dyn Module {
        self.module.as_mut()
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
