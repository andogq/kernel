```sh
RUSTFLAGS="-C link-arg=--library-path=./bsp/rpi3 -C link-arg=--script=kernel.ld" cargo rustc --target=aarch64-unknown-none-softfloat
rust-objcopy --strip-all -O binary ./target/aarch64-unknown-none-softfloat/debug/kernel out
qemu-system-aarch64 -serial stdio -display none -M raspi3b -kernel ./out -S -s
```
