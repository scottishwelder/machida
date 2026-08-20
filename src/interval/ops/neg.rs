use std::ops::Neg;

use super::super::Interval;

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.1, -self.0)
    }
}
