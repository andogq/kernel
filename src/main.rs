#![no_std]
#![no_main]

use rpi3::Arch;

/// Instance of the architecture running the kernel.
#[allow(dead_code)]
pub static ARCH: Arch = Arch::new();

/// Re-export the `_start` symbol in order for the linker to pick it up.
///
/// **Note:** The implementation of `_start` *must* specify `#[no_mangle]`, as it is not possible
/// to add the attribute here.
#[allow(non_upper_case_globals)]
pub static _start: unsafe extern "C" fn() -> ! = Arch::_start;

/// Re-export the `_start_rust` symbol in order for the linker to pick it up.
///
/// **Note:** The implementation of `_start_rust` *must* specify `#[no_mangle]`, as it is not
/// possible to add the attribute here.
#[allow(non_upper_case_globals)]
pub static _start_rust: unsafe extern "C" fn() -> ! = Arch::_start_rust;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
