#![no_std]

use mem_info::{MemInfo, MemoryRegion};
// tbh i am not understanding it well, probably will dig deeper into this
pub struct FrameAllocator {
    regions: *const MemoryRegion,
    region_count: usize,
    current_region: usize, // index into regions slice
    next_addr: u64,        // next physical addr to hand out within current region
}
impl FrameAllocator {
    pub fn new(info: *const Option<MemInfo>) -> Self {
        let info = unsafe { (*info).as_ref().unwrap() };
        let regions = info.ptr as *const MemoryRegion;
        let first_addr = unsafe { (*regions).addr };
        Self {
            regions,
            region_count: info.len,
            current_region: 0,
            next_addr: first_addr,
        }
    }
    pub fn allocate(&mut self) -> Option<u64> {
        loop {
            if self.current_region >= self.region_count {
                return None;
            }

            let region = unsafe { &*self.regions.add(self.current_region) };
            let region_end = region.addr + region.size * 4096;

            if self.next_addr < region_end {
                let addr = self.next_addr;
                self.next_addr += 4096;
                return Some(addr);
            }

            // Exhausted region
            self.current_region += 1;
            if self.current_region < self.region_count {
                let next = unsafe { &*self.regions.add(self.current_region) };
                self.next_addr = next.addr;
            }
        }
    }
}

pub static mut PHYS_ALLOC: Option<FrameAllocator> = None;
