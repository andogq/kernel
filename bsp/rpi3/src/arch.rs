use core::marker::PhantomData;

use aarch64::Aarch64Config;

use crate::Rpi3Config;

/// Configuration for the Aarch64 core suitable to run on this board.
pub struct Config<Rpi3Config> {
    _config: PhantomData<Rpi3Config>,
}
impl<C: Rpi3Config> Aarch64Config for Config<C> {
    const BOOT_CORE_ID: usize = 0;
    const KERNEL_MAIN: fn() -> ! = C::KERNEL_MAIN;
}
