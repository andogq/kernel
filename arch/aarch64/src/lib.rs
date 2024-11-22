#![no_std]
#![feature(naked_functions)]

mod boot;
mod time;

use core::marker::PhantomData;

use lib_kernel::Arch;

/// Configuration that a BSP must provide if it relies on the Aarch64 architecture.
pub trait Aarch64Config {
    /// ID of the boot core.
    const BOOT_CORE_ID: usize;

    /// Entry point for the kernel to be called once the device has booted.
    const KERNEL_MAIN: fn() -> !;
}

/// Core structure to contain all state of this architecture.
///
/// This is intended to avoid linker shenanigans and statics spread everywhere. Ideally this can
/// provide a typed (such as the config) namespace for everything to be contained within.
pub struct Aarch64<Config> {
    _config: PhantomData<Config>,
}

impl<C: Aarch64Config> Arch for Aarch64<C> {
    const LINKER_FUNCTIONS: &[unsafe extern "C" fn() -> !] = &[_start, Self::_start_rust];
}

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    bring_up::entry();
}
