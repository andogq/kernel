#![no_std]
#![no_main]

use lib_kernel::{Arch as _, Bsp as BspTrait};
use rpi3::{Rpi3, Rpi3Config};

/// Configuration object so that a pointer to `kernel_main` can be passed as a type parameter to
/// the BSP.
struct Config;
impl Rpi3Config for Config {
    const KERNEL_MAIN: fn() -> ! = kernel_main;
}

/// Type of the BSP used in this compilation.
type Bsp = Rpi3<Config>;

/// Instance of the BSP.
static BSP: Bsp = Bsp::new();

/// Re-export the `_start` symbol in order for the linker to pick it up.
///
/// **Note:** The implementation of `_start` *must* specify `#[no_mangle]`, as it is not possible
/// to add the attribute here.
#[allow(non_upper_case_globals)]
pub static _start: unsafe extern "C" fn() -> ! = <Bsp as BspTrait>::Arch::START;

/// Re-export the `_start_rust` symbol in order for the linker to pick it up.
///
/// **Note:** The implementation of `_start_rust` *must* specify `#[no_mangle]`, as it is not
/// possible to add the attribute here.
#[allow(non_upper_case_globals)]
pub static _start_rust: unsafe extern "C" fn() -> ! = <Bsp as BspTrait>::Arch::START_RUST;

pub fn kernel_main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
