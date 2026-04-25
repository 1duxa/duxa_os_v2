use core::ops::{Add, AddAssign};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PhysAddr(pub u64);

impl Add<u64> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        PhysAddr(self.0.wrapping_add(rhs))
    }
}

impl AddAssign<u64> for PhysAddr {
    fn add_assign(&mut self, rhs: u64) {
        self.0 = self.0.wrapping_add(rhs);
    }
}
