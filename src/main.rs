#![no_std]
#![no_main]

use core::arch::global_asm;

mod lang_items;

global_asm!(include_str!("entry.S"));

#[no_mangle]
fn kernel_main() {
    loop {}
}
