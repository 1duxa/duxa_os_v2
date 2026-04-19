#![no_std]

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
