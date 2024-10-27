#![no_std]

use core::fmt::Write;

/// All the required functionality that a board must provide to the kernel.
pub trait Bsp {
    /// Underlying CPU architecture of this board.
    type Arch: Arch;

    /// Hook for the board to perform initialisation when the kernel is ready for it.
    ///
    /// This may be useful to setup core devices on the board.
    fn initialise(&self) {}

    /// Run a closure with the debug console.
    ///
    /// If this board does not have a debug console, then the closure will not run, and [`None`]
    /// will be returned. Otherwise the result of the closure will be returned.
    fn with_debug_console<F, T>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&mut dyn Write) -> T;
}

/// Alias for a function with C FFI that takes no parameters and will never return to the caller.
pub type RawFunction = unsafe extern "C" fn() -> !;

/// All required functionality that an architecture must provide to the kernel.
pub trait Arch {
    /// Functions to expose to the linker.
    ///
    /// For best effect, each function should be annotated with `#[no_mangle]`.
    const LINKER_FUNCTIONS: &[RawFunction];
}
