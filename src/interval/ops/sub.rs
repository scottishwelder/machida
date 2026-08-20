use std::ops::{Sub, SubAssign};

use srmfpa::{CielArithmetic, FloorArithmetic};

use super::super::Interval;
use crate::uncertain::Numeric;

impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_empty() || rhs.is_empty() {
            Self::EMPTY
        } else {
            Self(self.0.floor_sub(rhs.1), self.1.ciel_sub(rhs.0))
        }
    }
}

impl SubAssign for Interval {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sub<Numeric> for Interval {
    type Output = Self;

    /// # Panics
    /// Panics if the scalar is [+∞], [-∞] or [`NaN`]
    ///
    /// [`NaN`]: Numeric::NAN
    /// [+∞]: Numeric::INFINITY
    /// [-∞]: Numeric::NEG_INFINITY
    fn sub(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Subtracting +∞, -∞ or NaN from interval");
        if self.is_empty() {
            return Self::EMPTY;
        }
        Self(self.0.floor_sub(rhs), self.1.ciel_sub(rhs))
    }
}

impl Sub<Interval> for Numeric {
    type Output = Interval;
    fn sub(self, rhs: Interval) -> Self::Output {
        assert!(self.is_finite(), "Subtracting interval from +∞, -∞ or NaN");
        if rhs.is_empty() {
            return Interval::EMPTY;
        }
        Interval(self.floor_sub(rhs.0), self.ciel_sub(rhs.1))
    }
}

impl SubAssign<Numeric> for Interval {
    fn sub_assign(&mut self, rhs: Numeric) {
        *self = *self - rhs;
    }
}
