#![no_std]
#![no_main]

use core::panic::PanicInfo;

core::arch::global_asm!(
    ".section .text.init",
    ".globl _start",
    "_start:",
    // **load address** of **__stack_top** symbol into the **sp** register
    "  la sp, __stack_top",
    // **jump** to the **rust_main** symbol
    "  j rust_main",
);

#[unsafe(no_mangle)]
extern "C" fn rust_main() -> ! {
    loop {
        //
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
