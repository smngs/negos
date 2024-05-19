use crate::{print, println};
use core::fmt::{self, Write};
use core::result::Result::Ok;
use crate::sbicall::sbi_call;

use alloc::string::String;

fn put_char(ch: char) {
    sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1);
}

fn delete_last_char() {
    // TODO; OpenSBI 経由でさわれるようにする
    putchar_uart('\u{8}');
    putchar_uart(' ');
    putchar_uart('\u{8}');
}

fn get_char() -> Option<char> {
    let res = sbi_call(0x01, 0x02, 0, 0, 0, 0, 0, 2);
    char::from_u32(res.error as u32)
}

use core::ptr;
const UART0_ADDR: usize = 0x10000000;

#[allow(dead_code)]
fn putchar_uart(ch: char) {
    let uart0 = UART0_ADDR as *mut u8;
    unsafe {
        ptr::write_volatile(uart0, ch as u8);
    }
}


pub fn read_line() -> String {
    let mut buffer = String::new();
    loop {
        if let Some(c) = get_char() {
            if c == '\n' || c == '\r' { 
                break
            } else if c == '\u{7f}' {
                delete_last_char();
                buffer.pop();
            } else {
                print!("{}", c);
                buffer.push(c);
            }
        }
    }
    buffer
}

// fmt::println の実装
// cf. https://tomoyuki-nakabayashi.github.io/embedded-rust-techniques/03-bare-metal/print.html

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn _print(args: fmt::Arguments) {
    let mut writer = Writer {};
    writer.write_fmt(args).unwrap();
}

struct Writer;
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            put_char(c as char);
        }
        Ok(())
    }
}
