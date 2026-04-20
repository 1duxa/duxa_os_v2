#![no_std]

use core::fmt::Display;

pub mod mem_type;

#[repr(C)]
pub struct BootInfo {
    pub mmap_ptr: u64,
    pub mmap_len: usize,
    pub mmap_desc_size: usize,
    pub stack_top: u64,
    pub kernel_p4_addr: u64,

    pub kernel_phys_base: u64,
    pub kernel_virt_base: u64,
    pub kernel_phys_end: u64,
    pub phys_map_base: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryDescriptor {
    pub mem_type: mem_type::MemoryType,
    _pad: u32,
    pub phys_start: u64,
    /// Starting virtual address.
    pub virt_start: u64,
    /// Number of 4 KiB pages contained in this range.
    pub page_count: u64,
    /// The capability attributes of this memory range.
    pub att: u64,
}
impl Display for MemoryDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{:<24}] phys={:#012x}  pages={:<6}  ({} KiB)",
            self.mem_type,
            self.phys_start,
            self.page_count,
            self.page_count * 4,
        )
    }
}
