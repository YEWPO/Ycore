use core::arch::global_asm;

use self::handler::set_kernel_trap_entry;

mod context;
mod handler;

global_asm!(include_str!("trap.S"));

pub fn init() {
    set_kernel_trap_entry();
}
