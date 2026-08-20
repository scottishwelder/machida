use std::ops::{Mul, MulAssign};

use srmfpa::{CielArithmetic, FloorArithmetic};

use super::super::{Interval, WeakSignClass};
use crate::uncertain::Numeric;

// Canonical
impl Mul for Interval {
    type Output = Self;

    #[cfg(not(feature = "simpler_interval_multiplication"))]
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_empty() || rhs.is_empty() {
            return Self::EMPTY;
        }
        if self == Self::ZERO || rhs == Self::ZERO {
            return Self::ZERO;
        }

        match self.get_weak_sign() {
            WeakSignClass::NonNegative => match rhs.get_weak_sign() {
                WeakSignClass::NonNegative => Self(self.0.floor_mul(rhs.0), self.1.ciel_mul(rhs.1)),
                WeakSignClass::NonPositive => Self(self.1.floor_mul(rhs.0), self.0.ciel_mul(rhs.1)),
                WeakSignClass::StraddlesZero => {
                    // TODO: Is FIGUEIREDO, 1997 wrong here?
                    Self(self.1.floor_mul(rhs.0), self.1.ciel_mul(rhs.1))
                }
            },
            WeakSignClass::NonPositive => match rhs.get_weak_sign() {
                WeakSignClass::NonNegative => Self(self.0.floor_mul(rhs.1), self.1.ciel_mul(rhs.0)),
                WeakSignClass::NonPositive => Self(self.1.floor_mul(rhs.1), self.0.ciel_mul(rhs.0)),
                WeakSignClass::StraddlesZero => {
                    Self(self.0.floor_mul(rhs.1), self.0.ciel_mul(rhs.0))
                }
            },
            WeakSignClass::StraddlesZero => match rhs.get_weak_sign() {
                WeakSignClass::NonNegative => Self(self.0.floor_mul(rhs.1), self.1.ciel_mul(rhs.1)),
                WeakSignClass::NonPositive => Self(self.1.floor_mul(rhs.0), self.0.ciel_mul(rhs.0)),
                WeakSignClass::StraddlesZero => {
                    let lo = self.0.floor_mul(rhs.1).min(self.1.floor_mul(rhs.0));
                    let hi = self.0.ciel_mul(rhs.0).max(self.1.ciel_mul(rhs.1));
                    Self(lo, hi)
                }
            },
        }
    }

    #[cfg(feature = "simpler_interval_multiplication")]
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_empty() || rhs.is_empty() {
            return Self::EMPTY;
        }
        if self == Self::ZERO || rhs == Self::ZERO {
            return Self::ZERO;
        }

        let a_do = self.0.floor_mul(rhs.0);
        let b_do = self.0.floor_mul(rhs.1);
        let c_do = self.1.floor_mul(rhs.0);
        let d_do = self.1.floor_mul(rhs.1);

        let a_up = self.0.ciel_mul(rhs.0);
        let b_up = self.0.ciel_mul(rhs.1);
        let c_up = self.1.ciel_mul(rhs.0);
        let d_up = self.1.ciel_mul(rhs.1);

        let lo = a_do.min(b_do).min(c_do).min(d_do);
        let hi = a_up.max(b_up).max(c_up).max(d_up);
        Self(lo, hi)
    }
}

// Forwards to Mul for Interval (no need to reuse memory)
impl MulAssign for Interval {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// Canonical
impl Mul<Numeric> for Interval {
    type Output = Self;

    fn mul(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Multiplying interval by +∞, -∞ or NaN");
        if self.is_empty() {
            return Self::EMPTY;
        }
        if rhs > 0.0 {
            Self(self.0.floor_mul(rhs), self.1.ciel_mul(rhs))
        } else if rhs < 0.0 {
            Self(self.1.floor_mul(rhs), self.0.ciel_mul(rhs))
        } else {
            Self::ZERO
        }
    }
}

// Forwards to Mul<Numeric> for Interval (commutation)
impl Mul<Interval> for Numeric {
    type Output = Interval;
    fn mul(self, rhs: Interval) -> Self::Output {
        rhs * self
    }
}

// Forwards to Mul<Numeric> for Interval (no need to reuse memory)
impl MulAssign<Numeric> for Interval {
    fn mul_assign(&mut self, rhs: Numeric) {
        *self = *self * rhs;
    }
}
