use aarch64::{Aarch64, Aarch64Config};

/// Configuration for the Aarch64 core suitable to run on this board.
pub struct Config;
impl Aarch64Config for Config {
    const BOOT_CORE_ID: usize = 0;
}

pub type Arch = Aarch64<Config>;
