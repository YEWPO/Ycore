#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

use log::info;

mod lang_items;
mod console;
mod sbi;
mod logger;

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
}

#[no_mangle]
fn kernel_main() {
    println!("Hello, YROS!");
    logger::init();

    clear_bss();

    panic!();
}
