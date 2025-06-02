#![no_std]
#![no_main]

use core::panic::PanicInfo;
use village_kernel::kernel::traits::vk_kernel::*;
use village_kernel::kernel::impls::vk_village;

#[unsafe(no_mangle)]
pub fn init() {
    init_kernel(&vk_village::KERNEL_INSTANCE);
}

#[unsafe(no_mangle)]
pub fn main() -> ! {
    let kernel = kernel();
    kernel.setup();
    kernel.start();
    kernel.exit();
    kernel.debug().info("hello village kernel");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
