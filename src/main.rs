#![no_std]
#![no_main]

use core::arch::global_asm;

mod lang_items;
mod console;
mod sbi;

global_asm!(include_str!("entry.S"));

#[no_mangle]
fn kernel_main() {
    println!("Hello, world!");
    panic!();
}
