#![no_main]
#![no_std]

use addr::phys::PhysAddr;
use com::{SerialPort, serial_println};
use mem_info::{MEM_INFO, MemInfo};
use page_table::{entry_addr, phys_to_virt};
use physical_allocator::{FrameAllocator, PHYS_ALLOC};
use uefi_bootinfo::{BootInfo, MemoryDescriptor};

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}
#[unsafe(no_mangle)]
/**
    # Safety
    It's inherently unsafe
*/
pub unsafe extern "C" fn kernel_main(boot_info: *const BootInfo) -> ! {
    SerialPort::init();
    serial_println!("Kernel started!");

    let info = unsafe { &*boot_info };
    let kern_offset = info.kernel_virt_base.0 - info.kernel_phys_base.0;

    let kernel_virt_start = unsafe { &_kernel_start as *const _ as u64 };
    let kernel_virt_end = unsafe { &_kernel_end as *const _ as u64 };
    let kernel_phys_end = kernel_virt_end - kern_offset;

    serial_println!(
        "Kernel virt: 0x{:x} - 0x{:x}",
        kernel_virt_start,
        kernel_virt_end
    );
    serial_println!(
        "Kernel phys: 0x{:x} - 0x{:x}",
        info.kernel_phys_base.0,
        kernel_phys_end
    );
    serial_println!("Stack top: 0x{:x}", info.stack_top.0);
    serial_println!("PHYS_MAP: 0x{:x}", info.phys_map_base);

    // Print and count usable memory regions
    let mut ptr = info.mmap_ptr.0 as *const MemoryDescriptor;
    let mut usable_len = 0usize;
    serial_println!("Memory map ({} entries):", info.mmap_len);
    for _ in 0..info.mmap_len {
        let desc = unsafe { &*ptr };
        if desc.mem_type.is_conventional_memory() {
            serial_println!("{}", desc);
            usable_len += 1;
        }
        ptr = unsafe { (ptr as *const u8).add(info.mmap_desc_size) as *const MemoryDescriptor };
    }

    // Move stack into higher half before removing identity map
    unsafe {
        core::arch::asm!(
            "add rsp, {offset}",
            offset = in(reg) kern_offset,
        );
    }

    let phys_map = info.phys_map_base;
    unsafe {
        MEM_INFO = Some(MemInfo::init(
            info.mmap_ptr,
            info.mmap_len,
            info.mmap_desc_size,
            PhysAddr(kernel_phys_end),
            usable_len,
            phys_map,
        ));
    }

    // Save everything we need from info before removing identity map
    let p4_phys: PhysAddr = info.kernel_p4_addr;

    // Remove identity map (P4[0]) and flush TLB
    unsafe {
        let p4 = p4_phys.0 as *mut PhysAddr;
        (*p4).0 = 0;
        core::arch::asm!(
            "mov rax, cr3",
            "mov cr3, rax",
            out("rax") _,
        );
    }

    // Verify page table state via PHYS_MAP window
    let p4_virt = phys_to_virt(p4_phys, phys_map).0 as *const u64;
    unsafe {
        let e0 = *p4_virt;
        let e256 = *p4_virt.add(256);
        let e511 = *p4_virt.add(511);

        assert!(
            e0 & 1 == 0,
            "P4[0] still present - identity map not removed"
        );
        assert!(e256 & 1 != 0, "P4[256] not present - PHYS_MAP not mapped");
        assert!(e511 & 1 != 0, "P4[511] not present - kernel not mapped");

        serial_println!("P4[0]: 0x{:x} (identity, removed)", e0);
        serial_println!("P4[256]: 0x{:x} (PHYS_MAP)", entry_addr(e256));
        serial_println!("P4[511]: 0x{:x} (kernel)", entry_addr(e511));
    }

    serial_println!("Early boot complete.");

    unsafe {
        PHYS_ALLOC = Some(FrameAllocator::new(&raw const MEM_INFO));
    }
    unsafe {
        if let Some(allocator) = (&raw mut PHYS_ALLOC).as_mut().unwrap()
            && let Some(addr) = allocator.allocate()
            && let Some(addr2) = allocator.allocate()
        {
            serial_println!("0x{:x}", addr.0);
            serial_println!("0x{:x}", addr2.0);
        }
    }
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}
