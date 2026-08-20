use std::ops::{Add, AddAssign};

use srmfpa::{CielArithmetic, FloorArithmetic};

use super::super::Interval;
use crate::uncertain::Numeric;

//Canonical
impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_empty() || rhs.is_empty() {
            Self::EMPTY
        } else {
            Self(self.0.floor_add(rhs.0), self.1.ciel_add(rhs.1))
        }
    }
}

// Forwards to Add for Interval (no need to reuse memory)
impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

//Canonical
impl Add<Numeric> for Interval {
    type Output = Self;

    fn add(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Adding +∞, -∞ or NaN to interval");
        if self.is_empty() {
            return Self::EMPTY;
        }
        Self(self.0.floor_add(rhs), self.1.ciel_add(rhs))
    }
}

// Forwards to Add<Numeric> for Interval (commutation)
impl Add<Interval> for Numeric {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}

// Forwards to Add<Numeric> for Interval (no need to reuse memory)
impl AddAssign<Numeric> for Interval {
    fn add_assign(&mut self, rhs: Numeric) {
        *self = *self + rhs;
    }
}
