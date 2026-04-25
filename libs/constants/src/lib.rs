#![no_std]

use bitflags::bitflags;

pub mod uefi;

#[allow(non_upper_case_globals)]
pub mod size {
    pub const KiB: u64 = 1024;
    pub const MiB: u64 = 1024 * KiB;
    pub const GiB: u64 = 1024 * MiB;
}
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PageFlags: u64 {
        const PRESENT = 1;
        const WRITABLE = 1 << 1;
        const HUGE = 1 << 7;
    }
}
