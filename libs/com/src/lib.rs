#![no_std]

use core::fmt;

const COM1: u16 = 0x3F8;

pub struct SerialPort;

impl SerialPort {
    pub fn init() {
        let com = COM1;
        outb(com + 1, 0x00); // Disable interrupts
        outb(com + 3, 0x80); // Enable DLAB
        outb(com + 0, 0x03); // 38400 baud
        outb(com + 1, 0x00);
        outb(com + 3, 0x03); // 8 bits, no parity
        outb(com + 2, 0xC7); // FIFO
        outb(com + 4, 0x0B); // IRQs enabled, RTS/DSR
        outb(com + 1, 0x00); // Disable interrupts again
    }

    pub fn write_byte(b: u8) {
        while (inb(COM1 + 5) & 0x20) == 0 {}
        outb(COM1, b);
    }
}

fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") val);
    }
}

fn inb(port: u16) -> u8 {
    let val: u8;
    unsafe {
        core::arch::asm!("in al, dx", in("dx") port, out("al") val);
    }
    val
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            Self::write_byte(byte);
        }
        Ok(())
    }
}

pub fn serial() -> SerialPort {
    SerialPort
}
pub fn print_hex(n: u64) {
    serial_println!("0x{:x}", n);
}

pub fn print_num(n: usize) {
    serial_println!("{}", n);
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = $crate::serial().write_fmt(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial_print!("\n")
    };
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = $crate::serial().write_fmt(format_args!("{}\n", format_args!($($arg)*)));
    }};
}

#[doc(hidden)]
#[allow(dead_code)]
fn serial_print_inner(args: fmt::Arguments) {
    use fmt::Write;
    let _ = serial().write_fmt(args);
}
