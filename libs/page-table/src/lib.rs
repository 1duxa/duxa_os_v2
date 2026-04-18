#![no_std]

// Should have been a beautiful abstration but i wanted to get it going
// then figure out what i can hide away
#[repr(align(4096))]
pub struct PageTable(pub [u64; 512]);
