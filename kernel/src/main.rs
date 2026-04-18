#![no_main]
#![no_std]

use core::arch::asm;

use com::{SerialPort, print_hex, print_num, serial_print, serial_println};
use constants::size::KiB;
use page_table::PageTable;
use uefi::boot::{MemoryDescriptor, MemoryType};
use uefi_bootinfo::BootInfo;

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

static mut P1: PageTable = PageTable([0; 512]);
static mut P2: PageTable = PageTable([0; 512]);
static mut P3: PageTable = PageTable([0; 512]);
static mut P4: PageTable = PageTable([0; 512]);

fn load_cr3(p4: *const PageTable) {
    let phys = p4 as u64;
    unsafe {
        asm!(
            "mov cr3, {}",
            in(reg) phys,
            options(nostack, preserves_flags)
        );
    }
}
fn print_hex_raw(n: u64) {
    serial_print!("0x");
    for i in (0..16).rev() {
        let nibble = ((n >> (i * 4)) & 0xF) as u8;
        let c = if nibble < 10 {
            b'0' + nibble
        } else {
            b'a' + nibble - 10
        };
        SerialPort::write_byte(c);
    }
    SerialPort::write_byte(b'\n');
}
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(boot_info: *const BootInfo) -> ! {
    SerialPort::init();
    serial_print!("Kernel started!\n");

    let info = unsafe { &*boot_info };
    let kernel_real_start = unsafe { &_kernel_start as *const _ as u64 };
    let kernel_real_end = unsafe { &_kernel_end as *const _ as u64 };

    serial_println!("Kernel starts at: ");
    print_hex(kernel_real_start);

    serial_println!("\nKernel ends at: ");
    print_hex(kernel_real_end);

    serial_println!("\nKernel stack top at: ");
    print_hex(info.stack_top);

    serial_println!("\nMemory map entries: ");
    print_num(info.mmap_len);
    serial_print!("\n");

    let mut ptr = info.mmap_ptr as *const MemoryDescriptor;

    for _ in 0..info.mmap_len {
        let desc = unsafe { &*ptr };

        if desc.ty == MemoryType::CONVENTIONAL {
            serial_print!("Usable RAM: ");
            print_hex(desc.phys_start);
            serial_print!("\n pages=");
            print_num(desc.page_count as usize);

            serial_print!(" (");
            print_num((desc.page_count * KiB) as usize);
            serial_println!(" bytes)\n");
        }

        ptr = unsafe { (ptr as *const u8).add(info.mmap_desc_size) as *const MemoryDescriptor };
    }

    loop {}
}
