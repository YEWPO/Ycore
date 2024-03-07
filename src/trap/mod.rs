use core::arch::global_asm;

use log::info;
use riscv::register::sstatus;

use crate::{drivers::set_next_trigger, trap::handler::enable_timer_interrupt};

use self::handler::set_kernel_trap_entry;

mod context;
mod handler;

global_asm!(include_str!("trap.S"));

pub fn init() {
    info!("Initalizing kernel trap.");
    set_kernel_trap_entry();
    enable_timer_interrupt();
    set_next_trigger();
    unsafe { sstatus::set_sie() };
}
