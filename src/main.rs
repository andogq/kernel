#![no_std]
#![no_main]

// pub use aarch64::_start;

pub use aarch64::*;

struct MyInfo;
impl Info for MyInfo {
    const BOOT_CORE_ID: usize = 0;
}

static ARCH: S<MyInfo> = S::new();

#[allow(non_upper_case_globals)]
pub static _start: unsafe extern "C" fn() -> ! = S::<MyInfo>::_start;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
