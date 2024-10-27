use crate::{Bsp, BSP};
use lib_kernel::Bsp as _;

/// Helper struct to contain all logging-related functionality. Uses the
/// [`lib_kernel::Bsp::with_debug_console`] method to provide logging to whatever device is most
/// appropriate.
pub struct KernelLogger;

impl KernelLogger {
    /// Setup [`log`] to use this logger. Also sets the logging level to `trace`.
    pub fn init() {
        log::set_logger(&KernelLogger).unwrap();

        log::set_max_level(log::LevelFilter::Trace);
    }
}

impl log::Log for KernelLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // TODO: Check if logger is registered
        true
    }

    fn log(&self, record: &log::Record) {
        let Some(result) = BSP.with_debug_console(|w| {
            if !self.enabled(record.metadata()) {
                return Ok(());
            }

            let timestamp = <Bsp as lib_kernel::Bsp>::Arch::uptime().as_secs_f64();

            writeln!(
                w,
                "[{}] {timestamp:10.4}: {}",
                record.level(),
                record.args()
            )
        }) else {
            return;
        };

        // Unwrap the result out of the lock, so there's no deadlock
        result.unwrap()
    }

    fn flush(&self) {}
}
