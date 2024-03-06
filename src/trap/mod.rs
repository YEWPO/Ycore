use core::arch::global_asm;

mod context;

global_asm!(include_str!("trap.S"));
