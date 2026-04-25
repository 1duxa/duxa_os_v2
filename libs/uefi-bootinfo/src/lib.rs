#![no_std]

use core::fmt::Display;

use addr::{phys::PhysAddr, virt::VirtAddr};
use constants::uefi::UefiMemAttrs;

pub mod mem_type;

#[repr(C)]
pub struct BootInfo {
    pub mmap_ptr: PhysAddr,
    pub mmap_len: usize,
    pub mmap_desc_size: usize,
    pub stack_top: PhysAddr,
    pub kernel_p4_addr: PhysAddr,

    pub kernel_phys_base: PhysAddr,
    pub kernel_virt_base: VirtAddr,
    pub kernel_phys_end: PhysAddr,
    pub phys_map_base: PhysAddr,

    pub region_buf_phys: PhysAddr,
    pub region_buf_pages: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryDescriptor {
    pub mem_type: mem_type::MemoryType,
    _pad: u32,
    pub phys_start: PhysAddr,
    /// Starting virtual address.
    pub virt_start: VirtAddr,
    /// Number of 4 KiB pages contained in this range.
    pub page_count: u64,
    /// The capability attributes of this memory range.
    pub att: UefiMemAttrs,
}
impl Display for MemoryDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{:<24}] phys={:#012x}  pages={:<6}  ({} KiB)",
            self.mem_type,
            self.phys_start.raw(),
            self.page_count,
            self.page_count * 4,
        )
    }
}
