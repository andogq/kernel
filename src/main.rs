#![no_std]
#![no_main]

pub use aarch64::_start;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
