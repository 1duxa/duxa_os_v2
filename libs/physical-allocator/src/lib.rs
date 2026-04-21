#![no_std]

use mem_info::MemInfo;

pub struct PhysicalAllocator {
    pub len: usize,
}
impl PhysicalAllocator {
    pub fn new(info: &MemInfo) -> Self {
        todo!();
    }
}
