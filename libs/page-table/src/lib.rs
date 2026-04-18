#![no_std]
 
#[repr(align(4096))]
pub struct PageTable(pub [u64;512]);
