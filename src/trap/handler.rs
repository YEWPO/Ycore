use core::arch::asm;

use log::{debug, trace};
use riscv::register::{scause::{self, Exception, Interrupt, Trap}, sie, sscratch, sstatus, stval, stvec::{self, TrapMode}};

use crate::{config::TRAMPOLINE, drivers::set_next_trigger};

extern "C" {
    fn __alltraps();
    fn __restore();
    fn __alltraps_k();
    fn __restore_k();
}

pub fn set_kernel_trap_entry() {
    let __alltraps_k_va = __alltraps_k as usize - __alltraps as usize + TRAMPOLINE;

    debug!("set kernel trap entry!");
    unsafe {
        stvec::write(__alltraps_k_va, TrapMode::Direct);
        sscratch::write(trap_from_kernel as usize);
    }
}

fn set_user_trap_entry() {
    debug!("set user trap entry!");
    unsafe { stvec::write(TRAMPOLINE, TrapMode::Direct) };
}

pub fn enable_timer_interrupt() {
    trace!("enable timer interrupt.");
    unsafe { sie::set_stimer() };
}

fn enable_supervisor_interrupt() {
    trace!("enable supervisor interrupt.");
    unsafe { sstatus::set_sie() };
}

fn disable_supervisor_interrupt() {
    trace!("disable supervisor interrupt.");
    unsafe { sstatus::clear_sie() };
}


#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();

    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            enable_supervisor_interrupt();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            debug!("timer interrupt!");
            set_next_trigger();
        }
        _ => {
            panic!("Unsupported trap {:?} from user, stval = {:#x}!", scause.cause(), stval);
        }
    }

    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    disable_supervisor_interrupt();
    set_user_trap_entry();

    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;

    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_from_kernel() {
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            debug!("timer interrupt!");
            set_next_trigger();
        }
        _ => {
            panic!("Unsupported trap {:?} from kernel, stval = {:#x}", scause.cause(), stval);
        }
    }
}
