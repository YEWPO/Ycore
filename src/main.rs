#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate alloc;

use core::arch::global_asm;

mod config;
mod lang_items;
mod console;
mod sbi;
mod logger;
mod mm;
mod sync;
mod trap;

global_asm!(include_str!("entry.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize).fill(0);
    }
}

#[no_mangle]
fn kernel_main() {
    clear_bss();

    println!("Hello, YROS!");
    logger::init();

    mm::init();
    trap::init();

    panic!();
}
