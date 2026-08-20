use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use srmfpa::{CielArithmetic, CielMath, FloorArithmetic, FloorMath};

use super::{Interval, WeakSignClass};
use crate::uncertain::ExtraOps;

impl ExtraOps for Interval {
    type SqrtResult = Self;
    type ReciprocalResult = Self;

    fn sqrt(self) -> Self::SqrtResult {
        if self.is_empty() || self.1 < 0.0 {
            return Self::EMPTY;
        }
        if self.0 <= 0.0 {
            return Self(0.0, self.1.ciel_sqrt());
        }
        Self(self.0.floor_sqrt(), self.1.ciel_sqrt())
    }

    fn reciprocal(self) -> Self::ReciprocalResult {
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

impl Add<Numeric> for Interval {
    type Output = Self;

    /// # Panics
    /// Panics if the scalar is [+∞], [-∞] or [`NaN`]
    ///
    /// [`NaN`]: Numeric::NAN
    /// [+∞]: Numeric::INFINITY
    /// [-∞]: Numeric::NEG_INFINITY
    fn add(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "The scalar cannot be infinite or NaN");
        Self(self.0.floor_add(rhs), self.1.ciel_add(rhs))
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<Numeric> for Interval {
    fn add_assign(&mut self, rhs: Numeric) {
        *self = *self + rhs;
    }
}

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

impl Sub<Numeric> for Interval {
    type Output = Self;

    /// # Panics
    /// Panics if the scalar is [+∞], [-∞] or [`NaN`]
    ///
    /// [`NaN`]: Numeric::NAN
    /// [+∞]: Numeric::INFINITY
    /// [-∞]: Numeric::NEG_INFINITY
    fn sub(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "The scalar cannot be infinite or NaN");
        Self(self.0.floor_sub(rhs), self.1.ciel_sub(rhs))
    }
}

impl SubAssign for Interval {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<Numeric> for Interval {
    fn sub_assign(&mut self, rhs: Numeric) {
        *self = *self - rhs;
    }
}

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

impl Mul<Numeric> for Interval {
    type Output = Self;

    fn mul(self, rhs: Numeric) -> Self::Output {
        // TODO: Is this necessary?
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

impl MulAssign for Interval {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<Numeric> for Interval {
    fn mul_assign(&mut self, rhs: Numeric) {
        *self = *self * rhs;
    }
}

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

impl DivAssign for Interval {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<Numeric> for Interval {
    fn div_assign(&mut self, rhs: Numeric) {
        *self = *self / rhs;
    }
}

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.1, -self.0)
    }
}
