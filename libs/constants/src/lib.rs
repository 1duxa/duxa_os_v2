#![no_std]
#[allow(non_upper_case_globals)]
pub mod size {
    pub const KiB: u64 = 1024;
    pub const MiB: u64 = 1024 * KiB;
    pub const GiB: u64 = 1024 * MiB;
}
pub mod page_flags {
    pub const PRESENT: u64 = 1;
    pub const WRITABLE: u64 = 1 << 1;
    pub const HUGE: u64 = 1 << 7;
}
