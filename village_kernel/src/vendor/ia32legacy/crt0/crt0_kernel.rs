

unsafe extern "Rust" {
    unsafe fn main() -> !;
}

unsafe extern "Rust" {
    unsafe fn init();
}

pub type InterruptHandler = extern "C" fn() -> !;

#[used]
#[unsafe(link_section = ".isr_vector")]
pub static G_PFN_VECTORS: [InterruptHandler; 1] = [
    _start,
];

// _start 入口点
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unsafe {
        init();
        main();
    }
}
