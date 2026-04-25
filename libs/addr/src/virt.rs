use core::ops::{Add, AddAssign, BitAnd, Shr, Sub};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VirtAddr(u64);

impl VirtAddr {
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

    pub fn as_virt_ptr(&self) -> *const VirtAddr {
        self as *const VirtAddr
    }

    pub fn as_virt_ptr_mut(&mut self) -> *mut VirtAddr {
        self as *mut VirtAddr
    }
}

impl Add<u64> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        VirtAddr(self.0.wrapping_add(rhs))
    }
}

impl AddAssign<u64> for VirtAddr {
    fn add_assign(&mut self, rhs: u64) {
        self.0 = self.0.wrapping_add(rhs);
    }
}

impl Sub for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}
impl Sub<u64> for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self::new(self.0 - rhs)
    }
}

impl BitAnd<u64> for VirtAddr {
    type Output = u64;

    fn bitand(self, rhs: u64) -> Self::Output {
        self.0 & rhs
    }
}

impl Shr<u64> for VirtAddr {
    type Output = u64;

    fn shr(self, rhs: u64) -> Self::Output {
        self.0 >> rhs
    }
}
