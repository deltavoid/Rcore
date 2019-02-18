use core::fmt::{Write, Result, Arguments};
use super::sbi;

struct SerialPort;

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.bytes() {
            if c == 127 {
                putchar(8);
                putchar(b' ');
                putchar(8);
            } else {
                putchar(c);
            }
        }
        Ok(())
    }
}

fn putchar(c: u8) {
    sbi::console_putchar(c as usize);
}

pub fn getchar() -> char {
    let c = sbi::console_getchar() as u8;
    match c {
        255 => '\0',   // null
        c => c as char,
    }
}

pub fn getchar_option() -> Option<char> {
    let c = sbi::console_getchar() as isize;
    match c {
        -1 => None,
        c => Some(c as u8 as char),
    }
}

pub fn putfmt(fmt: Arguments) {
    SerialPort.write_fmt(fmt).unwrap();
}
