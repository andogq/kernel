#![no_std]
#![no_main]

mod logging;

use crate::logging::KernelLogger;
use lib_kernel::{Arch as _, Bsp as BspTrait, RawFunction};
use log::{error, info};
use rpi3::{Rpi3, Rpi3Config};
use uom::{fmt::DisplayStyle, si::frequency::megahertz};

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

    info!(
        "Counter running at {}",
        <Bsp as lib_kernel::Bsp>::Arch::frequency()
            .into_format_args(megahertz, DisplayStyle::Abbreviation),
    );

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("==== Panic occurred! ====");

    if let Some(location) = info.location() {
        error!("Location: {}", location);
    }

    error!("{}", info.message());

    loop {}
}
