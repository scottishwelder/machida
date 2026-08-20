use std::ops::{Div, DivAssign};

use srmfpa::{CielArithmetic, FloorArithmetic};

use super::super::{Interval, WeakSignClass};
use crate::uncertain::Numeric;

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

impl DivAssign for Interval {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Div<Numeric> for Interval {
    type Output = Self;

    fn div(self, rhs: Numeric) -> Self::Output {
        if rhs > 0.0 {
            Self(self.0.floor_div(rhs), self.1.ciel_div(rhs))
        } else if rhs < 0.0 {
            Self(self.1.floor_div(rhs), self.0.ciel_div(rhs))
        } else {
            Self::EMPTY
        }
    }
}

impl DivAssign<Numeric> for Interval {
    fn div_assign(&mut self, rhs: Numeric) {
        *self = *self / rhs;
    }
}
