#![no_std]

use addr::{phys::PhysAddr, virt::VirtAddr};

// Should have been a beautiful abstration but i wanted to get it going
// then figure out what i can hide away
#[repr(align(4096))]
pub struct PageTable(pub [u64; 512]);
/**
    # Safety
    Inherently unsafe
*/
pub unsafe fn walky_talky(virt_addr: VirtAddr, phys_map: u64, p4: *mut PageTable) -> Option<u64> {
    let p4_idx = virt_addr_unmask(virt_addr >> 39) as usize;
    let p3_idx = virt_addr_unmask(virt_addr >> 30) as usize;
    let p2_idx = virt_addr_unmask(virt_addr >> 21) as usize;
    let p1_idx = virt_addr_unmask(virt_addr >> 12) as usize;
    let offset = virt_addr & 0xFFF;

    let p4e = unsafe { (*p4).0[p4_idx] };
    if !entry_present(p4e) {
        return None;
    }

    let p3 = phys_to_virt(PhysAddr(entry_addr(p4e)), phys_map).0 as *mut PageTable;

    let p3e = unsafe { (*p3).0[p3_idx] };
    if !entry_present(p3e) {
        return None;
    }

    // TODO:
    //
    // if p2e & (1 << 7) != 0 {
    // 2 MiB page
    // }
    //
    // and:
    //
    // if p3e & (1 << 7) != 0 {
    //     // 1 GiB page
    // }
    //

    let p2 = phys_to_virt(PhysAddr(entry_addr(p3e)), phys_map).0 as *mut PageTable;

    let p2e = unsafe { (*p2).0[p2_idx] };
    if !entry_present(p2e) {
        return None;
    }

    let p1 = phys_to_virt(PhysAddr(entry_addr(p2e)), phys_map).0 as *mut PageTable;

    let p1e = unsafe { (*p1).0[p1_idx] };
    if !entry_present(p1e) {
        return None;
    }

    let phys_page = entry_addr(p1e);
    Some(phys_page + offset)
}
pub fn phys_to_virt(phys: PhysAddr, phys_map: u64) -> VirtAddr {
    VirtAddr(phys.0 + phys_map)
}

pub fn virt_to_phys(virt: VirtAddr, phys_map: u64) -> PhysAddr {
    PhysAddr(virt.0 - phys_map)
}
pub fn entry_addr(entry: u64) -> u64 {
    entry & !0xFFF
}
pub fn entry(entry: u64) -> u64 {
    entry & 0xFFF
}
#[inline]
pub fn virt_addr_unmask(entry: u64) -> u64 {
    entry & 0x1FF
}
pub fn entry_present(entry: u64) -> bool {
    entry & 1 != 0
}
