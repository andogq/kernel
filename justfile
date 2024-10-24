# Configuration
target := "aarch64-unknown-none-softfloat"
binary_name := "kernel.bin"

# Helpers
elf_path := "target" / target / "debug/kernel"

# Compile and dump the kernel binary
build: compile dump-binary

# Compile the ELF of the kernel.
compile:
    cargo rustc

# Dump the binary from an existing ELF.
dump-binary:
    rust-objcopy --strip-all -O binary {{elf_path}} {{binary_name}}

# Clean the workspace, including removing the final binary.
clean:
    cargo clean
    rm -f {{binary_name}}

# Run the binary in qemu.
#
# If `mode` is `debug`, QEMU will be started with the appropriate flags to attach GDB.
run mode="":
    qemu-system-aarch64 \
        -M raspi3b -kernel {{binary_name}} \
        -serial stdio -display none \
        {{ if mode == "debug" { "-S -s" } else { "" } }}

# Launch GDB with the kernel as a remote target
gdb:
    aarch64-elf-gdb -q \
        -ex 'target remote localhost:1234' \
        -ex 'load' \
        {{elf_path}}
