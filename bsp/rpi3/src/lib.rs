#![no_std]

use core::{fmt::Write, marker::PhantomData};

use aarch64::{Aarch64, Aarch64Config};
use lib_kernel::Bsp;
use pl011::{Initialised, Pl011};
use spin::mutex::SpinMutex;

const PL011_ADDRESS: usize = 0x3F201000;
type Uart = Pl011<PL011_ADDRESS, Initialised>;

/// Instance of this BSP. Config is used as a generic paramter so that it can be evaluated at
/// compile time.
pub struct Rpi3<Config> {
    _config: PhantomData<Config>,

    uart: SpinMutex<Option<Uart>>,
}

impl<C: Rpi3Config> Rpi3<C> {
    /// Create a new instance of the board.
    pub const fn new() -> Self {
        Self {
            _config: PhantomData,
            uart: SpinMutex::new(None),
        }
    }
}

/// Provide required information to the kernel by implementing the [`Bsp`] trait.
impl<C: Rpi3Config> Bsp for Rpi3<C> {
    type Arch = Aarch64<ArchConfig<C>>;

    fn initialise(&self) {
        // TODO: Verify that board hasn't already been initialised
        let mut uart = self.uart.lock();

        *uart = Some(Uart::new().initialise());
    }

    fn with_debug_console<F, T>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&mut dyn Write) -> T,
    {
        // Use the PL011 peripheral as a debug console.
        let mut guard = self.uart.lock();

        Some(f(guard.as_mut()?))
    }
}

impl<C: Rpi3Config> Default for Rpi3<C> {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration required for this BSP.
pub trait Rpi3Config {
    /// Entry point to the kernel, which will be called once the device has booted.
    const KERNEL_MAIN: fn() -> !;
}

/// Configuration for the Aarch64 core suitable to run on this board.
pub struct ArchConfig<Rpi3Config> {
    _config: PhantomData<Rpi3Config>,
}
impl<C: Rpi3Config> Aarch64Config for ArchConfig<C> {
    const BOOT_CORE_ID: usize = 0;
    const KERNEL_MAIN: fn() -> ! = C::KERNEL_MAIN;
}
