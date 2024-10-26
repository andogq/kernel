#![no_std]
#![no_main]

use core::fmt::Write;
use lib_kernel::{Arch as _, Bsp as BspTrait, RawFunction};
use pl011::Pl011;
use rpi3::{Rpi3, Rpi3Config};

/// Configuration object so that a pointer to `kernel_main` can be passed as a type parameter to
/// the BSP.
struct Config;
impl Rpi3Config for Config {
    const KERNEL_MAIN: fn() -> ! = kernel_main;
}

/// Type of the BSP used in this compilation.
type Bsp = Rpi3<Config>;

pub static LINKER_FUNCTIONS: &[RawFunction] = <Bsp as BspTrait>::Arch::LINKER_FUNCTIONS;

pub fn kernel_main() -> ! {
    let pl011 = Pl011::<0x3F201000>::new();
    let mut pl011 = pl011.initialise();

    writeln!(pl011, "kernel starting").unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
