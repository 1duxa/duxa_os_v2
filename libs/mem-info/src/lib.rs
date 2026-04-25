#![no_std]

use addr::{phys::PhysAddr, virt::VirtAddr};
use constants::uefi::UefiMemAttrs;
use page_table::phys_to_virt;
use uefi_bootinfo::MemoryDescriptor;

pub static mut MEM_INFO: Option<MemInfo> = None;

pub struct MemInfo {
    pub ptr: VirtAddr,
    pub len: usize,
}
impl MemInfo {
    pub fn init(
        mmap_ptr: PhysAddr,
        mmap_len: usize,
        mmap_desc_size: usize,
        phys_write_to: PhysAddr,
        usable: usize,
        phys_map: u64,
    ) -> Self {
        let mem_regions = unsafe {
            core::slice::from_raw_parts_mut(phys_write_to.0 as *mut MemoryRegion, usable)
        };

        let mut ptr = mmap_ptr.0 as *const MemoryDescriptor;
        let mut count = 0usize;
        for _ in 0..mmap_len {
            let desc = unsafe { &*ptr };
            if desc.mem_type.is_conventional_memory() {
                mem_regions[count] = MemoryRegion::new(*desc);
                count += 1;
            }
            ptr = unsafe { (ptr as *const u8).add(mmap_desc_size) as *const MemoryDescriptor };
        }
        Self {
            ptr: phys_to_virt(PhysAddr(mem_regions.as_ptr() as u64), phys_map),
            len: count,
        }
    }
}

pub struct MemoryRegion {
    pub addr: PhysAddr,
    pub attr: UefiMemAttrs,
    pub size: u64,
}
impl MemoryRegion {
    pub fn new(desc: MemoryDescriptor) -> Self {
        Self {
            addr: desc.phys_start,
            attr: desc.att,
            size: desc.page_count,
        }
    }
}
