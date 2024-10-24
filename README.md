# Configuration

The plan is two base kernel compilation around a specific 'board support package' (BSP), located in
`/bsp`. The BSP will provide a specific architecture implementation, in conjunction with required
devices, memory maps, and linker scripts to generate an appropriate binary to run on the target.

Currently this is hard-coded in `.cargo/config.toml`:

- `build.target`: The target triple to compile for (eg `aarch64-unknown-none-softfloat`)
- `build.rustflags`: Additional flags to control the linker for this target
  - `link-arg=--library-path`: An additional path for the linker to search, which should point to
    the BSP directory (eg `./bsp/rpi3`)
  - `link-arg=--script`: Name of the linker script, which will likely remain the same across
    multiple targets (eg `kernel.ld`)

