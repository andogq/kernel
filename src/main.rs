#![no_std]
#![no_main]

mod logging;

use crate::logging::KernelLogger;
use lib_kernel::{Arch as _, Bsp as BspTrait, RawFunction};
use log::info;
use rpi3::{Rpi3, Rpi3Config};

/// Configuration object so that a pointer to `kernel_main` can be passed as a type parameter to
/// the BSP.
struct Config;
impl Rpi3Config for Config {
    const KERNEL_MAIN: fn() -> ! = kernel_main;
}

/// Type of the BSP used in this compilation.
type Bsp = Rpi3<Config>;

/// Instance of the BSP with all of it's state.
static BSP: Bsp = Bsp::new();

pub static LINKER_FUNCTIONS: &[RawFunction] = <Bsp as BspTrait>::Arch::LINKER_FUNCTIONS;

pub fn kernel_main() -> ! {
    // Ensure the board is initialised.
    BSP.initialise();

    // Configure the global logger
    KernelLogger::init();

    info!("Kernel starting");

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
