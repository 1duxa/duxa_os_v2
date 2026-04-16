#![no_main]
#![no_std]

use com::{SerialPort, print_hex, print_num, serial_print, serial_println};
use uefi::boot::{MemoryDescriptor, MemoryType};

#[repr(C)]
pub struct BootInfo {
    pub mmap_ptr: u64,
    pub mmap_len: usize,
    pub mmap_desc_size: usize,
}
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(boot_info: *const BootInfo) -> ! {
    SerialPort::init();
    serial_print!("Kernel started!\n");

    let info = unsafe { &*boot_info };

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
