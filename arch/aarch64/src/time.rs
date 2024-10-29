use aarch64_cpu::{asm, registers::*};
use core::time::Duration;
use uom::si::{
    f64::{Frequency, Ratio},
    frequency::hertz,
    ratio::ratio,
};

use crate::{Aarch64, Aarch64Config};

impl<C: Aarch64Config> Aarch64<C> {
    /// Returns the frequency in Hz.
    pub fn frequency() -> Frequency {
        // NOTE: Although a 64 bit register, only bits [31:0] contain the frequency.
        Frequency::new::<hertz>((CNTFRQ_EL0.get() & (u32::MAX as u64)) as f64)
    }

    pub fn uptime() -> Duration {
        asm::barrier::isb(asm::barrier::SY);

        let count = CNTPCT_EL0.get();

        let count = Ratio::new::<ratio>(count as f64);
        let frequency = Self::frequency();

        Duration::try_from(count / frequency).unwrap()
    }
}
