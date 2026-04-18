#![no_std]

#[repr(C)]
pub struct BootInfo {
    pub mmap_ptr: u64,
    pub mmap_len: usize,
    pub mmap_desc_size: usize,
    pub stack_top: u64,
    pub kernel_high_p4_addr: u64,
}
