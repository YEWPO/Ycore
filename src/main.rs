#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate alloc;

use core::arch::global_asm;

use log::{info, debug};

mod config;
mod lang_items;
mod console;
mod sbi;
mod logger;
mod mm;
mod sync;

global_asm!(include_str!("entry.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    info!("Clearing bss section.");
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize).fill(0);
    }
    debug!("Cleared bss section.");
}

#[no_mangle]
fn kernel_main() {
    println!("Hello, YROS!");
    logger::init();

    clear_bss();
    mm::init();

    panic!();
}
