#![no_std]

/// All the required functionality that a board must provide to the kernel.
pub trait Bsp {
    /// Underlying CPU architecture of this board.
    type Arch: Arch;
}

/// All required functionality that an architecture must provide to the kernel.
pub trait Arch {
    /// Pointer to a function to be exposed as the `_start` symbol to the linker.
    ///
    /// **Note:** The function must be defined with `#[no_mangle]` in order to be correctly
    /// exposed.
    const START: unsafe extern "C" fn() -> !;

    /// Pointer to a function to be exposed to the linker.
    ///
    /// **Note:** The function must be defined with `#[no_mangle]` in order to be correctly
    /// exposed.
    const START_RUST: unsafe extern "C" fn() -> !;
}
