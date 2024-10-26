#![no_std]
#![feature(naked_functions)]

mod boot;

use core::marker::PhantomData;

/// Configuration that a BSP must provide if it relies on the Aarch64 architecture.
pub trait Aarch64Config {
    /// ID of the boot core.
    const BOOT_CORE_ID: usize;

    const KERNEL_MAIN: fn() -> !;
}

/// Core structure to contain all state of this architecture.
///
/// This is intended to avoid linker shenanigans and statics spread everywhere. Ideally this can
/// provide a typed (such as the config) namespace for everything to be contained within.
pub struct Aarch64<Config> {
    _config: PhantomData<Config>,
}

impl<C> Aarch64<C> {
    /// Create a new instance of the architecture with a specific config.
    pub const fn new<Config: Aarch64Config>() -> Aarch64<Config> {
        Aarch64 {
            _config: PhantomData,
        }
    }
}
