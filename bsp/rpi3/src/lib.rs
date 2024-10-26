#![no_std]

use core::marker::PhantomData;

use aarch64::{Aarch64, Aarch64Config};
use lib_kernel::Bsp;

/// Instance of this BSP. Config is used as a generic paramter so that it can be evaluated at
/// compile time.
pub struct Rpi3<Config> {
    _config: PhantomData<Config>,
}

/// Provide required information to the kernel by implementing the [`Bsp`] trait.
impl<C: Rpi3Config> Bsp for Rpi3<C> {
    type Arch = Aarch64<ArchConfig<C>>;
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
