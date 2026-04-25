#![no_std]

use addr::phys::PhysAddr;
use mem_info::{MemInfo, MemoryRegion};

// tbh i am not understanding it well, probably will dig deeper into this
pub struct FrameAllocator {
    regions: *const MemoryRegion,
    region_count: usize,
    current_region: usize, // index into regions slice
    next_addr: PhysAddr,   // next physical addr to hand out within current region
}
impl FrameAllocator {
    /**
        # Safety
        It's inherently unsafe
    */
    pub unsafe fn new(info: *const Option<MemInfo>) -> Self {
        let info = unsafe { (*info).as_ref().unwrap() };
        let regions = info.ptr.raw() as *const MemoryRegion;
        let first_addr = if info.len > 0 {
            unsafe { (*regions).addr }
        } else {
            PhysAddr::new(0)
        };
        Self {
            regions,
            region_count: info.len,
            current_region: 0,
            next_addr: first_addr,
        }
    }
    pub fn allocate(&mut self) -> Option<PhysAddr> {
        loop {
            if self.current_region >= self.region_count {
                return None;
            }

            let region = unsafe { &*self.regions.add(self.current_region) };
            let region_end = region.addr + region.page_count * 4096;

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
