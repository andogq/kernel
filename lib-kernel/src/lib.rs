#![no_std]

pub trait Bsp {
    type Arch: Arch;
}

pub trait Arch {
    const START: unsafe extern "C" fn() -> !;
    const START_RUST: unsafe extern "C" fn() -> !;
}
