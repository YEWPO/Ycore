use core::fmt::{Write, Result, Arguments};

use crate::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}

pub fn println(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
    Stdout.write_char('\n').unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($args: tt)+)?) => {
        crate::console::print(format_args!($fmt $(, $($args)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($args: tt)+)?) => {
        crate::console::println(format_args!($fmt $(, $($args)+)?));
    };
}
