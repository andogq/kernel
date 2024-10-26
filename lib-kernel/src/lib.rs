#![no_std]

/// All the required functionality that a board must provide to the kernel.
pub trait Bsp {
    /// Underlying CPU architecture of this board.
    type Arch: Arch;
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
