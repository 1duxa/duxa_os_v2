#![no_main]
#![no_std]

use com::{SerialPort, print_hex, print_num, serial_print, serial_println};
use uefi::boot::{MemoryDescriptor, MemoryType};
use uefi_bootinfo::BootInfo;

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(boot_info: *const BootInfo) -> ! {
    SerialPort::init();
    serial_print!("Kernel started!\n");

    let info = unsafe { &*boot_info };
    let kernel_start = unsafe { &_kernel_start as *const _ as u64 };
    let kernel_end = unsafe { &_kernel_end as *const _ as u64 };
    serial_println!("Kernel starts at: ");
    print_hex(kernel_start);
    serial_println!("Kernel ends at: ");
    print_hex(kernel_end);

    serial_println!("Memory map entries: ");
    print_num(info.mmap_len);
    serial_print!("\n");

    let mut ptr = info.mmap_ptr as *const MemoryDescriptor;

    for _ in 0..info.mmap_len {
        let desc = unsafe { &*ptr };

        if desc.ty == MemoryType::CONVENTIONAL {
            serial_println!("  Usable RAM: ");
            print_hex(desc.phys_start);
            serial_println!(" pages=");
            print_num(desc.page_count as usize);
            serial_println!("  (");
            print_num((desc.page_count * 4096) as usize);
            serial_println!(" bytes)\n");
        }

        ptr = unsafe { (ptr as *const u8).add(info.mmap_desc_size) as *const MemoryDescriptor };
    }

    serial_println!("Done!\n");
    loop {}
}
