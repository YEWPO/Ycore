use core::panic::PanicInfo;

use log::error;

use crate::{sbi::shutdown, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info.message().unwrap());

    if let Some(location) = info.location() {
        println!("Panic at {}: {} {}", location.file(), location.line(), info.message().unwrap());
    } else {
        println!("Panic: {}", info.message().unwrap());
    }
    shutdown(true)
}
