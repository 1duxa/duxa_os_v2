#![no_main]
#![no_std]

use core::arch::asm;

use com::{SerialPort, print_hex, print_num, serial_print, serial_println};
use constants::{
    page_flags::{HUGE, PRESENT, WRITABLE},
    size::{KiB, MiB},
};
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

fn enable_pae() {
    let mut cr4: u64;
    unsafe {
        asm!("mov {}, cr4", out(reg) cr4);
        cr4 |= 1 << 5;
        asm!("mov cr4, {}", in(reg) cr4);
    }
}

fn enable_paging() {
    let mut cr0: u64;
    unsafe {
        asm!("mov {}, cr0", out(reg) cr0);
        cr0 |= 1 << 31;
        asm!("mov cr0, {}", in(reg) cr0);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(boot_info: *const BootInfo) -> ! {
    SerialPort::init();
    serial_print!("Kernel started!\n");

    let info = unsafe { &*boot_info };
    let kernel_start = unsafe { &_kernel_start as *const _ as u64 };
    let kernel_end = unsafe { &_kernel_end as *const _ as u64 };

    unsafe {
        for i in 0..512 {
            P2.0[i] = (i as u64 * 2 * MiB) | PRESENT | WRITABLE | HUGE;
        }
    }

    unsafe {
        P3.0[0] = &raw const P2 as u64 | PRESENT | WRITABLE;
    }

    unsafe {
        P4.0[0] = &raw const P3 as u64 | PRESENT | WRITABLE;
    }

    enable_pae();
    load_cr3(&raw const P4);
    enable_paging();

    serial_println!("Kernel starts at: ");
    print_hex(kernel_start);

    serial_println!("Kernel ends at: ");
    print_hex(kernel_end);

    serial_println!("Kernel stack top at: ");
    print_hex(info.stack_top);

    serial_println!("Memory map entries: ");
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
