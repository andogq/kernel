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

## AArch64 MMU Configuration

### `MAIR_ELx`

Memory attributes are defined in the `MAIR_ELx` register (Memory Attribute Indirection Register).
The register consists 8x 1-byte 'attributes', each of which can be used to specify a different set 
of memory attributes. A specific attribute can then be referred to by index in the table or page
descriptors when writing them to memory.

### Memory Attributes

_(reference values [Redox kernel MAIR])_
_(full specification in ARM ARM D19.2.100)_

- `0b00`: `0b0000_0000`

  - Rule `0b0000dd00` where `dd` = `00`

  - Device memory (device-nGnRnE memory)

- `0b01`: `0b0100_0100`

  - Rule `0boooo_iiii` where `oooo` = `0100` and `iiii` = `0100`

  - Normal memory (Outer Non-cacheable, Inner None-cacheable)

- `0b10`: `0b1111_1111`

  - Rule `0b0000_iiii` where `oooo` = `11RW` and `iiii` = `11RW`

  - Normal memory (Outer Write-Back Non-transient, Inner Write-Back Non-transient)

### Memory Regions

- Kernel text: `0b01` (normal non-cacheable)

  - It's standard to not cache kernel text in order to ensure the 'latest' code version is always
    being read (eg incase it's updated). It also apparently assists with preventing side-channel
    attacks?

- Kernel stack: `0b10` (normal, cacheable)

  - Kernel stack is fine to be cached as it will receive frequent reads and writes. Caching it
    reduces activity on the memory busses.

- Device memory map: `0b00` (device memory)

  - Since this is just the device's entire RAM re-mapped at a higher address, it's best to limit
    additional caching since it will contain memory mapped devices.

---

[Redox]: https://gitlab.redox-os.org/redox-os/kernel/-/blob/ea0356b26a753f053def5c966c3af118b5e6dfaa/src/arch/aarch64/init/pre_kstart/helpers/post_mmu_enabled.S#L63
[Redox Linker Script]: https://gitlab.redox-os.org/redox-os/kernel/-/blob/ea0356b26a753f053def5c966c3af118b5e6dfaa/linkers/aarch64.ld
[Redox kernel MAIR]: https://gitlab.redox-os.org/redox-os/kernel/-/blob/ea0356b26a753f053def5c966c3af118b5e6dfaa/src/arch/aarch64/init/pre_kstart/helpers/pre_mmu_enabled.S#L60
