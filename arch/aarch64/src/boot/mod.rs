use core::arch::naked_asm;

#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!("2:", "nop", "nop", "wfe", "b 2b");
}
