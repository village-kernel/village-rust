use crate::kernel::traits::vk_kernel::*;
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
use crate::arch::ia32::legacy::vk_system::ConcreteSystem;
use crate::arch::ia32::legacy::vk_scheduler::ConcreteScheduler;
use crate::filesys::impls::vk_filesystem::ConcreteFileSystem;
use crate::protocol::impls::vk_protocol::ConcreteProtocol;
use crate::terminal::impls::vk_terminal::ConcreteTerminal;

pub struct Village {
    system: ConcreteSystem,
    memory: ConcreteMemory,
    debug: ConcreteDebug,
    interrupt: ConcreteInterrupt,
    scheduler: ConcreteScheduler,
    thread: ConcreteThread,
    workqueue: ConcreteWorkQueue,
    event: ConcreteEvent,
    symbol: ConcreteSymbol,
    device: ConcreteDevice,
    feature: ConcreteFeature,
    filesys: ConcreteFileSystem,
    loader: ConcreteLoader,
    process: ConcreteProcess,
    timer: ConcreteTimer,
    terminal: ConcreteTerminal,
    signal: ConcreteSignal,
    protocol: ConcreteProtocol,
}

impl Village {
    pub const fn new() -> Self {
        Self {
            system: ConcreteSystem,
            memory: ConcreteMemory,
            debug: ConcreteDebug,
            interrupt: ConcreteInterrupt,
            scheduler: ConcreteScheduler,
            thread: ConcreteThread,
            workqueue: ConcreteWorkQueue,
            event: ConcreteEvent,
            symbol: ConcreteSymbol,
            device: ConcreteDevice,
            feature: ConcreteFeature,
            filesys: ConcreteFileSystem,
            loader: ConcreteLoader,
            process: ConcreteProcess,
            timer: ConcreteTimer,
            terminal: ConcreteTerminal,
            signal: ConcreteSignal,
            protocol: ConcreteProtocol,
        }
    }
}

impl Kernel for Village {
    fn setup(&self) {
        
    }

    fn start(&self) {
        
    }

    fn exit(&self) {
        
    }

    fn sleep(&self) {
        self.system.sleep();
    }

    fn standby(&self) {
        self.system.standby();
    }

    fn shutdown(&self) {
        self.system.shutdown();
    }

    fn reboot(&self) {
        self.system.reboot();
    }

    fn get_build_date(&self) {
        
    }

    fn get_build_time(&self) {
        
    }

    fn get_build_version(&self) {
        
    }

    fn get_build_git_sha(&self) {
        
    }

    fn system(&self) -> &dyn System {
        &self.system
    }
    
    fn memory(&self) -> &dyn Memory {
        &self.memory
    }

    fn debug(&self) -> &dyn Debug {
        &self.debug
    }

    fn interrupt(&self) -> &dyn Interrupt {
        &self.interrupt
    }

    fn scheduler(&self) -> &dyn Scheduler {
        &self.scheduler
    }

    fn thread(&self) -> &dyn Thread {
        &self.thread
    }

    fn workqueue(&self) -> &dyn WorkQueue {
        &self.workqueue
    }

    fn event(&self) -> &dyn Event {
        &self.event
    }

    fn symbol(&self) -> &dyn Symbol {
        &self.symbol
    }

    fn device(&self) -> &dyn Device {
        &self.device
    }

    fn feature(&self) -> &dyn Feature {
        &self.feature
    }

    fn filesys(&self) -> &dyn FileSystem {
        &self.filesys
    }

    fn loader(&self) -> &dyn Loader {
        &self.loader
    }

    fn process(&self) -> &dyn Process {
        &self.process
    }

    fn timer(&self) -> &dyn Timer {
        &self.timer
    }

    fn terminal(&self) -> &dyn Terminal {
        &self.terminal
    }

    fn signal(&self) -> &dyn Signal {
        &self.signal
    }

    fn protocol(&self) -> &dyn Protocol {
        &self.protocol
    }
}

pub static KERNEL_INSTANCE: Village = Village::new();
