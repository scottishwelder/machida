use srmfpa::{CielMath, FloorArithmetic, FloorMath};

use super::super::{Interval, WeakSignClass};
use crate::uncertain::{ExtraOps, Numeric};

impl ExtraOps for Interval {
    type SqrtResult = Self;
    type RecipResult = Self;

    fn sqrt(self) -> Self::SqrtResult {
        if self.is_empty() || self.1 < 0.0 {
            return Self::EMPTY;
        }
        if self.0 <= 0.0 {
            return Self(0.0, self.1.ciel_sqrt());
        }
        Self(self.0.floor_sqrt(), self.1.ciel_sqrt())
    }

    fn recip(self) -> Self::RecipResult {
        if self.is_empty() || self == Self::ZERO {
            return Self::EMPTY;
        }
        if self.get_weak_sign() == WeakSignClass::StraddlesZero {
            return Self::R;
        }
        let lo = if self.1 == 0.0 {
            Numeric::NEG_INFINITY
        } else {
            1.0.floor_div(self.1)
        };
        let hi = if self.0 == 0.0 {
            Numeric::INFINITY
        } else {
            1.0.floor_div(self.0)
        };
        Self(lo, hi)
    }
}
