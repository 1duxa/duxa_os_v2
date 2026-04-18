#![no_main]
#![no_std]

use com::{SerialPort, serial_print, serial_println};
use constants::size::KiB;
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
    let kernel_real_start = unsafe { &_kernel_start as *const _ as u64 };
    let kernel_real_end = unsafe { &_kernel_end as *const _ as u64 };

    serial_println!("Kernel starts at: 0x{:x}", kernel_real_start);
    serial_println!("Kernel ends at: 0x{:x}", kernel_real_end);
    serial_println!("Kernel stack top at: 0x{:x}", info.stack_top);
    serial_println!("Memory map entries: {}", info.mmap_len);

    let mut ptr = info.mmap_ptr as *const MemoryDescriptor;
    let mut usable_len = 0;
    for _ in 0..info.mmap_len {
        let desc = unsafe { &*ptr };

        if desc.ty == MemoryType::CONVENTIONAL {
            serial_print!("Usable RAM: 0x{:x}", desc.phys_start);
            serial_print!(" pages= {}", desc.page_count as usize);
            serial_print!(" ({}) bytes\n", (desc.page_count * KiB) as usize);
            usable_len += 1;
        }

        ptr = unsafe { (ptr as *const u8).add(info.mmap_desc_size) as *const MemoryDescriptor };
    }
    unsafe {
        let kernel_phys_end = kernel_real_end - 0xFFFFFFFF80000000 + 0x100000;

        let after_kernel =
            core::slice::from_raw_parts_mut(kernel_phys_end as *mut MemoryDescriptor, usable_len);
        let mut ptr = info.mmap_ptr as *const MemoryDescriptor;
        let mut count = 0;
        for _ in 0..info.mmap_len {
            let desc = &*ptr;

            if desc.ty == MemoryType::CONVENTIONAL {
                after_kernel[count] = *desc;
                count += 1;
            }

            ptr = (ptr as *const u8).add(info.mmap_desc_size) as *const MemoryDescriptor;
        }
    }

    unsafe {
        let p4 = info.kernel_p4_addr as *mut u64;
        *p4 = 0;

        core::arch::asm!(
            "mov rax, cr3",
            "mov cr3, rax",
            out("rax") _,
        );
    }
    loop {}
}
