#![no_std]

#[repr(C)]
pub struct BootInfo {
    pub mmap_ptr: u64,
    pub mmap_len: usize,
    pub mmap_desc_size: usize,
}
