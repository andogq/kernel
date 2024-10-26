use core::{arch::naked_asm, cell::UnsafeCell};

use aarch64_cpu::{asm, registers::*};

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
    pub(crate) unsafe extern "C" fn _start() -> ! {
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
    pub(crate) unsafe extern "C" fn _start_rust() -> ! {
        // D19-6632: Configure hypervisor controller to enable aarch64 in EL1
        HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

        // Set up timer access for EL1
        Self::enable_el1_timers();

        // C5-800: Fake an exception return to enter EL1
        SPSR_EL2.write(
            SPSR_EL2::D::Masked
                + SPSR_EL2::A::Masked
                + SPSR_EL2::I::Masked
                + SPSR_EL2::F::Masked
                + SPSR_EL2::M::EL1h,
        );

        // Set the link address to return from the exception
        ELR_EL2.set(Config::KERNEL_MAIN as *const () as u64);

        extern "C" {
            static __boot_core_stack_end_exclusive: UnsafeCell<()>;
        }

        // Set up the EL1 stack to re-use the existing stack
        SP_EL1.set(__boot_core_stack_end_exclusive.get() as u64);

        // Perform the exception return
        asm::eret()
    }

    /// Configure access to timers and counters in EL1.
    ///
    /// # Safety:
    ///
    /// Caller must ensure that processor is already in EL2, otherwise timer configurations cannot
    /// be changed.
    unsafe fn enable_el1_timers() {
        // D19-7863: Disable traps for accessing EL1 counter and timer.
        CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

        // D19-7960: Clear timer offsets for the virtual timer.
        CNTVOFF_EL2.set(0);
    }
}
