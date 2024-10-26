#![no_std]

mod arch;

use core::marker::PhantomData;

use aarch64::Aarch64;
use arch::Config;
use lib_kernel::Bsp;

pub struct Rpi3<Config> {
    _config: PhantomData<Config>,
}

impl<C: Rpi3Config> Rpi3<C> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            _config: PhantomData,
        }
    }
}

pub trait Rpi3Config {
    const KERNEL_MAIN: fn() -> !;
}

impl<C: Rpi3Config> Bsp for Rpi3<C> {
    type Arch = Aarch64<Config<C>>;
}
