#![no_main]
#![no_std]

use uefi::boot::{MemoryDescriptor, MemoryType};

#[repr(C)]
pub struct BootInfo {
    pub mmap_ptr: u64,
    pub mmap_len: usize,
    pub mmap_desc_size: usize,
}

// Write a single byte to COM1 (0x3F8)
fn serial_write_byte(b: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3F8u16,
            in("al") b,
        );
    }
}

fn serial_write(s: &str) {
    for b in s.bytes() {
        serial_write_byte(b);
    }
}
unsafe extern "C" {
    static mut __bss_start: u8;
    static mut __bss_end: u8;
}

fn serial_init() {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") 0x3F8 + 1, in("al") 0x00u8); // disable interrupts
        core::arch::asm!("out dx, al", in("dx") 0x3F8 + 3, in("al") 0x80u8); // enable DLAB
        core::arch::asm!("out dx, al", in("dx") 0x3F8 + 0, in("al") 0x03u8); // divisor low (38400 baud)
        core::arch::asm!("out dx, al", in("dx") 0x3F8 + 1, in("al") 0x00u8); // divisor high
        core::arch::asm!("out dx, al", in("dx") 0x3F8 + 3, in("al") 0x03u8); // 8 bits, no parity
        core::arch::asm!("out dx, al", in("dx") 0x3F8 + 2, in("al") 0xC7u8); // FIFO
        core::arch::asm!("out dx, al", in("dx") 0x3F8 + 4, in("al") 0x0Bu8); // IRQs enabled
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(boot_info: *const BootInfo) -> ! {
    serial_init();
    serial_write("Kernel started!\n");

    let info = unsafe { &*boot_info };

    serial_write("Memory map entries: ");
    print_num(info.mmap_len);
    serial_write("\n");

    let mut ptr = info.mmap_ptr as *const MemoryDescriptor;

    for _ in 0..info.mmap_len {
        let desc = unsafe { &*ptr };

        if desc.ty == MemoryType::CONVENTIONAL {
            serial_write("  Usable RAM: ");
            print_hex(desc.phys_start);
            serial_write(" pages=");
            print_num(desc.page_count as usize);
            serial_write("  (");
            print_num((desc.page_count * 4096) as usize);
            serial_write(" bytes)\n");
        }

        ptr = unsafe { (ptr as *const u8).add(info.mmap_desc_size) as *const MemoryDescriptor };
    }

    serial_write("Done!\n");
    loop {}
}

fn print_hex(mut n: u64) {
    serial_write("0x");
    if n == 0 {
        serial_write("0");
        return;
    }
    let mut buf = [0u8; 16];
    let mut i = 16;
    while n > 0 {
        i -= 1;
        let nibble = (n & 0xF) as u8;
        buf[i] = if nibble < 10 {
            b'0' + nibble
        } else {
            b'a' + nibble - 10
        };
        n >>= 4;
    }
    for &b in &buf[i..] {
        serial_write_byte(b);
    }
}

fn print_num(mut n: usize) {
    if n == 0 {
        serial_write("0");
        return;
    }
    let mut buf = [0u8; 20];
    let mut i = 20;
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    for &b in &buf[i..] {
        serial_write_byte(b);
    }
}

