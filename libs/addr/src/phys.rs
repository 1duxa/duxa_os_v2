use core::ops::{Add, AddAssign, BitAnd, Shr, Sub};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PhysAddr(u64);

impl PhysAddr {
    pub fn new(addr: u64) -> Self {
        Self(addr)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }

    pub fn set(&mut self, val: u64) {
        self.0 = val;
    }

    pub fn as_ptr(&self) -> *const u64 {
        self.0 as *const u64
    }

    pub fn as_ptr_mut(&mut self) -> *mut u64 {
        self.0 as *mut u64
    }

    pub fn as_phys_ptr(&self) -> *const PhysAddr {
        self as *const PhysAddr
    }

    pub fn as_phys_ptr_mut(&mut self) -> *mut PhysAddr {
        self as *mut PhysAddr
    }
}

impl Add<u64> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        PhysAddr(self.0.wrapping_add(rhs))
    }
}

impl Add<PhysAddr> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: PhysAddr) -> Self::Output {
        PhysAddr(self.0.wrapping_add(rhs.0))
    }
}

impl AddAssign<u64> for PhysAddr {
    fn add_assign(&mut self, rhs: u64) {
        self.0 = self.0.wrapping_add(rhs);
    }
}

impl Sub for PhysAddr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        PhysAddr::new(self.0 - rhs.0)
    }
}
impl Sub<u64> for PhysAddr {
    type Output = PhysAddr;

    fn sub(self, rhs: u64) -> Self::Output {
        PhysAddr::new(self.0 - rhs)
    }
}

impl BitAnd<u64> for PhysAddr {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        self.0 & rhs
    }
}

impl Shr<u64> for PhysAddr {
    type Output = u64;

    fn shr(self, rhs: u64) -> Self::Output {
        self.0 >> rhs
    }
}
