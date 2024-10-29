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

# Pre-MMU enabled

1. Generate tables (maybe const with Rust?)
  - Identity map kernel (VA == PA, temporary until execution switches over to MMU)
  - Map high addresses to kernel
  - Map peripherals

2. Begin execution with PIC
  - Seperate Rust crate with `-pie` enabled
  - Assembly
  - TODO: How to configure linker script to place this code at the top of the binary? [Redox Linker
    Script]

3. PIC configures tables

4. PIC 'trampolines' into regular kernel setup, compiled for high execution

```
mmu_on_trampoline:
    adr     x0, mmu_on_marker               // x0: paddr of mmu_on_marker
    ldr     x0, [x0]                        // x0: vaddr of mmu_on
    br      x0                              // MMU now On. Jump to mmu_on using it's vaddr
```

> Copied from [Redox]

  - **Note:** This code is executing whilst the MMU is on, but the link return register will contain
    the 'old' address

  - `adr x0, mmu_on_marker`: Store an offset from the current location to the label `mmu_on_marker`
    (which contains the virtual memory address of the `mmu_on` label, inserted by the linker).

  - `ldr x0, [x0]`: Load from the offset stored in `x0` a value, and place it in `x0`. This fetches
    the virtual address of `mmu_on` stored by the linker.

  - `br x0`: Branch to `mmu_on` using the virtual address.

5. Identity mapped kernel is removed from tables (optional)

6. MMU is now enabled!

---

[Redox]: https://gitlab.redox-os.org/redox-os/kernel/-/blob/master/src/arch/aarch64/init/pre_kstart/helpers/post_mmu_enabled.S#L63
[Redox Linker Script]: https://gitlab.redox-os.org/redox-os/kernel/-/blob/master/linkers/aarch64.ld?ref_type=heads
