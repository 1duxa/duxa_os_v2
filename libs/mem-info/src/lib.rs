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
        phys_write_to_pages: usize,
        phys_map: PhysAddr,
    ) -> Self {
        let max_regions = phys_write_to_pages * 4096 / core::mem::size_of::<MemoryRegion>();
        let mem_regions = unsafe {
            core::slice::from_raw_parts_mut(phys_write_to.raw() as *mut MemoryRegion, max_regions)
        };

        let mut ptr = mmap_ptr.raw() as *const MemoryDescriptor;
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
            ptr: phys_to_virt(PhysAddr::new(mem_regions.as_ptr() as u64), phys_map),
            len: count,
        }
    }
}

pub struct MemoryRegion {
    pub addr: PhysAddr,
    pub attr: UefiMemAttrs,
    pub page_count: u64,
}
impl MemoryRegion {
    pub fn new(desc: MemoryDescriptor) -> Self {
        Self {
            addr: desc.phys_start,
            attr: desc.att,
            page_count: desc.page_count,
        }
    }
}
