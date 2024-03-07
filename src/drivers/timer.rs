use log::trace;
use riscv::register::time;
use sbi_rt::set_timer;

pub fn get_time() -> usize {
    time::read()
}

pub fn set_next_trigger() {
    trace!("set next trigger");
    set_timer(get_time() as u64 + 0x1000000);
}
