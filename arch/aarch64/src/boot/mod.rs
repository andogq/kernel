use core::arch::{asm, naked_asm};

use crate::{Aarch64, Aarch64Config};

#[allow(no_mangle_generic_items)]
impl<Config: Aarch64Config> Aarch64<Config> {
    const BOOT_CORE_ID: usize = Config::BOOT_CORE_ID;

    /// Start procedure to be immediately called in order to begin booting the core.
    ///
    /// # Safety
    ///
    /// Implemented using naked assembly, which directly interacts with registers within the core.
    /// Must only be called at the *very beginning* of the boot process. Also, it must be the only
    /// item with the `_start` symbol, in order for the linker script to correctly find it.
    #[naked]
    #[no_mangle]
    pub unsafe extern "C" fn _start() -> ! {
        naked_asm!(
            include_str!("boot.s"),
            CONST_CURRENTEL_EL2 = const 0x8,
            CONST_CORE_ID_MASK = const 0b11,
            CONST_BOOT_CORE_ID = const Self::BOOT_CORE_ID,
        )
    }

    /// Entry point for Rust.
    ///
    /// # Safety
    ///
    /// Requires memory, including the stack, to be correctly configured.
    #[no_mangle]
    pub unsafe extern "C" fn _start_rust() -> ! {
        loop {
            asm!("nop");
            asm!("nop");
            asm!("nop");
        }
    }
}
