#![no_std]

// Should have been a beautiful abstration but i wanted to get it going
// then figure out what i can hide away
#[repr(align(4096))]
pub struct PageTable(pub [u64; 512]);

pub fn phys_to_virt(phys: u64, phys_map: u64) -> u64 {
    phys_map + phys
}

pub fn virt_to_phys(virt: u64, phys_map: u64) -> u64 {
    virt - phys_map
}

pub fn entry_addr(entry: u64) -> u64 {
    entry & !0xFFF
}

pub fn entry_present(entry: u64) -> bool {
    entry & 1 != 0
}
