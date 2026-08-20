// TODO: Check bounds at zero

use std::ops::{Div, DivAssign};

use srmfpa::{CielArithmetic, FloorArithmetic};

use super::super::{Interval, WeakSignClass};
use crate::uncertain::Numeric;

// Canonical
impl Div for Interval {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.is_empty() || rhs.is_empty() || rhs == Self::ZERO {
            return Self::EMPTY;
        }
        if self == Self::ZERO {
            return Self::ZERO;
        }
        match rhs.get_weak_sign() {
            WeakSignClass::NonNegative => match self.get_weak_sign() {
                WeakSignClass::NonNegative => Self(self.0.floor_div(rhs.1), self.1.ciel_div(rhs.0)),
                WeakSignClass::NonPositive => Self(self.0.floor_div(rhs.0), self.1.ciel_div(rhs.1)),
                WeakSignClass::StraddlesZero => {
                    Self(self.0.floor_div(rhs.0), self.1.ciel_div(rhs.0))
                }
            },
            WeakSignClass::NonPositive => match self.get_weak_sign() {
                WeakSignClass::NonNegative => Self(self.1.floor_div(rhs.1), self.0.ciel_div(rhs.0)),
                WeakSignClass::NonPositive => Self(self.1.floor_div(rhs.0), self.0.ciel_div(rhs.1)),
                WeakSignClass::StraddlesZero => {
                    Self(self.1.floor_div(rhs.1), self.0.ciel_div(rhs.1))
                }
            },
            WeakSignClass::StraddlesZero => Self::R,
        }
    }
}

// Forwards to Div for Interval (no need to reuse memory)
impl DivAssign for Interval {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

// Canonical
impl Div<Numeric> for Interval {
    type Output = Self;

    fn div(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Dividing interval by +∞, -∞ or NaN");
        if self.is_empty() {
            return Self::EMPTY;
        }
        if rhs > 0.0 {
            Self(self.0.floor_div(rhs), self.1.ciel_div(rhs))
        } else if rhs < 0.0 {
            Self(self.1.floor_div(rhs), self.0.ciel_div(rhs))
        } else {
            Self::EMPTY
        }
    }
}

// Canonical.
// Cannot forward to Div<Numeric> for Interval because division is not commutative.
impl Div<Interval> for Numeric {
    type Output = Interval;
    fn div(self, rhs: Interval) -> Self::Output {
        assert!(self.is_finite(), "Dividing +∞, -∞ or NaN by interval");
        if rhs.is_empty() {
            return Interval::EMPTY;
        }
        if self == 0.0 {
            return if rhs == Interval::ZERO { Interval::EMPTY } else { Interval::ZERO };
        }
        match rhs.get_weak_sign() {
            WeakSignClass::StraddlesZero => Interval::R,
            WeakSignClass::NonNegative | WeakSignClass::NonPositive => {
                if self > 0.0 {
                    Interval(self.floor_div(rhs.1), self.floor_div(rhs.0))
                } else {
                    Interval(self.floor_div(rhs.0), self.floor_div(rhs.1))
                }
            }
        }
    }
}

// Forwards to Div<Numeric> for Interval (no need to reuse memory)
impl DivAssign<Numeric> for Interval {
    fn div_assign(&mut self, rhs: Numeric) {
        *self = *self / rhs;
    }
}
