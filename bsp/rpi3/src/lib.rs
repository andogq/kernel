#![no_std]
#![feature(inherent_associated_types)]

mod arch;

use core::marker::PhantomData;

use aarch64::Aarch64;
use arch::Config;

pub struct Rpi3<Config> {
    _config: PhantomData<Config>,
}

impl<C: Rpi3Config> Rpi3<C> {
    pub type Arch = Aarch64<Config<C>>;

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
